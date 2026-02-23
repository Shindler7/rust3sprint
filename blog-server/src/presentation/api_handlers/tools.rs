//! Поддерживающие утилиты и инструменты для роутеров.

use crate::{
    domain::{error::DomainError, user::User},
    infrastructure::jwt::JwtService,
    settings::{POSTS_LIMIT_RANGE, POSTS_OFFSET_MAX},
};

/// Поддерживающая функция. Получает JWT-токен для пользователя.
pub(super) fn get_jwt_token(user: &User, jwt_service: &JwtService) -> Result<String, DomainError> {
    let user_id = user
        .id
        .as_ref()
        .ok_or_else(|| DomainError::server_err("Пользователь не имеет ID"))?;
    Ok(jwt_service.generate_token(user_id, &user.username)?)
}

/// Быстрая проверка валидности значений `limit` и `offset` в query-параметрах
/// для выгрузки списка публикаций и преобразование значений в ожидаемые.
///
/// Например, `u32` будут преобразованы в `i32`, поддерживаемые `PostgresSQL`.  
pub(super) fn valid_query_posts_params(
    limit: &u32,
    offset: &u32,
) -> Result<(i32, i32), DomainError> {
    if !POSTS_LIMIT_RANGE.contains(limit) {
        return Err(DomainError::api_error(format!(
            "значение `limit` должно быть больше {}, но менее {}",
            POSTS_LIMIT_RANGE.start(),
            POSTS_LIMIT_RANGE.end()
        )));
    }

    if *offset > POSTS_OFFSET_MAX {
        return Err(DomainError::api_error(format!(
            "допускаемое значение 'offset' не более {}",
            POSTS_OFFSET_MAX
        )));
    }

    /// Вспомогательная функция для конвертации u32 в i32 с проверкой.
    fn to_i32(value: u32, param_name: &str) -> Result<i32, DomainError> {
        value.try_into().map_err(|_| {
            DomainError::api_error(format!(
                "некорректное значение '{}': {}. Может быть в пределах [0, {}], но в API максимально допустимое = {}",
                param_name,
                value,
                i32::MAX,
                POSTS_OFFSET_MAX
            ))
        })
    }

    let limit_i32 = to_i32(*limit, "limit")?;
    let offset_i32 = to_i32(*offset, "offset")?;

    Ok((limit_i32, offset_i32))
}
