//! Инфраструктура сервера для обработки gRPC.

use crate::infrastructure::jwt::JwtService;
use crate::{
    application::{auth_service::AuthService, blog_service::BlogService, AppServices},
    data::{post_repo::PostRepo, user_repo::UserRepo},
    domain::user::{CreateUser, LoginUser},
    presentation::tools::{get_jwt_token, verified_user_password},
};
use proto_crate::proto_blog::blog_service_server::BlogService as TraitBlogService;
use proto_crate::proto_blog::{
    AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
    ListPostsRequest, ListPostsResponse, LoginRequest, PostResponse, RegisterRequest,
    UpdatePostRequest, User as ProtoUser,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::error;

/// gRPC-сервис блога, использующий методы взаимодействия с базой данных.
pub(crate) struct BlogGrpcService {
    /// Серверный сервис аутентификации.
    auth_service: Arc<AuthService<UserRepo>>,
    /// Серверный сервис обработки данных блога.
    post_service: Arc<BlogService<PostRepo>>,
    /// Сервис обработки JWT-токенов приложения.
    jwt_service: Arc<JwtService>,
}

impl BlogGrpcService {
    /// Создать экземпляр [`BlogGrpcService`] с привязкой к пулу соединений
    /// к базе данных.
    pub(crate) fn new(app_services: AppServices, jwt_service: Arc<JwtService>) -> Self {
        Self {
            auth_service: Arc::clone(&app_services.auth_service),
            post_service: Arc::clone(&app_services.blog_service),
            jwt_service,
        }
    }
}

#[tonic::async_trait]
impl TraitBlogService for BlogGrpcService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let create_user = CreateUser::try_from(request.into_inner())?;

        let user = self
            .auth_service
            .create_user(&create_user)
            .await
            .inspect_err(|err| {
                error!(
            error = %err,
            username = %create_user.username,
            email = %create_user.email,
            "Ошибка регистрации")
            })?;

        let token = get_jwt_token(&user, &self.jwt_service)?;
        let proto_user: ProtoUser = user.try_into()?;

        let auth_response = AuthResponse {
            token,
            user: Some(proto_user),
        };

        Ok(Response::new(auth_response))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let login_user = LoginUser::try_from(request.into_inner())?;
        
        let user = self
            .auth_service
            .get_user(&login_user.username)
            .await
        .inspect_err(|err| {
            error!(
                error = %err,
                username = %login_user.username,
                "Ошибка авторизации"
            )
        })?;
        
        verified_user_password(&login_user, &user.password_hash)?;
        let token = get_jwt_token(&user, &self.jwt_service)?;
        
        let proto_user: ProtoUser = user.try_into()?;
        let auth_response = AuthResponse {
            token,
            user: Some(proto_user),
        };

        Ok(Response::new(auth_response))
    }

    async fn create_post(
        &self,
        _request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn get_post(
        &self,
        _request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn update_post(
        &self,
        _request: Request<UpdatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn delete_post(
        &self,
        _request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        todo!()
    }

    async fn list_posts(
        &self,
        _request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        todo!()
    }
}
