//! Инфраструктура сервера для обработки gRPC.

use proto_crate::proto_blog::{
    blog_service_server::BlogService, AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse,
    GetPostRequest, ListPostsRequest, ListPostsResponse, LoginRequest, PostResponse,
    RegisterRequest, UpdatePostRequest,
};
use sqlx::{Database, Pool};
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// gRPC-сервис блога, использующий пул соединений с базой данных.
///
/// Хранит `Arc<Pool<DB>>`, что позволяет безопасно клонировать сервис
/// и переиспользовать соединения между запросами.
pub(crate) struct BlogGrpcService<DB: Database> {
    pool: Arc<Pool<DB>>,
}

impl<DB: Database> Clone for BlogGrpcService<DB> {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
        }
    }
}

impl<DB: Database> BlogGrpcService<DB> {
    /// Создать экземпляр [`BlogGrpcService`] с привязкой к пулу соединений
    /// к базе данных.
    pub(crate) fn new(pool: Pool<DB>) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    /// Доступ к пулу соединений.
    pub(crate) fn pool(&self) -> Arc<Pool<DB>> {
        Arc::clone(&self.pool)
    }
}

#[tonic::async_trait]
impl<DB: Database> BlogService for BlogGrpcService<DB> {
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
