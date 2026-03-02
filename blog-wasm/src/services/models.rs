//! Модели для организации запросов и получения информации.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Экземпляр пользователя.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct User {
    /// Уникальный id пользователя (при наличии).
    pub(crate) id: u64,
    /// Имя пользователя.
    pub(crate) username: String,
    /// Адрес электронной почты.
    pub(crate) email: String,
}

/// Экземпляр ответа по авторизованному пользователю.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct AuthResponse {
    /// JWT-токен пользователя.
    pub(crate) token: String,
    /// DTO-экземпляр пользователя.
    pub(crate) user: User,
}

/// DTO, регистрация пользователя.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct RegisterUser {
    /// Имя пользователя.
    pub(crate) username: String,
    /// Электронный адрес.
    pub(crate) email: String,
    /// Пароль.
    pub(crate) password: String,
}

impl RegisterUser {
    /// Создать DTO для регистрации пользователя.
    pub(crate) fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// Dto-форма для авторизации пользователя.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct LoginUser {
    /// Имя пользователя (username).
    pub(crate) username: String,
    /// Пароль.
    pub(crate) password: String,
}

impl LoginUser {
    /// Предоставить новый экземпляр для авторизации пользователя.
    pub(crate) fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// Структура сообщения (поста) в блоге.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Post {
    /// Уникальный id сообщения. Допускается `None` при создании экземпляра
    /// перед сохранением в базу данных.
    pub(crate) id: u64,
    /// Заголовок сообщения.
    pub(crate) title: String,
    /// Содержание сообщения.
    pub(crate) content: String,
    /// Id автора поста, на основе [`UserId`].
    pub(crate) author_id: u64,
    /// Время создания поста.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub(crate) created_at: DateTime<Utc>,
    /// Время, когда пост был обновлён.
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

/// Успешный ответ со списком публикаций в блоге.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct ListPostsResponse {
    /// Перечень публикаций.
    pub(crate) posts: Vec<Post>,
    /// Всего публикаций.
    pub(crate) total: i32,
    /// Заказанное количество публикаций.
    pub(crate) limit: i32,
    /// Сдвиг по публикациям.
    pub(crate) offset: i32,
}

/// Dto-структура для создания записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CreatePost {
    /// Заголовок поста.
    pub(crate) title: String,
    /// Содержимое поста.
    pub(crate) content: String,
}

impl CreatePost {
    pub(crate) fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

/// Dto-структура для редактирования записи (поста).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct EditPost {
    /// Заголовок поста.
    pub(crate) title: Option<String>,
    /// Содержимое поста.
    pub(crate) content: Option<String>,
}

impl EditPost {
    pub(crate) fn new(title: &str, content: &str) -> Self {
        Self {
            title: Some(title.to_string()),
            content: Some(content.to_string()),
        }
    }
}
