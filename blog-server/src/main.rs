use crate::server::run_blog_grpc;
use crate::{
    infrastructure::{
        config::BlogConfig,
        database::{get_pool_postgres, migrations},
        logging::init_logging,
    },
    server::run_blog_server,
    settings::ENV_HELP,
};
use actix_web::rt::spawn;
use anyhow::{Context, Result as AnyhowResult};
use dotenvy::dotenv;
use tracing::{error, info};

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

    tracing::info!("Настройка серверной инфраструктуры перед запуском...");

    let cfg = BlogConfig::load()?;

    // Соединение с БД и осуществление миграций.
    let pool = get_pool_postgres(&cfg.db).await?;
    info!("Успешное подключение к базе данных");
    migrations(&pool).await?;

    tokio::try_join!(
        run_blog_server(cfg.clone(), pool.clone()),
        run_blog_grpc(cfg),
    )?;

    info!("Все серверы остановлены");
    Ok(())
}
