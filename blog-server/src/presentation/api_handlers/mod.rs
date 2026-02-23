//! Роутеры для внешнего взаимодействия HTTP-сервера.

use crate::presentation::middleware;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub(crate) mod protected;
pub(crate) mod public;
mod tools;

const API_ENDPOINT: &str = "/api";

/// Конфигуратор структуры роутеров `API`.
///
/// Позволяет разгрузить инициализацию приложения при запуске сервера,
/// и всю ответственность за структуру api возложить на конфигуратор.
pub(crate) fn configure_api_routers(cfg: &mut web::ServiceConfig) {
    let jwt_auth = HttpAuthentication::bearer(middleware::jwt_validator);

    cfg.service(
        web::scope(API_ENDPOINT)
            .service(web::scope("/auth").configure(public::configure_auth_routes))
            .configure(public::configure_list_routes)
            .service(
                web::scope("")
                    .wrap(jwt_auth)
                    .configure(protected::configure_posts_routes),
            ),
    );
}
