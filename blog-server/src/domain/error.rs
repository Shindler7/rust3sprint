//! Собственные ошибки.

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    /// Пользователь не найден.
    #[error("Пользователь {0} не найден")]
    UserNotFound(String),

    /// Пользователь пытается зарегистрироваться повторно.
    #[error("Пользователь с таким именем уже зарегистрирован")]
    UserAlreadyExists,

    /// Ошибка валидации адреса электронной почты.
    #[error("Некорректный email: {0}")]
    InvalidEmail(String),

    /// Электронный адрес уже был ранее использован.
    #[error("Указанный email уже использован")]
    EmailAlreadyExists,

    /// Ошибка при валидации пароля.
    #[error("Некорректный пароль: {0}")]
    InvalidPassword(String),

    /// Ошибка данных пользователя (например, при авторизации).
    #[error("Ошибка учётных данных: {0}")]
    InvalidCredentials(String),

    /// Пост (публикация) не найден (в том числе для ситуаций, когда пытаются
    /// найти чужой пост).
    #[error("Публикация не найдена")]
    PostNotFound,

    /// Доступ для пользователя к запрошенному разделу запрещён.
    #[error("Вы не можете изменять эти данные")]
    Forbidden,
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let (status, details) = match self {
            Self::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),

            Self::UserNotFound(_) | Self::PostNotFound => (StatusCode::NOT_FOUND, self.to_string()),

            Self::UserAlreadyExists | Self::EmailAlreadyExists => {
                (StatusCode::CONFLICT, self.to_string())
            }

            Self::InvalidEmail(_) | Self::InvalidCredentials(_) | Self::InvalidPassword(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": status.as_u16(),
            "details": details
        }))
    }
}

impl DomainError {
    /// Конструктор для ошибки [`DomainError::UserNotFound`].
    ///
    /// ## Args
    /// - `name` — запрошенный `username`
    pub fn user_not_found<S: Into<String>>(name: S) -> DomainError {
        DomainError::UserNotFound(name.into())
    }

    /// Конструктор для ошибки [`DomainError::InvalidEmail`].
    pub fn invalid_email<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidEmail(err_msg.into())
    }

    /// Конструктор для ошибки [`DomainError::InvalidPassword`].
    pub fn invalid_password<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidPassword(err_msg.into())
    }

    /// Конструктор для ошибки [`DomainError::InvalidCredentials`].
    pub fn invalid_credentials<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidCredentials(err_msg.into())
    }
}
