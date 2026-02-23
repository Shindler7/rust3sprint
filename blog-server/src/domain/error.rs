//! Собственные ошибки.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use crate::impl_domain_error_ctor;
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

    /// Публикация содержит некорректные данные (например, слишком короткий
    /// или слишком длинный текст и т.п.)
    #[error("Публикация не соответствует Правилам: {0}")]
    InvalidPostContent(String),

    /// Доступ для пользователя к запрошенному разделу запрещён.
    #[error("Вы не можете изменять эти данные")]
    Forbidden,

    /// Ошибки взаимодействия с API, не закрытые точными типами.
    #[error("Ошибка запроса к API: {0}")]
    ApiError(String),

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

            Self::InvalidEmail(_)
            | Self::InvalidUsername(_)
            | Self::InvalidPostContent(_)
            | Self::ApiError(_) => (StatusCode::BAD_REQUEST, self.to_string()),

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
    impl_domain_error_ctor! {
        /// Конструктор для ошибки [`DomainError::InvalidEmail`].
        fn invalid_email => InvalidEmail;

        /// Конструктор для ошибки [`DomainError::InvalidUsername`].
        fn invalid_username => InvalidUsername;

        /// Конструктор для ошибки [`DomainError::InvalidCredentials`].
        fn invalid_credentials => InvalidCredentials;

        /// Конструктор для ошибки [`DomainError::ServerError`].
        fn server_err => ServerError;

        /// Конструктор для ошибки [`DomainError::InvalidPostContent`].
        fn invalid_post => InvalidPostContent;

        /// Конструктор для ошибки [`DomainError::ApiError`].
        fn api_error => ApiError;

        @custom
        /// Конструктор для ошибки [`DomainError::InvalidPassword`].
        fn invalid_password(err_msg: impl Into<String>) -> DomainError {
            let mut err_msg = err_msg.into();
            if err_msg.is_empty() {
                err_msg = "проверьте ввод, выключен ли Caps Lock".to_string();
            }
            DomainError::InvalidPassword(err_msg)
        }
    }
}

/// Контекст маппинга SQLx-ошибок в доменные.
pub(crate) struct RepoErrorMap {
    /// Ошибка, соответствующая отсутствию записи.
    pub(crate) not_found: DomainError,
    /// Соответствие constraint доменной ошибке.
    // pub(crate) unique_violations: &'static [(&'static str, DomainError)],
    // pub(crate) unique_violations: Vec<(&'static str, DomainError)>,
    pub(crate) unique_violations: Option<Vec<(&'static str, DomainError)>>,
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

                if let Some(uv) = ctx.unique_violations {
                    match uv.into_iter().find(|(c, _)| *c == constraint) {
                        Some((_, domain_err)) => domain_err,
                        None => DomainError::DataBaseInternal(SqlxError::Database(db_err)),
                    }
                } else {
                    DomainError::DataBaseInternal(SqlxError::Database(db_err))
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
