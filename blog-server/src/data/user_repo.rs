//! Репозиторий пользователей.

use crate::domain::error::DomainError::UserNotFound;
use crate::domain::error::{RepoErrorMap, SqlxResultExt};
use crate::domain::types::Username;
use crate::{
    domain::{error::DomainError, user::User},
    repo_pg_pool,
};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use tonic::async_trait;

#[async_trait]
pub(crate) trait UserRepository: Send + Sync {
    /// Создать пользователя.
    async fn create(&self, user: &User) -> Result<User, DomainError>;
    /// Предоставить экземпляр [`User`] по имени пользователя.
    async fn get_by_username(&self, username: &Username) -> Result<User, DomainError>;
}

repo_pg_pool!(
    #[derive(Clone)]
    /// Структура взаимодействия с базой данной для работы с `user`.
    pub(crate) struct UserRepo;
);

#[async_trait]
impl UserRepository for UserRepo {
    /// Создать нового пользователя на основе объекта [`User`]. При успехе
    /// будет возвращён обновлённый объект пользователя, с полями на основе
    /// данных в Базе.
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let User {
            username,
            email,
            password_hash,
            created_at,
            ..
        } = user;

        let record = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await
        .map_repo_err(RepoErrorMap {
            not_found: UserNotFound,
            unique_violations: vec![
                ("users_username_key", DomainError::UserAlreadyExists),
                ("users_email_key", DomainError::EmailAlreadyExists),
            ],
        })?;

        Ok(make_user_by_row(&record))
    }

    async fn get_by_username(&self, username: &Username) -> Result<User, DomainError> {
        let record = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_repo_err(RepoErrorMap {
            not_found: UserNotFound,
            unique_violations: vec![],
        })?;

        Ok(make_user_by_row(&record))
    }
}

/// Поддерживающая функция: создаёт [`User`] на основе предоставленной записи
/// из базы данных, обёрнутой в [`PgRow`].
fn make_user_by_row(record: &PgRow) -> User {
    User {
        id: record.get("id"),
        username: record.get("username"),
        email: record.get("email"),
        password_hash: record.get("password_hash"),
        created_at: record.get("created_at"),
    }
}
