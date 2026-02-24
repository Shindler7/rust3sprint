//! Репозиторий пользователей.

use crate::{
    domain::{types::Username, user::User},
    repo_pg_pool,
};
use sqlx::{postgres::PgRow, Error as SqlxError, PgPool, Row};
use tonic::async_trait;

#[async_trait]
pub(crate) trait UserRepository: Send + Sync {
    /// Создать пользователя.
    async fn create(&self, user: &User) -> Result<User, SqlxError>;
    /// Предоставить экземпляр [`User`] по имени пользователя.
    async fn get_by_username(&self, username: &Username) -> Result<User, SqlxError>;
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
    async fn create(&self, user: &User) -> Result<User, SqlxError> {
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
        .await?;

        Ok(make_user_by_row(&record))
    }

    async fn get_by_username(&self, username: &Username) -> Result<User, SqlxError> {
        let record = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(make_user_by_row(&record))
    }
}

/// Поддерживающая функция: создаёт [`User`] на основе предоставленной записи
/// из базы данных, обёрнутой в [`PgRow`].
fn make_user_by_row(record: &PgRow) -> User {
    User::new(
        record.get("id"),
        record.get("username"),
        record.get("email"),
        record.get("password_hash"),
        record.get("created_at"),
    )
}
