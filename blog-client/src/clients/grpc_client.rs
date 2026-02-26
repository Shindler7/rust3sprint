//! Транспорт для взаимодействия с gRPC-сервером.

use crate::clients::models::{PostCreateCmd, PostId, PostUpdateCmd, UserAuthCmd, UserRegisterCmd};
use crate::clients::traits::ClientTransportExt;
use crate::models::{AuthResponse, Token};
use proto_crate::proto_blog::Post;
use reqwest::Url;
use tonic::async_trait;
use crate::BlogClientError;

/// Клиент для взаимодействия с gRPC-сервером.
#[derive(Debug)]
pub(crate) struct GrpcClient {
    /// Ссылка для доступа к API сервера.
    server_url: Url,
}

impl GrpcClient {
    pub(crate) fn new(server_url: Url) -> Self {
        Self { server_url }
    }
}

#[async_trait]
impl ClientTransportExt for GrpcClient {
    type Error = BlogClientError;

    async fn register_user(&self, cmd: UserRegisterCmd) -> Result<AuthResponse, Self::Error> {
        todo!()
    }

    async fn login_user(&self, cmd: UserAuthCmd) -> Result<AuthResponse, Self::Error> {
        todo!()
    }

    async fn create_post(&self, cmd: PostCreateCmd, token: &Token) -> Result<Post, Self::Error> {
        todo!()
    }

    async fn get_post(&self, post_id: PostId) -> Result<Post, Self::Error> {
        todo!()
    }

    async fn update_post(&self, cmd: PostUpdateCmd, token: &Token) -> Result<Post, Self::Error> {
        todo!()
    }

    async fn delete_post(&self, post_id: PostId, token: &Token) -> Result<(), Self::Error> {
        todo!()
    }

    async fn list_posts(&self) -> Result<Vec<Post>, Self::Error> {
        todo!()
    }
}
