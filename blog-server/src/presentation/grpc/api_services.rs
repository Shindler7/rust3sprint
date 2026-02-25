//! Инфраструктура сервера для обработки gRPC.

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService, AppServices},
    data::{post_repo::PostRepo, user_repo::UserRepo},
    domain::{
        post::{CreatePost, EditPostCommand},
        types::DataId,
        user::{AuthResponse as UserAuthResponse, CreateUser, LoginUser, UserDto},
    },
    infrastructure::jwt::JwtService,
    presentation::{
        grpc::tools::get_auth_user,
        tools::{get_jwt_token, validate_list_params, verified_user_password},
    },
};
use proto_crate::proto_blog::{
    blog_service_server::BlogService as TraitBlogService, AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse,
    GetPostRequest, ListPostsRequest, ListPostsResponse, LoginRequest, Post as ProtoPost,
    PostResponse, RegisterRequest, UpdatePostRequest,
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

        let user_dto: UserDto = user.into();
        let token = get_jwt_token(&user_dto, &self.jwt_service)?;
        let auth_response: AuthResponse = UserAuthResponse::new(token, user_dto).try_into()?;

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

        let user_dto: UserDto = user.into();
        let token = get_jwt_token(&user_dto, &self.jwt_service)?;
        let auth_response: AuthResponse = UserAuthResponse::new(token, user_dto).try_into()?;

        Ok(Response::new(auth_response))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let auth_user = get_auth_user(&request.metadata(), self.jwt_service.clone())?;
        let create_post = CreatePost::try_from(request.into_inner())?;

        let post = self
            .post_service
            .create_post(&create_post, &auth_user.id)
            .await
            .inspect_err(|err| {
                error!(
                    error = %err,
                    author_id = %auth_user.id,
                    "Ошибка создания новой записи в блоге"
                )
            })?;

        let post_grpc: ProtoPost = post.try_into()?;

        Ok(Response::new(PostResponse {
            post: Some(post_grpc),
        }))
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let post_id: DataId = request.into_inner().id.into();
        let post = self
            .post_service
            .get_post(&post_id)
            .await
            .inspect_err(|err| {
                error!(
                    error = %err,
                    post_id = %post_id,
                    "Публикация не найдена"
                )
            })?;

        let post_grpc: ProtoPost = post.try_into()?;

        Ok(Response::new(PostResponse {
            post: Some(post_grpc),
        }))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let auth_user = get_auth_user(&request.metadata(), self.jwt_service.clone())?;
        let edit_command = EditPostCommand::try_from(request.into_inner())?;

        let post = self
            .post_service
            .update_post(&edit_command, &auth_user.id)
            .await
            .inspect_err(|err| {
                error!(
                    error=%err,
                    post_id=%edit_command.post_id,
                    user_id=%auth_user.id,
                    "Обновить публикацию (пост) не удалось"
                )
            })?;

        let post_grpc: ProtoPost = post.try_into()?;

        Ok(Response::new(PostResponse {
            post: Some(post_grpc),
        }))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let auth_user = get_auth_user(&request.metadata(), self.jwt_service.clone())?;
        let user_id: DataId = auth_user.id.into();
        let post_id: DataId = request.into_inner().id.into();

        self.post_service
            .delete_post(&post_id, &user_id)
            .await
            .inspect_err(|err| {
                error!(
                    error = %err,
                    post_id=%post_id,
                    user_id=%user_id,
                    "Ошибка удаления публикации"
                )
            })?;

        Ok(Response::new(DeletePostResponse { success: true }))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let list_posts = request.into_inner();
        validate_list_params(list_posts.limit, list_posts.offset)?;

        let posts = self
            .post_service
            .list_posts(list_posts.limit, list_posts.offset)
            .await?;

        let grpc_posts: Vec<ProtoPost> = posts
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<_, _>>()?;

        Ok(Response::new(ListPostsResponse { posts: grpc_posts }))
    }
}
