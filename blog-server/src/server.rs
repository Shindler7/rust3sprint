//! Серверная инфраструктура.

use crate::infrastructure::config::BlogConfig;
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE}, middleware::Logger,
    web,
    App,
    HttpServer,
};
use anyhow::{Context, Result as AnyhowResult};
use sqlx::PgPool;
use tonic::transport::Server;
use tracing::info;

pub mod proto_blog {
    tonic::include_proto!("blog");
}

/// Сервер `actix_web`, обслуживающий блог.
pub async fn run_blog_server(cfg: BlogConfig, pool: PgPool) -> AnyhowResult<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.security.cors_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
            .supports_credentials()
            .max_age(cfg.security.cors_max_age);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(cfg.server.server_addr())?
    .run()
    .await
    .with_context(|| "HTTP сервер остановился с ошибкой")?;

    info!("Сервер остановлен");

    Ok(())
}

/// Сервер `tonik`, обслуживающий механизмы `grpc`.
pub async fn run_blog_grpc(cfg: BlogConfig) -> AnyhowResult<()> {
    todo!("Запуск grpc-сервера.")
    // Server::builder()
    //     .add_service(proto_blog)
    //     .serve(cfg.server.grpc_addr())
    //     .await?;
    //
    // Ok(())
}
