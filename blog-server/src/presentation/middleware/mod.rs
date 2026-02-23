//! Промежуточное программное обеспечение.

mod jwt;

use crate::settings::APP_HEADERS;
use actix_web::middleware::DefaultHeaders;
pub(crate) use jwt::jwt_validator;

/// Стандартный генератор заголовка запроса (`headers`) для приложения.
pub(crate) fn default_headers() -> DefaultHeaders {
    let app_version = env!("CARGO_PKG_VERSION");
    DefaultHeaders::new()
        .add((APP_HEADERS, app_version))
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("Referrer-Policy", "no-referrer"))
        .add(("Permissions-Policy", "geolocation=()"))
        .add(("Cross-Origin-Opener-Policy", "same-origin"))
        .add_content_type()
}
