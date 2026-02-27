//! Параметры приложения.
//!
//! Используют для формирования файл `settings.json`, в корневом каталоге
//! проекта.

use anyhow::{Context, Result as AnyhowResult};
use serde::Deserialize;
use serde_json;
use std::{fs::read_to_string, path::PathBuf};

/// Имя файла конфигурации.
const CONFIG_JSON: &str = "settings.json";

/// Предоставить ссылку к файлу конфигурации.
fn get_config_json_path() -> PathBuf {
    PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join(CONFIG_JSON)
}

/// Адреса используемых серверов.
#[derive(Debug, Deserialize)]
pub(crate) struct SettingsServers {
    /// HTTP-сервер.
    pub(crate) http_server: String,
    /// gRPC-сервер.
    pub(crate) grpc_server: String,
}

/// Различные параметры состояния приложения.
#[derive(Debug, Deserialize)]
pub(crate) struct AppState {
    /// Файл для хранения токена.
    pub(crate) token_file: PathBuf,
}

/// Конфигурация приложения.
///
/// Для инициализации конфигурации необходимо использовать метод
/// [`Settings::setup`].
#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    /// Адреса используемых серверов.
    pub(crate) servers: SettingsServers,
    pub(crate) app_state: AppState,
}

impl Settings {
    /// Инициализация конфигурации из файла.
    pub(crate) fn setup() -> AnyhowResult<Self> {
        let json_file = get_config_json_path();
        let contents = read_to_string(&json_file).with_context(|| {
            format!(
                "Файл конфигурации отсутствует или повреждён ({})",
                CONFIG_JSON
            )
        })?;

        serde_json::from_str(&contents).with_context(|| "Ошибка парсинга файла конфигурации")
    }
}
