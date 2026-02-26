//! Ошибки клиентского приложения.

use reqwest::{Error as ReqwestError, StatusCode};
use thiserror::Error;
use tonic::transport::Error as TonicError;

/// Ошибки клиента.
#[derive(Error, Debug)]
pub enum BlogClientError {
    /// Данные не найдены.
    #[error("Данные не найдены")]
    NotFound,

    /// Ошибка авторизации на сервере.
    #[error("Ошибка авторизации")]
    Unauthorized,

    /// Некорректный запрос.
    #[error("Некорректный запрос: {0}")]
    InvalidRequest(String),

    /// Некорректная ссылка на API.
    #[error("Неправильный формат ссылки: {0}")]
    InvalidUrl(String),

    /// Токен отсутствует или некорректный.
    #[error("Отсутствует JWT-токен для запроса")]
    TokenNotSet,

    /// Ошибки от [`ReqwestError`].
    #[error(transparent)]
    ReqwestError(ReqwestError),

    /// Ошибки от [`TonicError`].
    #[error(transparent)]
    TonicError(#[from] TonicError),

    /// Ошибки от [`tonic::Status`].
    #[error(transparent)]
    GrpcError(#[from] tonic::Status),

    /// Универсальный тип ошибок клиента.
    #[error("Ошибка исполнения клиента: {0}")]
    ClientError(String),
}

impl BlogClientError {
    /// Конструктор для ошибки [`BlogClientError::InvalidRequest`].
    pub(crate) fn invalid_req(msg_err: impl Into<String>) -> Self {
        Self::InvalidRequest(msg_err.into())
    }

    /// Конструктор для ошибки [`BlogClientError::InvalidUrl`]
    pub(crate) fn invalid_url(msg_err: impl Into<String>) -> Self {
        Self::InvalidUrl(msg_err.into())
    }

    /// Конструктор для ошибки [`BlogClientError::ClientError`]
    pub(crate) fn client_error(msg_err: impl Into<String>) -> Self {
        Self::ClientError(msg_err.into())
    }
}

impl From<ReqwestError> for BlogClientError {
    fn from(e: ReqwestError) -> Self {
        if let Some(status) = e.status()
            && status.is_client_error()
        {
            match status {
                StatusCode::UNAUTHORIZED => BlogClientError::Unauthorized,
                StatusCode::NOT_FOUND => BlogClientError::NotFound,
                _ => BlogClientError::invalid_req(status.to_string()),
            }
        } else {
            Self::ReqwestError(e)
        }
    }
}
