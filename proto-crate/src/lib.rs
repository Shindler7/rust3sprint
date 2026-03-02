//! Доступ к методам `proto_blog`.

/// Сгенерированные типы и gRPC‑клиент/сервер для `blog` из `proto/blog.proto`.
///
/// Модуль формируется `tonic::include_proto!` в `build.rs`.
/// Содержит сообщения и сервис `BlogService`.
pub mod proto_blog {
    tonic::include_proto!("blog");
}
