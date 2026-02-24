//! Ошибки приложения.

pub(crate) mod domain;
pub(crate) mod grpc;
pub(crate) mod http;
pub(crate) mod jws;
mod macros;
pub(crate) mod repo;

pub(crate) use domain::DomainError;
pub(crate) use repo::{RepoErrorMap, SqlxResultExt};
