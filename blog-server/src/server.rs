//! Серверная инфраструктура.

use crate::{
    application::AppServices,
    infrastructure::config::BlogConfig,
    presentation::{api_handlers, grpc_service::BlogGrpcService, middleware},
};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE}, middleware::Logger, web,
    App,
    HttpResponse,
    HttpServer,
};
use anyhow::{Context, Result as AnyhowResult};
use proto_crate::proto_blog::blog_service_server::BlogServiceServer;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tonic::transport::Server;
use tracing::info;

/// Сервер `actix_web`, обслуживающий блог.
pub(crate) async fn run_blog_server(
    cfg: Arc<BlogConfig>,
    app_services: AppServices,
    mut shutdown: Receiver<bool>,
) -> AnyhowResult<()> {
    info!(
        "Запуск основного HTTP сервера... {}",
        cfg.server.server_addr()
    );

    let cfg_clone = Arc::clone(&cfg);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg_clone.security.cors_url)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
            .supports_credentials()
            .max_age(cfg_clone.security.cors_max_age);

        let default_headers = middleware::default_headers();

        App::new()
            .wrap(Logger::default())
            .wrap(default_headers)
            .wrap(cors)
            .configure(api_handlers::configure_api_routers)
            .app_data(web::Data::new(Arc::clone(&app_services.auth_service)))
            .app_data(web::Data::new(Arc::clone(&app_services.blog_service)))
            .app_data(web::Data::new(Arc::clone(&cfg_clone)))
            .default_service(web::to(|| async { HttpResponse::NotFound().finish() }))
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
    cfg: Arc<BlogConfig>,
    app_services: AppServices,
    mut shutdown: Receiver<bool>,
) -> AnyhowResult<()> {
    info!("Запуск gPRC...");

    let service = BlogGrpcService::new(app_services);

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
