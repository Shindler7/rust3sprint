//! Ошибки клиентского приложения.

use reqwest::Error as ReqwestError;
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

    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),

    #[error(transparent)]
    TonicError(#[from] TonicError),

    #[error(transparent)]
    GrpcError(#[from] tonic::Status),
}

impl BlogClientError {
    /// Конструктор для ошибки [`BlogClientError::InvalidRequest`].
    pub(crate) fn invalid_req(msg_err: impl Into<String>) -> Self {
        Self::InvalidRequest(msg_err.into())
    }
}
