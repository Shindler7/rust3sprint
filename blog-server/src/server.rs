//! Серверная инфраструктура.

use crate::{infrastructure::config::BlogConfig, presentation::grpc_service::BlogGrpcService};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE}, middleware::Logger,
    web,
    App,
    HttpServer,
};
use anyhow::{Context, Result as AnyhowResult};
use proto_crate::proto_blog::blog_service_server::BlogServiceServer;
use sqlx::PgPool;
use tokio::sync::broadcast::Receiver;
use tonic::transport::Server;
use tracing::info;

/// Сервер `actix_web`, обслуживающий блог.
pub(crate) async fn run_blog_server(
    cfg: BlogConfig,
    pool: PgPool,
    mut shutdown: Receiver<bool>,
) -> AnyhowResult<()> {
    info!("Запуск основного HTTP сервера...");

    let server = HttpServer::new(move || {
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
    .run();

    let handle = server.handle();

    tokio::select! {
        r = server => { r.with_context(|| "HTTP сервер остановился с ошибкой")?; }
        _ = shutdown.recv() => {
            info!("HTTP сервер получил команду shutdown");
            handle.stop(true).await
        }
    }

    info!("Сервер остановлен");

    Ok(())
}

/// Сервер `tonik`, обслуживающий механизмы `gRPC`.
pub(crate) async fn run_blog_grpc(
    cfg: BlogConfig,
    pool: PgPool,
    mut shutdown: Receiver<bool>,
) -> AnyhowResult<()> {
    info!("Запуск gPRC...");

    let service = BlogGrpcService::new(pool);

    Server::builder()
        .add_service(BlogServiceServer::new(service))
        .serve_with_shutdown(cfg.server.grpc_addr(), async move {
            let _ = shutdown.recv().await;
            info!("gRPC сервер получил команду shutdown");
        })
        .await
        .with_context(|| "сервер gRPC остановился с ошибкой")?;

    info!("Сервер gRPC остановлен");

    Ok(())
}
