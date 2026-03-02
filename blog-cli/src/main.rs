//! Консольное приложение для взаимодействия с сервером через командную строку.
//!
//! Самостоятельно сетевые запросы не осуществляет, взаимодействует с внешними
//! сервисами через библиотеку `blog-client`.
//!
//! Предоставляет команды для взаимодействия с API сервисами, сервируемыми
//! серверами (например, `http` и `gRPC`).
//!
//! ## Примеры
//!
//! Получить справку о доступных сервисах:
//!
//! ```sh,ignore
//! blog-cli --help
//! ```
//!
//! Информация о конкретном сервисе:
//!
//! ```sh,ignore
//! blog-cli create --help
//! ```
//!
//! Пример запроса:
//!
//! ```sh,ignore
//! blog-cli register -u jenny -e flower@mail.no -p secret@12345
//! ```
//!
//! (Не используйте такие простые варианты пароля, как приведённый в примере)
//!
//! ## Аутентификация
//!
//! При успешной авторизации, JWT-токен сохраняется по умолчанию в файле
//! `.blog_token`, расположенному в корне проекта, в открытом виде. Оттуда
//! он автоматически подставляется в запросы, требующие подтверждённый доступ.
//!
//! Место расположения `.blog_token` можно изменить в настройках.
//!
//! ## Конфигурация
//!
//! Настройки приложения сервируются через `settings.json` в корне проекта.
//! В том числе адреса серверов, место расположения файла хранения JWT-токена.
//! Название файла настроек и место его расположения можно изменить в модуле
//! `config.rs`.
//!
//! ## Ошибки
//!
//! Собственной структуры ошибок приложение не предоставляет. Использует
//! универсальный механизм `Anyhow`, а для взаимодействия с сервисами API
//! набор ошибок, предоставляемых `blog-client`.

mod cli;
pub(crate) mod client;
mod config;

use crate::client::request::execute_request;
use anyhow::{Context, Result as AnyhowResult};
use blog_client::{BlogClient, Transport};
use cli::read_args;
use config::Settings;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let config = Settings::init()?;
    let cli_args = read_args()?;

    let transport = if cli_args.grpc {
        Transport::grpc(&config.servers.grpc_server)?
    } else {
        Transport::http(&config.servers.http_server)?
    };

    let mut server = BlogClient::new(transport)
        .await
        .with_context(|| "Ошибка инициализации сервиса клиент-сервер")?;

    execute_request(&mut server, &cli_args.command, &config).await?;

    Ok(())
}
