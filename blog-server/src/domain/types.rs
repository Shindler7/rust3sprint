//! Новые типы моделей для domain.

use crate::{
    domain::validators::*,
    errors::DomainError,
    settings::{ArgonConfig, ARGON_ALGORITHM, ARGON_ALGORITHM_VERSION},
    validated_newtype,
};
use anyhow::{anyhow, Result as AnyhowResult};
use argon2::{
    password_hash::{rand_core::OsRng, Error as PwdHashError, SaltString}, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Единый тип `id` для моделей. Поддерживает преобразование в `i64`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[serde(transparent)]
#[sqlx(transparent)]
pub(crate) struct DataId(pub i64);

impl From<DataId> for i64 {
    fn from(id: DataId) -> Self {
        id.0
    }
}

impl Display for DataId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

validated_newtype! {
    /// Новый тип для имени пользователя (`username`).
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
    #[serde(try_from = "String")]
    #[sqlx(transparent)]
    pub(crate) struct Username;
    validate = validate_username;
    error = DomainError::invalid_username;
}

impl Username {
    /// Создаёт новый экземпляр [`Username`], переводя значение в нижний
    /// регистр.
    pub(crate) fn to_lowercase(&self) -> Self {
        Self(self.0.to_lowercase())
    }
}

validated_newtype! {
    /// Новый тип для электронного адреса.
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
    #[serde(try_from = "String")]
    #[sqlx(transparent)]
    pub(crate) struct Email;
    validate = validate_email;
    error = DomainError::invalid_email;
}

impl Email {
    /// Создаёт новый экземпляр [`Email`], переводя значение в нижний регистр.
    pub(crate) fn to_lowercase(&self) -> Self {
        Self(self.0.to_lowercase())
    }
}

validated_newtype! {
    /// Новый тип пароля пользователя.
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
    #[serde(try_from = "String")]
    #[sqlx(transparent)]
    pub(crate) struct UserPassword;
    validate = validate_password;
    error = DomainError::invalid_password;
}

impl UserPassword {
    /// Универсальный метод, который возвращает хеш пароля. Лениво генерирует
    /// его при запросе, не изменяет пароль и не сохраняет созданный хеш.
    pub(crate) fn hash(&self) -> AnyhowResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = self.get_argon2()?;

        Ok(argon2
            .hash_password(self.as_bytes(), &salt)
            .map_err(|err| anyhow!("Неудача создания хеша пароля: {err}"))?
            .to_string())
    }

    /// Проверить совпадение хеша с имеющимся в структуре паролем.
    pub(crate) fn verify_hash(&self, hash: &str) -> AnyhowResult<bool> {
        let parsed =
            PasswordHash::new(hash).map_err(|err| anyhow!("Неверный формат хеша: {err}"))?;

        match Argon2::default().verify_password(self.as_bytes(), &parsed) {
            Ok(_) => Ok(true),
            Err(PwdHashError::Password) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    /// Предоставить пароль в формате байт-строки.
    fn as_bytes(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }

    /// Предоставить экземпляр [`Argon2`] на основе параметров, установленных
    /// для приложения.
    fn get_argon2(&self) -> AnyhowResult<Argon2<'_>> {
        let cfg = ArgonConfig::default();
        let params = Params::new(cfg.m_cost, cfg.t_cost, cfg.p_cost, Some(cfg.output_len))
            .map_err(|err| anyhow!("Некорректные параметры Argon2: {err}"))?;

        Ok(Argon2::new(
            ARGON_ALGORITHM,
            ARGON_ALGORITHM_VERSION,
            params,
        ))
    }
}

validated_newtype! {
    /// Новый тип для заголовка публикации.
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
    #[serde(try_from = "String")]
    #[sqlx(transparent)]
    pub(crate) struct PostTitle;
    validate = validate_title;
    error = DomainError::invalid_post;
}

validated_newtype! {
    /// Новый тип для заголовка публикации.
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
    #[serde(try_from = "String")]
    #[sqlx(transparent)]
    pub(crate) struct PostContent;
    validate = validate_content;
    error = DomainError::invalid_post;
}
