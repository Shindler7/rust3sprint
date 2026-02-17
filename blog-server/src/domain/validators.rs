//! Валидаторы для различных ситуаций.
use crate::settings::{EMAIL_RANGE_LEN_CHARS, PASSWORD_MIN_CHARS};

/// Универсальный валидатор проекта для паролей.
pub(super) fn validate_password(pass: &str) -> Result<(), String> {
    if pass.len() < PASSWORD_MIN_CHARS {
        return Err(format!(
            "Пароль должен быть более {} символов",
            PASSWORD_MIN_CHARS
        ));
    }

    let has_upper = pass.chars().any(|c| c.is_uppercase());
    let has_lower = pass.chars().any(|c| c.is_lowercase());
    let has_special = pass.chars().any(|c| c.is_alphanumeric());

    if !has_upper || !has_lower {
        return Err("пароль должен содержать буквы разного регистра".to_string());
    }

    if !has_special {
        return Err("пароль должен содержать хотя бы один спецсимвол".to_string());
    }

    Ok(())
}

/// Универсальный валидатор предоставленного адреса электронной почты.
pub(super) fn validate_email(email: &str) -> Result<(), String> {
    let at_count = email.matches('@').count();
    if at_count == 0 {
        return Err("отсутствует символ '@'".to_string());
    } else if at_count > 1 {
        return Err("адрес содержит более одного символа '@'".to_string());
    }

    if !EMAIL_RANGE_LEN_CHARS.contains(&email.len()) {
        return Err(format!(
            "допустимая длина email от {} до {} символов",
            EMAIL_RANGE_LEN_CHARS.start(),
            EMAIL_RANGE_LEN_CHARS.end()
        ));
    }

    let parts: Vec<&str> = email.split('@').collect();
    let local = parts[0].trim();
    let domain = parts[1].trim();

    if local.is_empty() || domain.is_empty() {
        return Err("некорректная структура адреса".to_string());
    }

    if !domain.contains('.') {
        return Err("в доменной части должен быть символ '.'".to_string());
    }

    Ok(())
}
