//! Доменные модели.

use chrono::{DateTime, Utc};
use crate::domain::types::{DataId, Email, UserPassword, Username};
use serde::{Deserialize, Serialize};

/// Структура пользователя.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct User {
    /// Уникальный id пользователя. Допускается `None` перед сохранением
    /// в базу данных.
    pub id: Option<DataId>,
    /// Username в системе.
    pub username: Username,
    /// Адрес электронной почты.
    pub email: Email,
    /// Хеш пароля пользователя.
    pub password_hash: String,
    /// Время создания пользователя.
    pub created_at: DateTime<Utc>
}

/// DTO-модель для создания пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CreateUser {
    /// Username в системе.
    pub username: Username,
    /// Адрес электронной почты.
    pub email: Email,
    /// Пароль пользователя.
    pub password: UserPassword,
}

/// DTO-модель для авторизации зарегистрированного пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoginUser {
    /// Имя пользователя.
    pub username: Username,
    /// Пароль для доступа к системе.
    pub password: UserPassword,
}

/// Усечённая модель [`User`] для ответов сервера.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserDto {
    /// Уникальный id пользователя (при наличии).
    pub(crate) id: Option<DataId>,
    /// Имя пользователя.
    pub(crate) username: Username,
    /// Адрес электронной почты.
    pub(crate) email: Email,
}

impl From<User> for UserDto {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            email: u.email,
        }
    }
}

/// DTO-структура ответа пользователя при успешных событиях регистрации,
/// авторизации.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthResponse {
    /// JWT-токен пользователя.
    pub(crate) token: String,
    /// DTO-экземпляр пользователя.
    pub(crate) user: UserDto,
}
