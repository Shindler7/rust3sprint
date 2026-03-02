//! Новые типы для приложения.

use crate::domain::errors::BlogWasmError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use yew::Properties;

/// Тип для id публикаций (post_id).
#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub(crate) struct PostId {
    /// Id публикации (поста).
    pub(crate) id: u64,
}

impl Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<u64> for PostId {
    fn from(id: u64) -> Self {
        Self { id }
    }
}

impl FromStr for PostId {
    type Err = BlogWasmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = u64::from_str(s).map_err(|_| {
            BlogWasmError::user_data_err(format!("Ожидается число, получено '{}'", s))
        })?;
        Ok(Self { id })
    }
}

/// `PostId` -> `i64` с проверкой переполнения.
impl TryFrom<PostId> for i64 {
    type Error = BlogWasmError;

    fn try_from(value: PostId) -> Result<Self, Self::Error> {
        i64::try_from(value.id).map_err(|_| {
            BlogWasmError::user_data_err(format!(
                "Слишком большой значение для id публикации: {}",
                value
            ))
        })
    }
}

/// Тип для хранения токена.
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct JwtToken(String);

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for JwtToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl JwtToken {
    /// Создать экземпляр типа с токеном.
    pub(crate) fn new(t: impl Into<String>) -> Self {
        Self(t.into())
    }

    /// Предоставить токен как Bearer.
    pub(crate) fn bearer(&self) -> String {
        format!("Bearer {}", self)
    }
}
