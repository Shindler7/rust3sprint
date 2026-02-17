//! Серверная инфраструктура блога.
//!
//! Обеспечивает работу `actix_web`, механики `gRPC` и взаимодействие с базой
//! данных.

use crate::settings::SLEEP_BEFORE_SHUTDOWN_MS;
use crate::{
    infrastructure::{
        config::BlogConfig,
        database::{get_pool_postgres, migrations},
        logging::init_logging,
    },
    server::{run_blog_grpc, run_blog_server},
    settings::ENV_HELP,
};
use actix_web::rt::{spawn, time};
use anyhow::{Context, Result as AnyhowResult};
use dotenvy::dotenv;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::info;

mod application;
mod data;
mod domain;
mod handlers;
mod infrastructure;
mod presentation;
mod server;
mod settings;

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    dotenv().with_context(|| format!("Ошибка загрузки env!\n{}", ENV_HELP))?;
    init_logging()?;

    info!("Настройка серверной инфраструктуры перед запуском...");

    let cfg = BlogConfig::load()?;

    // Соединение с БД и осуществление миграций.
    let pool = get_pool_postgres(&cfg.db).await?;
    info!("Успешное подключение к базе данных");
    migrations(&pool).await?;

    // Запуск серверов.
    let tx = broadcast::channel::<bool>(1).0;

    let http = spawn(run_blog_server(cfg.clone(), pool.clone(), tx.subscribe()));
    let grpc = spawn(run_blog_grpc(cfg, pool, tx.subscribe()));

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
