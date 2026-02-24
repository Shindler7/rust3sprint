//! Ошибки взаимодействия с репозиториями.

use crate::errors::DomainError;
use sqlx::Error as SqlxError;

/// Контекст маппинга SQLx-ошибок в доменные.
pub(crate) struct RepoErrorMap {
    /// Ошибка, соответствующая отсутствию записи.
    pub not_found: DomainError,
    /// Соответствие constraint доменной ошибке.
    pub unique_violations: Option<Vec<(&'static str, DomainError)>>,
}

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
                        None => DomainError::ServerError(db_err.to_string()),
                    }
                } else {
                    DomainError::ServerError(db_err.to_string())
                }
            }
            _ => DomainError::ServerError(err.to_string()),
        })
    }
}
