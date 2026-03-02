//! Модели работы с данными.

/// Состояния регистрации и авторизации пользователя.
#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) enum AuthStatus {
    /// Пользователь авторизован.
    Signin,

    /// Идёт процесс аутентификации.
    #[default]
    InProcess,

    /// Ошибка аутентификации.
    Error(String),

    /// Пользователь не авторизован.
    Logout,
}

impl AuthStatus {
    /// Конструктор для состояния [`AuthStatus::Error`].
    pub(crate) fn auth_error<S: Into<String>>(msg: S) -> Self {
        AuthStatus::Error(msg.into())
    }
}

/// Состояние отображения публикаций.
#[derive(Clone, PartialEq)]
pub(crate) enum PostsStatus {
    /// Стартовое.
    Idle,
    /// Загрузка публикаций.
    Loading,
    /// Ошибка при загрузке данных.
    Error(String),
}
