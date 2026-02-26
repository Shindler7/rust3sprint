//! Поддерживающие утилиты инструменты приложения.

use crate::BlogClientError;
use reqwest::Url;

/// Преобразовать строковое представление ссылки в [`Url`].
pub(super) fn str_to_url(str_url: &str) -> Result<Url, BlogClientError> {
    let url = Url::parse(str_url).map_err(|err| BlogClientError::invalid_url(err.to_string()))?;

    Ok(url)
}
