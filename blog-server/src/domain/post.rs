//! Модели для сообщений в блоге.

use crate::domain::types::{DataId, PostContent, PostTitle};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Структура сообщения (поста) в блоге.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    /// Уникальный id сообщения. Допускается `None` при создании экземпляра
    /// перед сохранением в базу данных.
    pub(crate) id: Option<DataId>,
    /// Заголовок сообщения.
    pub(crate) title: PostTitle,
    /// Содержание сообщения.
    pub(crate) content: PostContent,
    /// Id автора поста, на основе [`UserId`].
    pub(crate) author_id: DataId,
    /// Время создания поста.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub(crate) created_at: DateTime<Utc>,
    /// Время, когда пост был обновлён.
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

impl Post {
    /// Создание экземпляра [`Post`] на основе предоставленных данных.
    ///
    /// Если `created_at` не передано, конструктор самостоятельно создаёт
    /// временную метку на основе текущего времени UTC.
    ///
    /// ## Важно
    ///
    /// `created_at` и `updated_at` принимаются "на веру", конструктор
    /// не проверяет их соотношение (кто раньше). Эта логика должна
    /// исследоваться у вызывающего.
    pub(crate) fn new(
        post_id: Option<DataId>,
        title: PostTitle,
        content: PostContent,
        author_id: DataId,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        let created_at = created_at.unwrap_or_else(Utc::now);

        Self {
            id: post_id,
            title,
            content,
            author_id,
            created_at,
            updated_at,
        }
    }

    /// Создать новый экземпляр [`Post`] с помощью [`CreatePost`].
    ///
    /// Временная метка проставляется автоматически.
    pub(crate) fn new_by_create(post: CreatePost, author_id: DataId) -> Self {
        Post::new(None, post.title, post.content, author_id, None, None)
    }

    /// Проверяет совпадение автора публикации с ID пользователя.
    pub(crate) fn is_author(&self, user_id: &DataId) -> bool {
        self.author_id.eq(user_id)
    }

    /// Обновить экземпляр на основе отредактированных данных.
    ///
    /// Автоматически проставляется временная метка внесения изменений.
    ///
    /// ## Важно
    ///
    /// Исходя из принципа разделения ответственности, метод не реагирует
    /// на состояние, когда все поля пришли неизменными (`None`). В этом
    /// случае сохранится исходное состояние, и метка изменения проставляться
    /// не будет.
    pub(crate) fn update(&mut self, edit_post: &EditPost) {
        let update_at = Utc::now();
        let mut updated = false;

        if let Some(title) = edit_post.title.clone() {
            self.title = title;
            updated = true;
        }

        if let Some(content) = edit_post.content.clone() {
            self.content = content;
            updated = true;
        }

        if updated {
            self.updated_at = Some(update_at);
        }
    }
}

/// Dto-структура для создания записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CreatePost {
    /// Заголовок поста.
    pub(crate) title: PostTitle,
    /// Содержимое поста.
    pub(crate) content: PostContent,
}

/// Dto-структура для редактирования записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct EditPost {
    /// Заголовок поста.
    pub(crate) title: Option<PostTitle>,
    /// Содержимое поста.
    pub(crate) content: Option<PostContent>,
}

/// Команда обновления поста.
pub(crate) struct EditPostCommand {
    /// Идентификатор поста.
    pub(crate) post_id: DataId,
    /// Изменяемые поля.
    pub(crate) edit_post: EditPost,
}

impl EditPostCommand {
    /// Создать экземпляр команды на редактирование публикации.
    pub(crate) fn new(post_id: DataId, edit_post: EditPost) -> Self {
        Self { post_id, edit_post }
    }
}

/// Dto-структура query-параметров для извлечения перечня постов.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct QueryPosts {
    /// Количество возвращаемых записей.
    pub(crate) limit: Option<u32>,
    /// Количество записей, которые необходимо пропустить.
    pub(crate) offset: Option<u32>,
}

impl Default for QueryPosts {
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}
