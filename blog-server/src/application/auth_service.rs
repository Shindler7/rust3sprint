//! Сервис аутентификации.

use crate::domain::types::Username;
use crate::{
    data::user_repo::UserRepository,
    domain,
    domain::{
        error::DomainError,
        user::{CreateUser, User},
    },
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{info, instrument};

/// Сервис аутентификации.
pub(crate) struct AuthService<R: UserRepository + 'static> {
    /// Репозиторий пользователей.
    repo: Arc<R>,
}

impl<R> AuthService<R>
where
    R: UserRepository + 'static,
{
    /// Создать сервис [`AuthService`] с репозиторием пользователей.
    pub(crate) fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Создать нового пользователя на основе [`CreateUser`].
    ///
    /// За счёт использования новых типов (например, [`types::Username`])
    /// данные валидированы на этапе создания.
    #[instrument(
        skip(self, create_user),
        level = "debug",
        fields(username = %create_user.username, email=%create_user.email))]
    pub(crate) async fn create_user(&self, create_user: &CreateUser) -> Result<User, DomainError> {
        let password_hash = create_user
            .password
            .hash()
            .map_err(|err| DomainError::server_err(err.to_string()))?;

        let created_at = Utc::now();

        let user = User {
            id: None,
            username: create_user.username.to_lowercase(),
            email: create_user.email.to_lowercase(),
            password_hash,
            created_at,
        };

        let user = self.repo.create(&user).await?;
        info!("Создан новый пользователь: {}", user.username);

        Ok(user)
    }

    /// Предоставить экземпляр [`User`] по имени пользователя (`username`).
    #[instrument(skip(self), level = "debug")]
    pub(crate) async fn get_user(&self, username: &Username) -> Result<User, DomainError> {
        let username = username.to_lowercase();
        self.repo.get_by_username(&username).await
    }
}
