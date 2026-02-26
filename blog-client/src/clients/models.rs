//! Локальные модели и команды клиентского транспорта.

use std::fmt::Display;
use serde::{Serialize, Deserialize};

/// Обёртка для id публикации.
pub(crate) struct PostId(i64);

impl Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for PostId {
    fn from(post_id: i64) -> Self {
        Self(post_id)
    }
}

/// Команда на регистрацию пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRegisterCmd {
    /// Имя пользователя.
    username: String,
    /// Адрес электронной почты.
    email: String,
    /// Пароль.
    password: String,
}

impl UserRegisterCmd {
    /// Создать команду на основе предоставленных значений.
    pub(crate) fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

/// Команда на авторизацию пользователя.
pub(crate) struct UserAuthCmd {
    /// Имя пользователя.
    username: String,
    /// Пароль.
    password: String,
}

impl UserAuthCmd {
    /// Создать команду для авторизации пользователя.
    pub(crate) fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// Команда создания публикации.
pub(crate) struct PostCreateCmd {
    /// Заголовок публикации.
    title: String,
    /// Содержание публикации.
    content: String,
}

impl PostCreateCmd {
    /// Создание команды для размещения публикации.
    pub(crate) fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

/// Команда обновления публикации.
pub(crate) struct PostUpdateCmd {
    /// Id публикации на сервере.
    post_id: PostId,
    /// Новый заголовок публикации (опционально).
    title: Option<String>,
    /// Новое содержание публикации (опционально).
    content: Option<String>,
}

impl PostUpdateCmd {
    /// Создание команды для изменения публикации.
    pub(crate) fn new(post_id: i64, title: Option<&str>, content: Option<&str>) -> Self {
        let post_id = PostId(post_id);
        let title = title.map(String::from);
        let content = content.map(String::from);

        Self {
            post_id,
            title,
            content,
        }
    }
}
