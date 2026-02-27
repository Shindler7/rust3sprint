//! Клиентский модуль взаимодействия с сервером.
//!
//! Это библиотека, которая используется CLI и WASM-фронтендом.

pub(crate) mod clients;
mod config;
pub mod error;
pub mod models;

use crate::{
    clients::{
        grpc_client::GrpcClient,
        http_client::HttpClient,
        models::{PostCreateCmd, PostId, PostUpdateCmd, UserAuthCmd, UserRegisterCmd},
        traits::ClientTransportExt,
    },
    models::{AuthResponse, Token},
};
pub use error::BlogClientError;
use proto_crate::proto_blog::{ListPostsResponse, Post};
use reqwest::Url;
use tonic::transport::Uri;

/// Доступный транспорт для запросов к API.
///
/// Для выбора нужного заказа рекомендуется использовать предоставленные
/// конструкторы.
///
/// ## Пример
///
/// ```
/// use blog_client::Transport;
///
/// let transport = Transport::http("http://127.0.0.1").unwrap();
/// ```
///
/// ## Errors
///
/// Если предоставленная конструктору ссылка не может быть успешно
/// преобразована в `url`, выбросится ошибка [`BlogClientError::InvalidUrl`].
#[derive(Debug)]
pub enum Transport {
    /// HTTP-сервер.
    Http(Url),
    /// gRPC-сервер.
    Grpc(Uri),
}

impl Transport {
    /// Заказ на использование HTTP-транспорта.
    ///
    /// ## Args
    ///
    /// - `url` — строковое представление ссылки к серверу.
    pub fn http<S: Into<String>>(url: S) -> Result<Self, BlogClientError> {
        let url =
            Url::parse(&url.into()).map_err(|err| BlogClientError::invalid_url(err.to_string()))?;

        Ok(Self::Http(url))
    }

    /// Заказ на использование gRPC-транспорта.
    ///
    /// ## Args
    ///
    /// - `url` — строковое представление ссылки к серверу.
    pub fn grpc<S: Into<String>>(url: S) -> Result<Self, BlogClientError> {
        let uri: Uri = url
            .into()
            .parse()
            .map_err(|_| BlogClientError::invalid_url("gRPC-клиент"))?;

        Ok(Self::Grpc(uri))
    }
}

/// Структура клиента.
#[derive(Debug)]
pub struct BlogClient {
    /// Установка транспорта.
    transport: Transport,
    /// Используемый клиент для HTTP-запросов.
    http_client: Option<HttpClient>,
    /// Используемый клиент для gRPC-запросов.
    grpc_client: Option<GrpcClient>,
    /// Токен для взаимодействия с API.
    token: Option<Token>,
}

impl BlogClient {
    /// Создать новый клиент для взаимодействия с API.
    ///
    /// Клиент предоставляет HTTP и gRPC транспорты для взаимодействия
    /// с сервером.
    ///
    /// ## Пример
    ///
    /// ```ignore
    /// use blog_client::{Transport, BlogClient};
    ///
    /// let server_url = "http:127.0.0.1:8080";
    /// let transport = Transport::http(server_url).unwrap();
    ///
    /// let mut client = BlogClient::new(transport).await.unwrap();
    /// let result = client.list_posts(10, 0).await.unwrap();
    /// ```
    ///
    /// ## Ошибки
    ///
    /// При заказе HTTP-транспорта может возникнуть ошибка: если не удалось
    /// инициализировать TLS-бэкенд (например, `rustls` или `native-tls`),
    /// либо конструктору HTTP-клиента не удаётся загрузить системные настройки.
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let (http_client, grpc_client) = match transport {
            Transport::Http(ref url) => {
                let client = HttpClient::new(url.clone()).map_err(|err| {
                    BlogClientError::client_error(format!("ошибка создания HTTP-клиента ({err})"))
                })?;
                (Some(client), None)
            }
            Transport::Grpc(ref uri) => {
                let client = GrpcClient::new(uri.clone()).await?;
                (None, Some(client))
            }
        };

        Ok(Self {
            transport,
            http_client,
            grpc_client,
            token: None,
        })
    }

    /// Добавить JWT-токен клиенту.
    fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    /// Получить JWT-токен. Если токен отсутствует, вернётся ошибка
    /// [`BlogClientError::TokenNotSet`].
    fn get_token(&self) -> Result<&Token, BlogClientError> {
        self.token.as_ref().ok_or(BlogClientError::TokenNotSet)
    }

    /// Предоставить установленный транспорт (например, HTTP или gRPC) для
    /// создания запроса.
    fn transport(&self) -> &dyn ClientTransportExt<Error = BlogClientError> {
        match &self.transport {
            Transport::Http(_) => self
                .http_client
                .as_ref()
                .expect("Транспорт HTTP не установлен"),
            Transport::Grpc(_) => self
                .grpc_client
                .as_ref()
                .expect("Транспорт GRPC не установлен"),
        }
    }

    /// Регистрация пользователя.
    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let register_cmd = UserRegisterCmd::new(username, email, password);
        let auth_response = self.transport().register_user(register_cmd).await?;

        self.set_token(auth_response.token.clone());
        Ok(auth_response)
    }

    /// Авторизация пользователя.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let auth_cmd = UserAuthCmd::new(username, password);
        let auth_response = self.transport().login_user(auth_cmd).await?;

        self.set_token(auth_response.token.clone());
        Ok(auth_response)
    }

    /// Создание публикации.
    ///
    /// Требуется предварительная авторизация.
    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let token = self.get_token()?;

        let create_post_cmd = PostCreateCmd::new(title, content);
        self.transport().create_post(create_post_cmd, token).await
    }

    /// Получение публикации.
    pub async fn get_post(&self, post_id: i64) -> Result<Post, BlogClientError> {
        let post_id: PostId = post_id.into();
        self.transport().get_post(post_id).await
    }

    /// Обновление публикации.
    ///
    /// Требуется предварительная авторизация.
    pub async fn update_post(
        &self,
        post_id: i64,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Post, BlogClientError> {
        let token = self.get_token()?;

        let update_post_cmd = PostUpdateCmd::new(post_id, title, content);
        self.transport().update_post(update_post_cmd, token).await
    }

    /// Удаление публикации.
    ///
    /// Требуется предварительная авторизация.
    pub async fn delete_post(&self, post_id: i64) -> Result<(), BlogClientError> {
        let token = self.get_token()?;

        let post_id: PostId = post_id.into();
        self.transport().delete_post(post_id, token).await
    }

    /// Просмотр публикаций с пагинацией.
    ///
    /// ## Args
    ///
    /// - `limit` — количество возвращаемых записей (опционально), по умолчанию
    ///   значение равно 10.
    /// - `offset` — количество записей для пропуска (опционально), по
    ///   умолчанию значение равно 0.
    ///
    /// Сервер может устанавливать ограничения по значениям.
    pub async fn list_posts(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ListPostsResponse, BlogClientError> {
        self.transport()
            .list_posts(limit.unwrap_or(10), offset.unwrap_or(0))
            .await
    }
}
