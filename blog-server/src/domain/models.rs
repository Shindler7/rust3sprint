//! Новые типы моделей для domain.

use crate::domain::error::DomainError;
use crate::domain::validators::{validate_email, validate_password};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Единый тип id для User.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    /// Создать новый уникальный Id для пользователя (User).
    pub fn new() -> UserId {
        Self(Uuid::new_v4())
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

/// Единый тип ID для сообщений в блоге.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostId(pub Uuid);

impl PostId {
    /// Создать новый уникальный id поста.
    pub fn new() -> PostId {
        Self(Uuid::new_v4())
    }
}

impl From<PostId> for Uuid {
    fn from(id: PostId) -> Self {
        id.0
    }
}

/// Новый тип для электронного адреса.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Email(pub String);

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match validate_email(&value) {
            Ok(_) => Ok(Self(value)),
            Err(err) => Err(DomainError::invalid_email(err)),
        }
    }
}

impl Email {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Новый тип пароля пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct UserPassword(String);

impl TryFrom<String> for UserPassword {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match validate_password(&value) {
            Ok(_) => Ok(Self(value)),
            Err(err) => Err(DomainError::invalid_password(err)),
        }
    }
}

impl UserPassword {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
