//! Доменные модели.

use serde::{Deserialize, Serialize};
use crate::domain::models::{Email, UserId, UserPassword};

/// Структура пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Уникальный id пользователя.
    pub id: UserId,
    /// Username в системе.
    pub username: String,
    /// Адрес электронной почты.
    pub email: Email,
    /// Хеш пароля пользователя.
    pub password_hash: String,
}

/// DTO-модель для создания пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    /// Username в системе.
    pub username: String,
    /// Адрес электронной почты.
    pub email: Email,
    /// Пароль пользователя.
    pub password: UserPassword,
}

/// DTO-модель для авторизации зарегистрированного пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: UserPassword,
}
