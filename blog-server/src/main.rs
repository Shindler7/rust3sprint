//! Серверная инфраструктура блога.
//!
//! Обеспечивает работу `actix_web`, механики `gRPC` и взаимодействие с базой
//! данных.

use crate::{
    application::AppServices,
    infrastructure::{config::BlogConfig, database::get_pool_postgres, logging::init_logging},
    server::{run_blog_grpc, run_blog_server},
    settings::{ENV_HELP, SLEEP_BEFORE_SHUTDOWN_MS},
};
use actix_web::rt::{spawn, time};
use anyhow::{Context, Result as AnyhowResult};
use dotenvy::dotenv;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tracing::info;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;
mod settings;

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    dotenv().with_context(|| format!("Ошибка загрузки env!\n{}", ENV_HELP))?;
    init_logging()?;

    info!("Настройка серверной инфраструктуры перед запуском...");

    let cfg = Arc::new(BlogConfig::load()?);

    // Соединение с БД и осуществление миграций.
    let pool = get_pool_postgres(&cfg.db, true).await?;
    info!("Успешное подключение к базе данных");
    let app_services = AppServices::new(&pool);

    // Запуск серверов.
    let tx = broadcast::channel::<bool>(1).0;

    let http = spawn(run_blog_server(
        Arc::clone(&cfg),
        app_services.clone(),
        tx.subscribe(),
    ));
    let grpc = spawn(run_blog_grpc(cfg, app_services, tx.subscribe()));

    tokio::signal::ctrl_c()
        .await
        .with_context(|| "ошибка перехвата Ctrl-C")?;
    info!("Применено Ctrl-C");
    let _ = tx.send(true);

    // Пауза для завершения задач обработки данных.
    time::sleep(Duration::from_millis(SLEEP_BEFORE_SHUTDOWN_MS)).await;

    http.await
        .with_context(|| "ошибка в асинхронном цикле сервера HTTP")?
        .with_context(|| "выход из цикла событий сервера HTTP")?;
    grpc.await
        .with_context(|| "ошибка в асинхронном цикле сервера gRPC")?
        .with_context(|| "выход из цикла событий сервера gRPC")?;

    info!("Все серверы остановлены");
    Ok(())
}
