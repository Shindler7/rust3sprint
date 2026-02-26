//! Транспорт для взаимодействия с HTTP-сервером.

use crate::BlogClientError;
use crate::{
    clients::{
        models::{PostCreateCmd, PostId, PostUpdateCmd, UserAuthCmd, UserRegisterCmd},
        traits::ClientTransportExt,
    },
    models::{AuthResponse, Token},
};
use proto_crate::proto_blog::Post;
use reqwest::Url;
use tonic::async_trait;

/// Эндпоинт для регистрации пользователя.
const API_AUTH_REGISTER: &str = "api/auth/register";
/// Эндпоинт для авторизации пользователя.
const API_AUTH_LOGIN: &str = "api/auth/login";
/// Эндпоинт для взаимодействия с публикациями.
const API_POSTS: &str = "api/posts";


/// Клиент для взаимодействия с HTTP-сервером.
#[derive(Debug)]
pub(crate) struct HttpClient {
    /// Ссылка для доступа к API сервера.
    server_url: Url,
}

impl HttpClient {
    pub(crate) fn new(server_url: Url) -> Self {
        Self { server_url }
    }
}

#[async_trait]
impl ClientTransportExt for HttpClient {
    type Error = BlogClientError;

    async fn register_user(&self, cmd: UserRegisterCmd) -> Result<AuthResponse, Self::Error> {
        let client = reqwest::Client::new();
        let url = self.server_url.join(API_AUTH_REGISTER);


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
