//! Обработка ошибок генерации и проверки JWT-токенов.

use crate::errors::DomainError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};

impl From<JwtError> for DomainError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            ErrorKind::InvalidToken | ErrorKind::ExpiredSignature | ErrorKind::InvalidSignature => {
                DomainError::invalid_credentials("токен недействителен")
            }
            _ => DomainError::server_err(err.to_string()),
        }
    }
}
