//! Инфраструктура сервера для обработки gRPC.

use crate::application::AppServices;
use proto_crate::proto_blog::{
    blog_service_server::BlogService, AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse,
    GetPostRequest, ListPostsRequest, ListPostsResponse, LoginRequest, PostResponse,
    RegisterRequest, UpdatePostRequest,
};
use tonic::{Request, Response, Status};

/// gRPC-сервис блога, использующий методы взаимодействия с базой данных.
pub(crate) struct BlogGrpcService {
    /// [`AppServices`] для взаимодействия с методами обработки данных.
    services: AppServices,
}

impl BlogGrpcService {
    /// Создать экземпляр [`BlogGrpcService`] с привязкой к пулу соединений
    /// к базе данных.
    pub(crate) fn new(app_services: AppServices) -> Self {
        Self {
            services: app_services,
        }
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        todo!()
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        todo!()
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        todo!()
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        todo!()
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        todo!()
    }
}
