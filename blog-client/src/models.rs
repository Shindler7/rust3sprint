//! Клиентские типы и модели для обработки информации.

use crate::config::DISPLAY_TOKEN_CHARS;
use proto_crate::proto_blog::{AuthResponse as ProtoAuthResponse, User};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Новый тип для хранения токена.
///
/// В целях безопасности, методы `Display` и `Debug` предоставляют усечённую
/// версию токена. Для получения валидной версии следует использовать
/// специальные методы: `bearer`, `as_str`.
#[derive(Clone, Serialize, Deserialize)]
pub struct Token(String);

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = DISPLAY_TOKEN_CHARS.min(self.0.len());
        write!(f, "{}...", &self.0[..n])
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = DISPLAY_TOKEN_CHARS.min(self.0.len());
        write!(f, "Токен ({}…)", &self.0[..n])
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Token {
    /// Предоставить токен в формате "Bearer <Token>".
    pub(crate) fn bearer(&self) -> String {
        format!("Bearer {}", self.0)
    }

    /// Предоставить ссылку на тип (`&str`).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Успешный ответ при регистрации и авторизации.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Пользователь.
    pub user: Option<User>,
    /// JWT-токен пользователя.
    pub token: Token,
}

impl From<ProtoAuthResponse> for AuthResponse {
    fn from(resp: ProtoAuthResponse) -> Self {
        Self {
            user: resp.user,
            token: Token(resp.token),
        }
    }
}
