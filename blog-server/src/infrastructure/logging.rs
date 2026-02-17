//! Настройка серверного логирования.

use anyhow::{anyhow, Result as AnyhowResult};
use tracing_subscriber::{fmt::time::ChronoUtc, EnvFilter};

/// Инициализация логирования с использованием `tracing_subscriber`.
///
/// Ожидается наличие в `env`-окружении параметра `RUST_LOG`, и вызывает
/// ошибку при проблеме доступа к этим данным.
/// Обработка "по умолчанию" исключена, чтобы избежать неочевидного поведения.
pub(crate) fn init_logging() -> AnyhowResult<()> {
    use tracing_subscriber::fmt::Subscriber;

    Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_timer(ChronoUtc::rfc_3339())
        .try_init()
        .map_err(|err| anyhow!("Ошибка инициализации логирования: {err}"))?;

    tracing::info!("Логирование инициализировано");

    Ok(())
}
