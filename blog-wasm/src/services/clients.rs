//! Инфраструктура создания клиентов для взаимодействия с внешними API.

use crate::{
    domain::{
        errors::BlogWasmError,
        types::{JwtToken, PostId},
    },
    services::{
        models::{
            AuthResponse, CreatePost, EditPost, ListPostsResponse, LoginUser, Post, RegisterUser,
        },
        tools::get_base_api_url,
    },
};
use gloo_net::http::Request;
use tracing::error;
use url::Url;

/// Универсальный адаптер для взаимодействия с API.
///
/// Создавать экземпляр нужно используя требуемый транспорт доступа
/// (http, grpc...), через соответствующий метод.
pub(crate) struct BlogClient {
    /// Созданный экземпляр клиента для доступа к API.
    api_url: Url,
}

impl BlogClient {
    /// Создать экземпляр для доступа к API.
    pub(crate) fn new() -> Result<Self, BlogWasmError> {
        let server_url = get_base_api_url().ok_or_else(|| {
            error!("Отсутствует корректная ссылка на API сервиса HTTP");
            BlogWasmError::InternalFatalError
        })?;

        let api_url = Url::parse(&server_url).map_err(|err| {
            error!(
                error=%err,
                "Некорректная ссылка для API"
            );
            BlogWasmError::InternalFatalError
        })?;

        Ok(Self { api_url })
    }

    /// Регистрация пользователя через API.
    pub(crate) async fn register(
        &self,
        reg_user: &RegisterUser,
    ) -> Result<AuthResponse, BlogWasmError> {
        let url_api = self.make_url(&["api", "auth", "register"])?;
        let resp = Request::post(url_api.as_str())
            .json(reg_user)?
            .send()
            .await?;

        let auth: AuthResponse = resp.json().await?;

        Ok(auth)
    }

    /// Авторизация пользователя.
    pub(crate) async fn login(&self, auth_user: &LoginUser) -> Result<AuthResponse, BlogWasmError> {
        let url_api = self.make_url(&["api", "auth", "login"])?;
        let resp = Request::post(url_api.as_str())
            .json(auth_user)?
            .send()
            .await?;

        let auth: AuthResponse = resp.json().await?;

        Ok(auth)
    }

    /// Загрузить список публикаций через API.
    ///
    /// ## Args
    ///
    /// - `limit` — количество новостей для выгрузки
    /// - `offset` — сдвиг для пагинации
    pub(crate) async fn load_posts(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<ListPostsResponse, BlogWasmError> {
        let url_api = self.make_url(&["api", "posts"])?;

        let resp = Request::get(url_api.as_str())
            .query([
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ])
            .send()
            .await?;

        let posts: ListPostsResponse = resp.json().await?;

        Ok(posts)
    }

    /// Получить отдельный пост по его id.
    pub(crate) async fn get_post(&self, post_id: PostId) -> Result<Post, BlogWasmError> {
        let url_api = self.make_url(&["api", "posts", post_id.to_string().as_str()])?;

        let resp = Request::get(url_api.as_str()).send().await?;

        let post: Post = resp.json().await?;
        Ok(post)
    }

    /// Создать публикацию через API.
    pub(crate) async fn create_post(
        &self,
        create_post: &CreatePost,
        jwt_token: Option<JwtToken>,
    ) -> Result<Post, BlogWasmError> {
        let token = jwt_token.ok_or(BlogWasmError::Forbidden)?;
        let url_api = self.make_url(&["api", "posts"])?;

        let resp = Request::post(url_api.as_str())
            .header("Authorization", &token.bearer())
            .json(create_post)?
            .send()
            .await?;

        let post: Post = resp.json().await?;
        Ok(post)
    }

    /// Обновить публикацию через API.
    pub(crate) async fn update_post(
        &self,
        post_id: PostId,
        edit_post: &EditPost,
        jwt_token: Option<JwtToken>,
    ) -> Result<Post, BlogWasmError> {
        let token = jwt_token.ok_or(BlogWasmError::Forbidden)?;
        let url_api = self.make_url(&["api", "posts", post_id.to_string().as_str()])?;

        let resp = Request::put(url_api.as_str())
            .header("Authorization", &token.bearer())
            .json(edit_post)?
            .send()
            .await?;

        let post: Post = resp.json().await?;
        Ok(post)
    }

    /// Удалить публикацию через API.
    pub(crate) async fn delete_post(
        &self,
        post_id: PostId,
        jwt_token: Option<JwtToken>,
    ) -> Result<(), BlogWasmError> {
        let token = jwt_token.ok_or(BlogWasmError::Forbidden)?;

        let post_id = post_id.to_string();
        let url = self.make_url(&["api", "posts", &post_id])?;

        Request::delete(url.as_str())
            .header("Authorization", &token.bearer())
            .send()
            .await?;

        Ok(())
    }

    /// Локальный метод обеспечивающий сборку ссылки для доступа к API.
    /// При ошибках генерирует ошибку [`BlogWasmError::InternalFatalError`],
    /// т.к. это поведение противоречит стабильной работе.
    fn make_url(&self, endpoint: &[&str]) -> Result<Url, BlogWasmError> {
        self.api_url.join(&endpoint.join("/")).map_err(|err| {
            error!(
                error=%err,
                endpoint=?endpoint,
                "Некорректная ссылка для API сервера HTTP"
            );
            BlogWasmError::InternalFatalError
        })
    }
}
