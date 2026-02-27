//! Транспорт для взаимодействия с HTTP-сервером.

use crate::{
    BlogClientError,
    clients::{
        models::{
            PostCreateCmd, PostId, PostUpdateCmd, PostUpdateCmdHttp, UserAuthCmd, UserRegisterCmd,
        },
        tools::compile_url,
        traits::ClientTransportExt,
    },
    models::{AuthResponse, Token},
};
use proto_crate::proto_blog::{ListPostsResponse, Post};
use reqwest::{Client, Method, Response, Url};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tonic::async_trait;

/// Эндпоинт для регистрации пользователя.
const API_AUTH_REGISTER: &str = "api/auth/register";
/// Эндпоинт для авторизации пользователя.
const API_AUTH_LOGIN: &str = "api/auth/login";
/// Эндпоинт для взаимодействия с публикациями.
const API_POSTS: &str = "api/posts";

/// Настройки для [`Client`].
struct ClientSettings {
    /// Временной лимит для фазы подключения (секунды).
    connect_timeout: Duration,
    /// Общий временной лимит запроса (секунды).
    timeout: Duration,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            timeout: Duration::from_secs(20),
        }
    }
}

/// Клиент для взаимодействия с HTTP-сервером.
#[derive(Debug)]
pub(crate) struct HttpClient {
    /// Ссылка для доступа к API сервера.
    server_url: Url,
    client: Arc<Client>,
}

impl HttpClient {
    /// Создать HTTP-клиента.
    ///
    /// ## Args
    ///
    /// - `server_url` — обязательный аргумент, ссылка на HTTP-сервер
    pub(crate) fn new(server_url: Url) -> Result<Self, BlogClientError> {
        let client_settings = ClientSettings::default();

        let client = Client::builder()
            .connect_timeout(client_settings.connect_timeout)
            .timeout(client_settings.timeout)
            .build()?;

        Ok(Self {
            server_url,
            client: Arc::new(client),
        })
    }

    /// Исполнитель HTTP-запросов.
    ///
    /// ## Args
    ///
    /// - `method` — метод HTTP-запроса, предоставляемый с помощью перечисления
    ///   [`Method`]
    /// - `url` — ссылка для запроса в обёртке [`Url`]
    /// - `body` — опционально, сериализуемая в JSON структура с данными для
    ///   тела запроса
    /// - `token` — опционально, Bearer-токен для запроса (добавляется в
    ///   заголовок)
    ///
    /// ## Ошибки
    ///
    /// Автоматически возвращает сетевые ошибки, а также трансформирует
    /// в ошибки приложения неудачные запросы к серверу.
    async fn send_request<T: Serialize>(
        &self,
        method: Method,
        url: Url,
        body: Option<&T>,
        token: Option<&Token>,
    ) -> Result<Response, BlogClientError> {
        let client = self.client.clone();

        let mut req_builder = client.request(method, url);
        if let Some(token) = token {
            req_builder = req_builder.bearer_auth(token);
        }
        if let Some(body) = body {
            req_builder = req_builder.json(&body);
        }

        let res = req_builder.send().await?;

        // if res.status().is_client_error() {
        //     return match res.status() {
        //         StatusCode::UNAUTHORIZED => Err(BlogClientError::Unauthorized),
        //         StatusCode::NOT_FOUND => Err(BlogClientError::NotFound),
        //         _ => Err(BlogClientError::InvalidRequest(res.status().to_string())),
        //     };
        // }

        res.error_for_status_ref()?;
        Ok(res)
    }
}

#[async_trait]
impl ClientTransportExt for HttpClient {
    type Error = BlogClientError;

    async fn register_user(&self, cmd: UserRegisterCmd) -> Result<AuthResponse, Self::Error> {
        let url = compile_url(&self.server_url, &[API_AUTH_REGISTER])?;

        let res = self
            .send_request(Method::POST, url, Some(&cmd), None)
            .await?;
        let auth_response: AuthResponse = res.json().await?;

        Ok(auth_response)
    }

    async fn login_user(&self, cmd: UserAuthCmd) -> Result<AuthResponse, Self::Error> {
        let url = compile_url(&self.server_url, &[API_AUTH_LOGIN])?;

        let res = self
            .send_request(Method::POST, url, Some(&cmd), None)
            .await?;
        let auth_response: AuthResponse = res.json().await?;

        Ok(auth_response)
    }

    async fn create_post(&self, cmd: PostCreateCmd, token: &Token) -> Result<Post, Self::Error> {
        let url = compile_url(&self.server_url, &[API_POSTS])?;

        let res = self
            .send_request(Method::POST, url, Some(&cmd), Some(token))
            .await?;
        let post: Post = res.json().await?;

        Ok(post)
    }

    async fn get_post(&self, post_id: PostId) -> Result<Post, Self::Error> {
        let url = compile_url(&self.server_url, &[API_POSTS, &post_id.to_string()])?;

        let res = self
            .send_request::<()>(Method::GET, url, None, None)
            .await?;
        let post: Post = res.json().await?;

        Ok(post)
    }

    async fn update_post(&self, cmd: PostUpdateCmd, token: &Token) -> Result<Post, Self::Error> {
        let post_id = cmd.post_id.to_string();
        let url = compile_url(&self.server_url, &[API_POSTS, &post_id])?;

        let post_cmd_http: PostUpdateCmdHttp = cmd.into();

        let res = self
            .send_request(Method::PUT, url, Some(&post_cmd_http), Some(token))
            .await?;
        let post: Post = res.json().await?;

        Ok(post)
    }

    async fn delete_post(&self, post_id: PostId, token: &Token) -> Result<(), Self::Error> {
        let url = compile_url(&self.server_url, &[API_POSTS, &post_id.to_string()])?;

        self.send_request::<()>(Method::DELETE, url, None, Some(token))
            .await?;

        Ok(())
    }

    async fn list_posts(&self, limit: u32, offset: u32) -> Result<ListPostsResponse, Self::Error> {
        let mut url = compile_url(&self.server_url, &[API_POSTS])?;

        url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());
        url.query_pairs_mut()
            .append_pair("offset", &offset.to_string());

        let res = self
            .send_request::<()>(Method::GET, url, None, None)
            .await?;
        let posts: ListPostsResponse = res.json().await?;

        Ok(posts)
    }
}
