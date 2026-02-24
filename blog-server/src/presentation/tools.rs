//! Общие утилиты для presentation.

use crate::{
    domain::user::{LoginUser, User},
    errors::DomainError,
    infrastructure::jwt::JwtService,
};

/// Поддерживающая функция. Получает JWT-токен для пользователя.
pub(crate) fn get_jwt_token(user: &User, jwt_service: &JwtService) -> Result<String, DomainError> {
    let user_id = user
        .id
        .as_ref()
        .ok_or_else(|| DomainError::server_err("Пользователь не имеет ID"))?;
    Ok(jwt_service.generate_token(user_id, &user.username)?)
}

/// Поддерживающая функция. Проверка корректности предоставленного пароля
/// авторизуемого пользователя.
pub(crate) fn verified_user_password(
    login_user: &LoginUser,
    password_hash: &str,
) -> Result<(), DomainError> {
    let verified_hash = login_user
        .password
        .verify_hash(password_hash)
        .map_err(|err| DomainError::invalid_credentials(format!("ошибка хеширования: {err}")))?;

    if !verified_hash {
        return Err(DomainError::invalid_password(""));
    }

    Ok(())
}
