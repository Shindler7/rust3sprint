//! Конфигурация сервера.

use std::ops::RangeInclusive;

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
pub(crate) const USERNAME_RANGE_LEN_CHARS: RangeInclusive<u64> = 3..=32;

/// Допустимый диапазон длины адреса электронной почты.
pub(crate) const EMAIL_RANGE_LEN_CHARS: RangeInclusive<usize> = 5..=254;

/// Минимальная длина пароля пользователя.
pub(crate) const PASSWORD_MIN_CHARS: usize = 10;

/// Максимальная длина заголовка публикации (поста).
pub(crate) const POSTS_TITLE_MAX_CHARS: usize = 100;
