//! Библиотека для взаимодействия с сервером через командную строку.

mod cli;
mod config;

use anyhow::{Context, Result as AnyhowResult};
use blog_client::{BlogClient, Transport};
use cli::{Commands, read_args};
use config::Settings;
use std::path::PathBuf;
use tokio::fs;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let config = Settings::setup()?;
    let cli_args = read_args()?;

    let transport = if cli_args.grpc {
        Transport::grpc(&config.servers.grpc_server)?
    } else {
        Transport::http(&config.servers.http_server)?
    };

    let server = BlogClient::new(transport)
        .await
        .with_context(|| "Ошибка инициализации сервиса клиент-сервер")?;

    let token_from_file = read_token(&config.app_state.token_file).await?;

    Ok(())
}

/// Загрузить токен из файла.
async fn read_token(token_file: &PathBuf) -> AnyhowResult<Option<String>> {
    let content = match fs::read_to_string(token_file).await {
        Ok(content) => content,
        Err(_) => {
            println!("Файл с токеном отсутствует");
            return Ok(None);
        }
    };

    Ok(Some(content))
}

/// Сохранить токен в файл.
fn save_token(file: PathBuf, token: String) -> AnyhowResult<()> {
    todo!()
}
