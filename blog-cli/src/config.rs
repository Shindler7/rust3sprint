//! Параметры приложения.
//!
//! Используют для формирования файл `settings.json`, в корневом каталоге
//! проекта.

use anyhow::{anyhow, Context, Result as AnyhowResult};
use serde::Deserialize;
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
    /// Имя файла для хранения токена.
    token_file: PathBuf,

    /// Путь к файлу токена. Если `None`, то используется каталог проекта.
    token_path: Option<PathBuf>,

    /// Полный путь к файлу хранения токена. Формируется автоматически.
    #[serde(skip)]
    pub(crate) token_full_path: PathBuf,
}

/// Конфигурация приложения.
///
/// Для инициализации конфигурации необходимо использовать метод
/// [`Settings::init`].
#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    /// Адреса используемых серверов.
    pub(crate) servers: SettingsServers,
    pub(crate) app_state: AppState,
}

impl Settings {
    /// Инициализация конфигурации из файла.
    pub(crate) fn init() -> AnyhowResult<Self> {
        let json_file = get_config_json_path();
        let contents = read_to_string(&json_file).with_context(|| {
            format!(
                "Файл конфигурации отсутствует или повреждён ({})",
                CONFIG_JSON
            )
        })?;

        let mut settings: Settings = serde_json::from_str(&contents)
            .map_err(|err| anyhow!("Ошибка парсинга файла конфигурации: {err}"))?;

        settings.setup();

        Ok(settings)
    }

    /// Первичная настройка параметров конфигурации.
    fn setup(&mut self) {
        let base = self
            .app_state
            .token_path
            .clone()
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));

        self.app_state.token_full_path = base.join(&self.app_state.token_file);
    }
}
