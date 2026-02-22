//! Бизнес-логика блога.

use crate::data::post_repo::PostRepository;
use std::sync::Arc;

/// Сервисы для взаимодействия с записями блога.
pub(crate) struct BlogService<R: PostRepository + 'static> {
    repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: PostRepository + 'static,
{
    /// Создать сервис [`AuthService`] с репозиторием пользователей.
    pub(crate) fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}
