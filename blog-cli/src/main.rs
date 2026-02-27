//! Библиотека для взаимодействия с сервером через командную строку.

mod cli;
pub(crate) mod client;
mod config;

use anyhow::{Context, Result as AnyhowResult};
use blog_client::{BlogClient, Transport};
use cli::read_args;
use config::Settings;
use crate::client::request::execute_request;

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
