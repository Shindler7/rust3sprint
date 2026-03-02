//! Серверная инфраструктура.

use crate::{
    application::AppServices,
    infrastructure::config::BlogConfig,
    presentation::{api_handlers, grpc::api_services::BlogGrpcService, middleware},
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
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
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allow_any_header()
            // .supports_credentials()
            .max_age(cfg_clone.security.cors_max_age);

        if cfg_clone.security.cors_urls.iter().any(|c| c == "*") {
            cors = cors.allow_any_origin()
        } else {
            for origin in &cfg_clone.security.cors_urls {
                cors = cors.allowed_origin(origin)
            }
        }

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

    let jwt_service = Arc::new(cfg.security.jwt_service.clone());

    let service = BlogGrpcService::new(app_services, jwt_service);

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
