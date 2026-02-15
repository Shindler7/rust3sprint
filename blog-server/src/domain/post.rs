//! Модели для сообщений в блоге.

use crate::domain::models::{PostId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Структура сообщения (поста) в блоге.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// Уникальный id сообщения.
    id: PostId,
    /// Заголовок сообщения.
    title: String,
    /// Содержание сообщения.
    content: String,
    /// Id автора поста, на основе [`UserId`].
    author_id: UserId,
    /// Время создания поста.
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    /// Время, когда пост был обновлён.
    #[serde(with = "chrono::serde::ts_seconds_option")]
    updated_at: Option<DateTime<Utc>>,
}

impl Post {
    /// Создание экземпляра [`Post`] на основе предоставленных данных.
    ///
    /// [`PostId`] создаётся уникальный, а валидность [`UserId`]
    /// не проверяется.
    pub fn new(author_id: UserId, post: &CreatePost) -> Self {
        let id = PostId::new();
        let created_at = Utc::now();

        Self {
            id,
            title: post.title.clone(),
            content: post.content.clone(),
            author_id,
            created_at,
            updated_at: None,
        }
    }

    /// Обновить экземпляр на основе отредактированных данных.
    pub fn update(&mut self, edit_post: &EditPost) {
        let update_at = Utc::now();

        self.updated_at = Some(update_at);
        self.title = edit_post.title.clone();
        self.content = edit_post.content.clone();
    }
}

/// Dto-структура для создания записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePost {
    /// Заголовок поста.
    title: String,
    /// Содержимое поста.
    content: String,
}

/// Dto-структура для редактирования записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditPost {
    /// Заголовок поста.
    title: String,
    /// Содержимое поста.
    content: String,
}
