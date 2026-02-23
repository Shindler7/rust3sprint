//! Middleware обработка JWT-токенов.

use crate::{domain::user::AuthenticatedUser, infrastructure::config::BlogConfig};
use actix_web::{dev::ServiceRequest, web, Error as ActixError, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;
use tracing::error;

/// Проверить валидность предоставленного JWT-токена.
///
/// В случае успеха пользователь авторизуется, возвращается экземпляр
/// [`AuthenticatedUser`] с его данными, сформированными на основе информации
/// из токена.
pub(crate) async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let jwt_service = match req.app_data::<web::Data<Arc<BlogConfig>>>() {
        Some(config) => config.security.jwt_service.clone(),
        None => {
            let error = actix_web::error::ErrorInternalServerError("ошибка конфигурации сервера");
            error!(
                error = %error,
                "Ошибка доступа к конфигурации, jwt_service недоступен"
            );
            return Err((error, req));
        }
    };

    let claim = match jwt_service.verify_token(credentials.token()) {
        Ok(claim) => claim,
        Err(err) => {
            error!(
                error = %err,
                "попытка доступа с недействительным токеном"
            );
            let error = actix_web::error::ErrorUnauthorized("токен недействительный");
            return Err((error, req));
        }
    };

    let auth_user: AuthenticatedUser = claim.into();

    req.extensions_mut().insert(auth_user);
    Ok(req)
}
