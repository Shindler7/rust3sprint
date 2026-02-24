//! Доменные ошибки.

use crate::impl_domain_error_ctor;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
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

    /// Ошибка обработки данных на сервере.
    #[error("Ошибка сервера: {0}")]
    ServerError(String),
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
