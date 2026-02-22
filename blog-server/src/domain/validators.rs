//! Валидаторы для различных ситуаций.
use crate::settings::{
    EMAIL_RANGE_LEN_CHARS, PASSWORD_MIN_CHARS, PASSWORD_VALID_SPECIAL_CHARS,
    USERNAME_RANGE_LEN_CHARS,
};

/// Является ли переданный `char` допустимым для пароля специальным символом.
fn is_special_char(c: char) -> bool {
    PASSWORD_VALID_SPECIAL_CHARS.contains(&c)
}

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
    let has_special = pass.chars().any(is_special_char);

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

/// Проверяет, содержит ли только допустимые символы: латинские буквы, цифры
/// и нижнее подчёркивание (`_`).
fn contains_only_allowed_chars(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_')
}

/// Проверяет, является ли первый символ латинской буквой.
fn starts_with_ascii_alphabetic(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
}

/// Универсальный валидатор имени пользователя (`username`), обеспечивающий
/// унифицированный формат для приложения.
pub(super) fn validate_username(username: &str) -> Result<(), String> {
    if !USERNAME_RANGE_LEN_CHARS.contains(&username.len()) {
        return Err(format!(
            "допускается длина имени от {} до {} символов",
            USERNAME_RANGE_LEN_CHARS.start(),
            USERNAME_RANGE_LEN_CHARS.end()
        ));
    }

    if !contains_only_allowed_chars(username) {
        return Err("имя может содержать латинские буквы, _, цифры".to_string());
    }

    if !starts_with_ascii_alphabetic(username) {
        return Err("первым символом должна быть латинская буква".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{
        EMAIL_RANGE_LEN_CHARS, PASSWORD_MIN_CHARS, PASSWORD_VALID_SPECIAL_CHARS,
        USERNAME_RANGE_LEN_CHARS,
    };

    fn email_with_len(len: usize) -> String {
        // Структура: <local>@b.c => длина = local_len + 4
        assert!(len >= 5);
        let local_len = len - 4;
        format!("{}@b.c", "a".repeat(local_len))
    }

    #[test]
    fn validate_password_too_short() {
        let pass = "A".repeat((PASSWORD_MIN_CHARS / 2).max(1));
        let err = validate_password(&pass).unwrap_err();
        assert!(err.contains(&PASSWORD_MIN_CHARS.to_string()));
    }

    #[test]
    fn validate_password_missing_case() {
        let pass = format!("allower{}!", "a".repeat(PASSWORD_MIN_CHARS));
        let err = validate_password(&pass).unwrap_err();
        assert_eq!(err, "пароль должен содержать буквы разного регистра");
    }

    #[test]
    fn validate_password_missing_special() {
        let pass = format!("Aa{}", "a".repeat(PASSWORD_MIN_CHARS));
        let err = validate_password(&pass).unwrap_err();
        assert_eq!(err, "пароль должен содержать хотя бы один спецсимвол");
    }

    #[test]
    fn validate_password_ok() {
        let special = *PASSWORD_VALID_SPECIAL_CHARS.first().unwrap();
        let pass = format!("Aa{}{}", "a".repeat(PASSWORD_MIN_CHARS), special);
        assert!(validate_password(&pass).is_ok());
    }

    #[test]
    fn validate_email_no_at() {
        let err = validate_email("foo.example.com").unwrap_err();
        assert_eq!(err, "отсутствует символ '@'");
    }

    #[test]
    fn validate_email_multiple_at() {
        let err = validate_email("a@b@c.com").unwrap_err();
        assert_eq!(err, "адрес содержит более одного символа '@'");
    }

    #[test]
    fn validate_email_bad_len() {
        let max = *EMAIL_RANGE_LEN_CHARS.end();
        let email = email_with_len(max + 1);
        let err = validate_email(&email).unwrap_err();
        assert!(err.contains("допустимая длина email"));
    }

    #[test]
    fn validate_email_bad_structure() {
        let err = validate_email("@example.com").unwrap_err();
        assert_eq!(err, "некорректная структура адреса");
    }

    #[test]
    fn validate_email_missing_dot_in_domain() {
        let err = validate_email("user@examplecom").unwrap_err();
        assert_eq!(err, "в доменной части должен быть символ '.'");
    }

    #[test]
    fn validate_email_ok() {
        assert!(validate_email("user@example.com").is_ok());
    }

    #[test]
    fn validate_username_bad_len() {
        let min = *USERNAME_RANGE_LEN_CHARS.start();
        let username = "a".repeat(min.saturating_sub(1));
        let err = validate_username(&username).unwrap_err();
        assert!(err.contains("допускается длина имени"));
    }

    #[test]
    fn validate_username_bad_chars() {
        let min = *USERNAME_RANGE_LEN_CHARS.start();
        let username = format!("a{}-", "a".repeat(min.saturating_sub(1)));
        let err = validate_username(&username).unwrap_err();
        assert_eq!(err, "имя может содержать латинские буквы, _, цифры");
    }

    #[test]
    fn validate_username_bad_start() {
        let min = *USERNAME_RANGE_LEN_CHARS.start();
        let username = format!("1{}", "a".repeat(min.saturating_sub(1)));
        let err = validate_username(&username).unwrap_err();
        assert_eq!(err, "первым символом должна быть латинская буква");
    }

    #[test]
    fn validate_username_ok() {
        let min = *USERNAME_RANGE_LEN_CHARS.start();
        let username = format!("a{}", "b".repeat(min.saturating_sub(1)));
        assert!(validate_username(&username).is_ok());
    }
}
