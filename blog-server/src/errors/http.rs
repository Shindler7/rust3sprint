//! Ошибки HTTP-сервера.

use crate::errors::DomainError;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let (status, details) = match self {
            DomainError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            DomainError::UserNotFound | DomainError::PostNotFound => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            DomainError::UserAlreadyExists | DomainError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, self.to_string())
            }
            DomainError::InvalidCredentials(_) | DomainError::InvalidPassword(_) => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            DomainError::InvalidEmail(_)
            | DomainError::InvalidUsername(_)
            | DomainError::InvalidPostContent(_)
            | DomainError::ApiError(_) => (StatusCode::BAD_REQUEST, self.to_string()),

            DomainError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": status.as_u16(),
            "details": details
        }))
    }
}
