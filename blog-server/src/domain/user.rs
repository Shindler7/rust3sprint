//! Доменные модели.

use crate::{
    domain::types::{DataId, Email, UserPassword, Username},
    infrastructure::jwt::Claims,
};
use chrono::{DateTime, Utc};
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
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Создать новый экземпляр [`User`] на основе предоставленных данных.
    ///
    /// `user_id` и `created_at` могут быть `None`. Если не передано
    /// `created_at`, временная метка конструктором создаётся самостоятельно,
    /// по текущему времени UTC.
    pub(crate) fn new(
        user_id: Option<DataId>,
        username: Username,
        email: Email,
        pwd_hash: &str,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        let created_at = created_at.unwrap_or_else(Utc::now);

        Self {
            id: user_id,
            username,
            email,
            password_hash: pwd_hash.to_string(),
            created_at,
        }
    }

    /// Создание нового экземпляра [`User`] с помощью [`CreateUser`].
    ///
    /// Временная метка создания проставляется автоматически.
    pub(crate) fn new_by_create(create_user: CreateUser, pwd_hash: &str) -> Self {
        User::new(
            None,
            create_user.username,
            create_user.email,
            pwd_hash,
            None,
        )
    }

    /// Преобразовать имя пользователя `username` в нижний регистр.
    pub(crate) fn username_to_lower(mut self) -> Self {
        self.username = self.username.to_lowercase();
        self
    }

    /// Преобразовать адрес электронной почты в нижний регистр.
    pub(crate) fn email_to_lower(mut self) -> Self {
        self.email = self.email.to_lowercase();
        self
    }
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

impl AuthResponse {
    /// Создать экземпляр ответа пользователя.
    pub(crate) fn new(token: impl Into<String>, user: UserDto) -> Self {
        Self {
            token: token.into(),
            user,
        }
    }
}

/// DTO-структура авторизованного пользователя.
///
/// Доступно преобразование из [`Claims`] через `from`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthenticatedUser {
    /// Id пользователя.
    pub(crate) id: DataId,
    /// Имя пользователя (`username`).
    pub(crate) username: Username,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            id: claims.user_id,
            username: claims.username,
        }
    }
}
