//! Общие утилиты для presentation.

use crate::{
    domain::user::{LoginUser, UserDto},
    errors::DomainError,
    infrastructure::jwt::JwtService,
    settings::{POSTS_LIMIT_RANGE, POSTS_OFFSET_MAX}
};

/// Поддерживающая функция. Получает JWT-токен для пользователя.
pub(crate) fn get_jwt_token(user: &UserDto, jwt_service: &JwtService) -> Result<String, DomainError> {
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

/// Проверить валидность значений, применяемых для выгрузки списка публикаций.
/// Например, `limit` и `offset` в `QueryPosts`.
pub(crate) fn validate_list_params(limit: i32, offset: i32) -> Result<(), DomainError> {
    if !POSTS_LIMIT_RANGE.contains(&limit) {
        return Err(DomainError::api_error(format!(
            "значение `limit` должно быть больше {}, но менее {}",
            POSTS_LIMIT_RANGE.start(),
            POSTS_LIMIT_RANGE.end()
        )));
    }

    if offset > POSTS_OFFSET_MAX {
        return Err(DomainError::api_error(format!(
            "допускаемое значение 'offset' не более {}",
            POSTS_OFFSET_MAX
        )));
    }
    
    Ok(())
}
