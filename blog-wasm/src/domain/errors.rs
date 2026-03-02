//! Ошибки приложения.

use thiserror::Error;
use gloo_net::Error as GlooErr;

/// Доменные ошибки приложения, включая конвертированные из других типов.
#[derive(Debug, Error)]
pub(crate) enum BlogWasmError {
    /// Фатальные ошибки, при которых дальнейшая полноценная работа приложения
    /// невозможна.
    #[error("Внутренняя ошибка приложения")]
    InternalFatalError,

    /// Ошибка неавторизованных действий.
    #[error("Для этой операции требуется авторизация")]
    Forbidden,

    /// Ошибки от API.
    #[error(transparent)]
    ApiError(#[from] GlooErr),

    /// Ошибки для некорректных данных от пользователя.
    #[error("Некорректный ввод: {0}")]
    UserDataError(String),
}

impl BlogWasmError {
    /// Конструктор для ошибки [`BlogWasmError::UserDataError`].
    pub(crate) fn user_data_err(err_msg: impl Into<String>) -> Self {
        Self::UserDataError(err_msg.into())
    }
}
