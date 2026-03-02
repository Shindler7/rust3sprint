//! Поддерживающие инструменты для клиентского модуля.
use anyhow::{Context, Result as AnyhowResult};
use blog_client::Post;
use std::path::Path;
use tokio::{
    fs::{read_to_string, OpenOptions},
    io::AsyncWriteExt,
};

use commons::string_from_timestamp;

/// Загрузить токен из файла.
pub(super) async fn read_token(token_file: &Path) -> Option<String> {
    let content = read_to_string(token_file).await.ok()?;
    let content = content.trim().to_string();

    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

/// Сохранить токен в файл.
pub(super) async fn save_token(file: &Path, token: &str) -> AnyhowResult<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file)
        .await
        .with_context(|| "Токен не удалось сохранить в файл")?;

    f.write_all(token.as_bytes()).await?;
    Ok(())
}

pub(super) fn print_success() {
    println!("УСПЕШНО\n");
}

/// Вывести в консоль сообщение с одной публикацией.
pub(super) fn print_one_post(post: &Post) {
    let created_at =
        string_from_timestamp(post.created_at, true).unwrap_or_else(|| "неизвестно".to_string());

    let updated_at = post
        .updated_at
        .and_then(|ts| string_from_timestamp(ts, true))
        .map(|upd| format!(" (обновлено: {upd})"))
        .unwrap_or_default();

    let message = format!(
        "[id {}] {}{}\n\
         {}\n\
         {}",
        post.id, created_at, updated_at, post.title, post.content
    );

    println!("{message}\n");
}

pub(super) fn print_token_not_set() {
    println!("Не выполнено. Отсутствует токен авторизации")
}
