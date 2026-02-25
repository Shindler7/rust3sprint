//! Клиентские типы и модели для обработки информации.

use std::fmt::{Display, Debug};
use crate::config::DISPLAY_TOKEN_CHARS;
use proto_crate::proto_blog::User;

/// Новый тип для хранения токена.
pub struct Token(String);

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = DISPLAY_TOKEN_CHARS.min(self.0.len());
        write!(f, "{}", &self.0[..n])
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = DISPLAY_TOKEN_CHARS.min(self.0.len());
        write!(f, "Токен ({}…)", &self.0[..n])
    }
}

/// Успешный ответ при регистрации и авторизации.
#[derive(Debug)]
pub struct AuthResponse {
    /// Пользователь.
    pub user: User,
    /// JWT-токен пользователя.
    pub token: Token,
}
