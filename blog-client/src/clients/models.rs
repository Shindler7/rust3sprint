//! Локальные модели и команды клиентского транспорта.

use proto_crate::proto_blog::{
    CreatePostRequest, LoginRequest, PostResponse, RegisterRequest, UpdatePostRequest,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Обёртка для id публикации.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PostId(i64);

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

impl From<PostId> for i64 {
    fn from(post_id: PostId) -> i64 {
        post_id.0
    }
}

/// Команда на регистрацию пользователя.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRegisterCmd {
    /// Имя пользователя.
    pub(crate) username: String,
    /// Адрес электронной почты.
    pub(crate) email: String,
    /// Пароль.
    pub(crate) password: String,
}

impl From<UserRegisterCmd> for RegisterRequest {
    fn from(u: UserRegisterCmd) -> Self {
        Self {
            username: u.username,
            email: u.email,
            password: u.password,
        }
    }
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
#[derive(Serialize, Deserialize)]
pub(crate) struct UserAuthCmd {
    /// Имя пользователя.
    pub(crate) username: String,
    /// Пароль.
    pub(crate) password: String,
}

impl From<UserAuthCmd> for LoginRequest {
    fn from(u: UserAuthCmd) -> Self {
        Self {
            username: u.username,
            password: u.password,
        }
    }
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
#[derive(Serialize, Deserialize)]
pub(crate) struct PostCreateCmd {
    /// Заголовок публикации.
    pub(crate) title: String,
    /// Содержание публикации.
    pub(crate) content: String,
}

impl From<PostCreateCmd> for CreatePostRequest {
    fn from(post_cmd: PostCreateCmd) -> Self {
        Self {
            title: post_cmd.title,
            content: post_cmd.content,
        }
    }
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
#[derive(Serialize, Deserialize)]
pub(crate) struct PostUpdateCmd {
    /// Id публикации на сервере.
    pub(crate) post_id: PostId,
    /// Новый заголовок публикации (опционально).
    pub(crate) title: Option<String>,
    /// Новое содержание публикации (опционально).
    pub(crate) content: Option<String>,
}

impl From<PostUpdateCmd> for UpdatePostRequest {
    fn from(upd_cmd: PostUpdateCmd) -> Self {
        Self {
            id: upd_cmd.post_id.into(),
            title: upd_cmd.title,
            content: upd_cmd.content,
        }
    }
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

/// Специализированная команда для обновления публикации для HTTP-сервера.
/// Отсутствует post_id в теле структуры.
#[derive(Serialize, Deserialize)]
pub(crate) struct PostUpdateCmdHttp {
    /// Новый заголовок публикации (опционально).
    pub(crate) title: Option<String>,
    /// Новое содержание публикации (опционально).
    pub(crate) content: Option<String>,
}

impl From<PostUpdateCmd> for PostUpdateCmdHttp {
    fn from(p: PostUpdateCmd) -> Self {
        Self {
            title: p.title,
            content: p.content,
        }
    }
}

/// Обёртка для [`PostResponse`].
///
/// При взаимодействии с gRPC-сервером, экземпляр [`Post`] возвращается внутри
/// [`Option`], и требуется дублирование кода в хендлерах для обработки этого
/// состояния.
///
/// Структура позволяет добавить унифицированные методы для решения этой
/// задачи.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct PostResponseWrap {
    /// Объект с публикацией [`Post`] от gRPC-сервера.
    pub(crate) post_response: PostResponse,
}

impl From<PostResponse> for PostResponseWrap {
    fn from(post_resp: PostResponse) -> Self {
        Self {
            post_response: post_resp,
        }
    }
}
