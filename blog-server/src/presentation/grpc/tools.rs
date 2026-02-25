//! Вспомогательные методы для модуля gRPC.

use crate::{domain::user::AuthenticatedUser, infrastructure::jwt::JwtService};
use std::sync::Arc;
use tonic::{
    metadata::{Ascii, MetadataMap, MetadataValue},
    Status,
};
use tracing::error;

/// Предоставить [`AuthenticatedUser`], если предоставленный токен валиден.
pub(super) fn get_auth_user(
    metadata: &MetadataMap,
    jwt_service: Arc<JwtService>,
) -> Result<AuthenticatedUser, Status> {
    let token = get_token(metadata.get("authorization"))?;

    let claim = jwt_service.verify_token(&token).map_err(|err| {
        error!(
            error=%err,
            "ошибка верификации токена"
        );
        Status::unauthenticated("Токен недействительный")
    })?;

    let auth_user: AuthenticatedUser = claim.into();

    Ok(auth_user)
}

/// Предоставить нормализованный токен из метаданных запроса.
fn get_token(token: Option<&MetadataValue<Ascii>>) -> Result<String, Status> {
    let raw = token
        .ok_or_else(|| Status::unauthenticated("Отсутствует токен авторизации"))?
        .to_str()
        .map_err(|err| {
            Status::unauthenticated(format!("Токен содержит некорректные символы: {err}"))
        })?;

    raw.strip_prefix("Bearer ")
        .map(|t| t.to_string())
        .ok_or_else(|| Status::unauthenticated("Отсутствует Bearer‑токен"))
}
