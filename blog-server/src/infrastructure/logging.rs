//! Настройка серверного логирования.

use anyhow::{Context, Result as AnyhowResult};
use tracing_subscriber::{
    fmt::time::ChronoUtc, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

/// Инициализация логирования с использованием `tracing_subscriber`.
///
/// Ожидается наличие в `env`-окружении параметра `RUST_LOG`, и вызывает
/// ошибку при проблеме доступа к этим данным.
/// Обработка "по умолчанию" исключена, чтобы избежать неочевидного поведения.
pub fn init_logging() -> AnyhowResult<()> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .with_context(|| "Ошибка чтения 'env' или отсутствует параметр 'RUST_LOG'")?,
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(ChronoUtc::rfc_3339()),
        )
        .init();

    tracing::info!("Логирование инициализировано");

    Ok(())
}
