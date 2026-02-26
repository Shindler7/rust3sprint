//! Поддерживающие методы для транспортных клиентов.

use crate::BlogClientError;
use reqwest::Url;

/// Собрать ссылку из компонентов пути. Обёртка для метода [`Url`].
///
/// ## Errors
///
/// Если на любом из этапов формирования ссылки возникнет проблема, то вернётся
/// ошибка [`BlogClientError::ClientError`].
pub(crate) fn compile_url(base: Url, endpoint: Vec<&str>) -> Result<Url, BlogClientError> {
    base.join(&endpoint.join("/"))
        .map_err(|err| BlogClientError::client_error(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_base_url() -> Url {
        Url::parse("https://yandex.ru").unwrap()
    }

    #[test]
    fn compile_url_one_endpoint() {
        let endpoint = vec!["one"];
        let res = compile_url(valid_base_url(), endpoint);
        assert!(res.is_ok());
    }
}
