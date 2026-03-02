//! Фронтенд приложение блога, созданное с помощью фреймворка `yem`.
//!
//! Сборка в debug режиме:
//!
//! ```sh,ignore
//! trunk serve
//! ```
//!
//! Для production:
//!
//! ```sh,ignore
//! trunk build
//! ```

mod app;
pub(crate) mod components;
pub(crate) mod pages;
mod routes;
pub mod state;
pub mod domain;
pub mod services;

use tracing_subscriber::{fmt::format::Pretty, prelude::*};
use tracing_web::{performance_layer, MakeWebConsoleWriter};

use app::Main;
use yew::Renderer;

fn main() {
    init_tracing();

    Renderer::<Main>::new().render();
}

fn init_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init()
}
