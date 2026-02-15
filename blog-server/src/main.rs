use crate::{
    infrastructure::{
        config::BlogConfig,
        database::{get_pool_postgres, migrations},
        logging::init_logging,
    },
    settings::ENV_HELP,
};
use anyhow::{Context, Result as AnyhowResult};
use dotenvy::dotenv;

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

    let cfg = BlogConfig::load()?;

    // Соединение с БД и осуществление миграций.
    let pool = get_pool_postgres(&cfg.db).await?;
    migrations(&pool).await?;

    Ok(())
}
