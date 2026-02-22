//! Собственные ошибки.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum DomainError {
    /// Пользователь не найден.
    #[error("Пользователь не найден")]
    UserNotFound,

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
    #[error("Неправильный пароль ({0})")]
    InvalidPassword(String),

    /// Пользователь придумал себе невалидное имя.
    #[error("Некорректный `username`: {0}")]
    InvalidUsername(String),

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

    /// Серверные ошибки взаимодействия с базой данных.
    ///
    /// В качестве аргумента принимает [`SqlxError`].
    #[error("Ошибка базы данных: {0}")]
    DataBaseInternal(#[from] SqlxError),

    /// Ошибка обработки данных на сервере.
    #[error("Ошибка сервера: {0}")]
    ServerError(String),
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let (status, details) = match self {
            Self::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),

            Self::UserNotFound | Self::PostNotFound => (StatusCode::NOT_FOUND, self.to_string()),

            Self::UserAlreadyExists | Self::EmailAlreadyExists => {
                (StatusCode::CONFLICT, self.to_string())
            }

            Self::InvalidCredentials(_) | Self::InvalidPassword(_) => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }

            Self::InvalidEmail(_) | Self::InvalidUsername(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }

            Self::ServerError(_) | Self::DataBaseInternal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": status.as_u16(),
            "details": details
        }))
    }
}

impl DomainError {
    /// Конструктор для ошибки [`DomainError::InvalidEmail`].
    pub(crate) fn invalid_email<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidEmail(err_msg.into())
    }

    /// Конструктор для ошибки [`DomainError::InvalidPassword`].
    pub(crate) fn invalid_password<S: Into<String>>(err_msg: S) -> DomainError {
        let mut err_msg = err_msg.into();
        if err_msg.is_empty() {
            err_msg = "проверьте ввод, выключен ли Caps Lock".to_string();
        }
        DomainError::InvalidPassword(err_msg)
    }

    /// Конструктор для ошибки [`DomainError::InvalidUsername`].
    pub(crate) fn invalid_username<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidUsername(err_msg.into())
    }

    /// Конструктор для ошибки [`DomainError::InvalidCredentials`].
    pub(crate) fn invalid_credentials<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::InvalidCredentials(err_msg.into())
    }

    /// Конструктор для ошибки [`DomainError::ServerError`].
    pub(crate) fn server_err<S: Into<String>>(err_msg: S) -> DomainError {
        DomainError::ServerError(err_msg.into())
    }
}

/// Контекст маппинга SQLx-ошибок в доменные.
pub(crate) struct RepoErrorMap {
    /// Ошибка, соответствующая отсутствию записи.
    pub(crate) not_found: DomainError,
    /// Соответствие constraint доменной ошибке.
    // pub(crate) unique_violations: &'static [(&'static str, DomainError)],
    pub(crate) unique_violations: Vec<(&'static str, DomainError)>,
}

/// Расширение для Result<T, SqlxError>.
pub(crate) trait SqlxResultExt<T> {
    /// Преобразует [`SqlxError`] в DomainError с учетом контекста.
    fn map_repo_err(self, ctx: RepoErrorMap) -> Result<T, DomainError>;
}

impl<T> SqlxResultExt<T> for Result<T, SqlxError> {
    fn map_repo_err(self, ctx: RepoErrorMap) -> Result<T, DomainError> {
        self.map_err(|err| match err {
            SqlxError::RowNotFound => ctx.not_found,
            SqlxError::Database(db_err) => {
                let constraint = db_err.constraint().unwrap_or_default();
                match ctx
                    .unique_violations
                    .into_iter()
                    .find(|(c, _)| *c == constraint)
                {
                    Some((_, domain_err)) => domain_err,
                    None => DomainError::DataBaseInternal(SqlxError::Database(db_err)),
                }
            }
            _ => DomainError::DataBaseInternal(err),
        })
    }
}

impl From<JwtError> for DomainError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            ErrorKind::InvalidToken | ErrorKind::ExpiredSignature | ErrorKind::InvalidSignature => {
                DomainError::invalid_credentials("токен недействителен")
            }
            _ => DomainError::ServerError(err.to_string()),
        }
    }
}
