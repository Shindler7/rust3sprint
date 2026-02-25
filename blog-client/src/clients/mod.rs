//! Клиентская инфраструктура взаимодействия с серверными API.

use crate::{
    error::BlogClientError,
    models::{AuthResponse, Token},
};
use proto_crate::proto_blog::Post;

mod grpc_client;
mod http_client;

/// Доступный транспорт для запросов к API.
#[derive(Debug)]
pub enum Transport {
    /// HTTP-сервер.
    Http(String),
    /// gRPC-сервер.
    Grpc(String),
}

/// Структура клиента.
#[derive(Debug)]
pub struct BlogClient {
    /// Установка транспорта.
    pub transport: Transport,
    /// Используемый клиент для HTTP-запросов.
    http_client: Option<reqwest::Client>,
    /// Используемый клиент для gRPC-запросов.
    grpc_url: Option<String>,
    /// Токен для взаимодействия с API.
    token: Option<Token>,
}

impl BlogClient {
    /// Создать новый клиент для взаимодействия с API.
    pub fn new(transport: Transport) -> Self {
        Self {
            transport,
            http_client: None,
            grpc_url: None,
            token: None,
        }
    }

    /// Добавить JWT-токен клиенту.
    fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    /// Получить JWT-токен.
    fn get_token(&self) -> &Option<Token> {
        &self.token
    }

    /// Регистрация пользователя.
    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        todo!()
    }

    /// Авторизация пользователя.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        todo!()
    }

    /// Создание публикации.
    ///
    /// Требуется предварительная авторизация.
    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        todo!()
    }

    /// Получение публикации.
    pub async fn get_post(&self, post_id: i64) -> Result<Post, BlogClientError> {
        todo!()
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
        todo!()
    }

    /// Удаление публикации.
    ///
    /// Требуется предварительная авторизация.
    pub async fn delete_post(&self, post_id: i64) -> Result<(), BlogClientError> {
        todo!()
    }

    /// Получение списка постов с пагинацией.
    pub async fn list_posts(&self) -> Result<Vec<Post>, BlogClientError> {
        todo!()
    }
}
