//! Конфигурация сервера.

use argon2::{Algorithm, Version};
use chrono::Duration;
use std::ops::RangeInclusive;

/// Типизированный заголовок для ответов сервера (`headers`).
pub(crate) const APP_HEADERS: &str = "App_version";

/// Время задержки после сигнала "стоп" серверам до окончательного останова
/// (миллисекунды).
pub(crate) const SLEEP_BEFORE_SHUTDOWN_MS: u64 = 100;

/// Подсказка при ошибке доступа к .env-файлу.
pub(crate) const ENV_HELP: &str = r#"
Файл `.env` должен располагаться в корне приложения (проекта).

Примерная структура:
# Host
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# CORS.
CORS_URL=http://localhost:3000
CORS_MAX_AGE=600

# gRPC
GRPC_PORT=50051

# Database.
DB_USERNAME=postgres
DB_PASSWORD=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_NAME=practice_db
DB_MAX_CONN=10

# Logging
RUST_LOG=blog-server=debug,actix_web=info
"#;

/// Шаблон ссылки для подключения к базе данных.
pub(crate) const DB_URL_TEMPLATE: &str = "postgres://{user}:{password}@{host}:{port}/{db}";

/// Максимальное количество соединений, поддерживаемых пулом.
pub(crate) const DB_MAX_CONN: u32 = 10;

/// Допустимый диапазон длины имени пользователя (username).
pub(crate) const USERNAME_RANGE_LEN_CHARS: RangeInclusive<usize> = 3..=32;

/// Допустимый диапазон длины адреса электронной почты.
pub(crate) const EMAIL_RANGE_LEN_CHARS: RangeInclusive<usize> = 5..=254;

/// Минимальная длина пароля пользователя.
pub(crate) const PASSWORD_MIN_CHARS: usize = 10;

/// Допустимые специальные символы для паролей.
pub(crate) const PASSWORD_VALID_SPECIAL_CHARS: [char; 9] =
    ['!', '@', '#', '$', '%', '^', '&', '*', '_'];

/// Конфигурация формирования хешей с использованием [`argon2`].
pub(crate) struct ArgonConfig {
    /// Память в кб.
    pub(crate) m_cost: u32,
    /// Количество итераций.
    pub(crate) t_cost: u32,
    /// Степень параллелизма.
    pub(crate) p_cost: u32,
    /// Длине хеша.
    pub(crate) output_len: usize,
}

impl Default for ArgonConfig {
    fn default() -> Self {
        Self {
            m_cost: 65536,
            t_cost: 3,
            p_cost: 4,
            output_len: 32,
        }
    }
}

/// Применяемый алгоритм хеширования [`argon2`].
pub(crate) const ARGON_ALGORITHM: Algorithm = Algorithm::Argon2id;

/// Версия алгоритма хеширования [`argon2`].
pub(crate) const ARGON_ALGORITHM_VERSION: Version = Version::V0x13;

/// Максимальный срок жизни токена JWT.
pub(crate) const JWT_LIFETIME: Duration = Duration::hours(24);

/// Максимальная длина заголовка публикации (поста).
pub(crate) const POSTS_TITLE_MAX_CHARS: usize = 100;

/// Границы выгрузки публикаций через API.
pub(crate) const POSTS_LIMIT_RANGE: RangeInclusive<u32> = 1..=100;

/// Максимальное значение `offset` при выгрузке публикаций через API.
pub(crate) const POSTS_OFFSET_MAX: u32 = 1000;
