//! Клиентский модуль взаимодействия с сервером.
//!
//! Это библиотека, которая используется CLI и WASM-фронтендом.

pub mod clients;
mod config;
pub mod error;
mod models;

pub use clients::{BlogClient, Transport};
pub use error::BlogClientError;
