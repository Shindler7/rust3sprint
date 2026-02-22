//! Элементы приложения.

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    data::{post_repo::PostRepo, user_repo::UserRepo},
};
use sqlx::PgPool;
use std::sync::Arc;

pub(crate) mod auth_service;
pub(crate) mod blog_service;

/// Структура сервисов обработки данных.
#[derive(Clone)]
pub(crate) struct AppServices {
    /// Взаимодействие с пользовательскими сессиями.
    pub(crate) auth_service: Arc<AuthService<UserRepo>>,
    /// Взаимодействие с публикацией постов.
    pub(crate) blog_service: Arc<BlogService<PostRepo>>,
}

impl AppServices {
    pub(crate) fn new(pool: &PgPool) -> Self {
        let user_repo = Arc::new(UserRepo::new(pool));
        let blog_repo = Arc::new(PostRepo::new(pool));

        let auth_service = AuthService::new(user_repo);
        let blog_service = BlogService::new(blog_repo);

        Self {
            auth_service: Arc::new(auth_service),
            blog_service: Arc::new(blog_service),
        }
    }
}
