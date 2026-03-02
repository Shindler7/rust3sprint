//! Репозиторий постов.

use crate::{
    domain::{post::Post, types::DataId},
    repo_pg_pool,
};
use sqlx::{postgres::PgRow, Error as SqlxError, PgPool, Row};
use tonic::async_trait;

#[async_trait]
pub(crate) trait PostRepository: Send + Sync {
    /// Получить публикацию по её id.
    async fn get(&self, post_id: &DataId) -> Result<Post, SqlxError>;

    /// Создать публикацию (пост).
    async fn create(&self, post: &Post) -> Result<Post, SqlxError>;

    /// Предоставить список публикаций, сделанных всеми авторами за всё время.
    ///
    /// ## Args
    ///
    /// - `limit` — количество записей, должно быть больше 1
    /// - `offset` — отступ от первой записи в извлечённом списке
    /// 
    /// ## Returns
    /// 
    /// Перечень публикаций, с учётом заказа, и общее число публикаций в базе.
    async fn list(&self, limit: i32, offset: i32) -> Result<(Vec<Post>, i64), SqlxError>;

    /// Обновление существующей публикации (поста).
    async fn update(&self, post: &Post) -> Result<(), SqlxError>;

    /// Удаление публикации (поста).
    async fn delete(&self, post_id: &DataId) -> Result<(), SqlxError>;

    /// Возвращает ID автора публикации, если пост с предоставленным id
    /// существует.
    async fn get_author_id(&self, post_id: &DataId) -> Result<DataId, SqlxError>;
}

repo_pg_pool!(
    #[derive(Clone)]
    pub(crate) struct PostRepo;
);

#[async_trait]
impl PostRepository for PostRepo {
    async fn get(&self, post_id: &DataId) -> Result<Post, SqlxError> {
        let record = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at FROM posts WHERE id = $1
            "#,
        )
        .bind(post_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(make_post_by_row(&record))
    }

    async fn create(&self, post: &Post) -> Result<Post, SqlxError> {
        let record = sqlx::query(
            r#"
            INSERT INTO posts (title, content, author_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.author_id)
        .bind(post.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(make_post_by_row(&record))
    }

    /// Предоставить список публикаций, сделанных всеми авторами за всё время.
    ///
    /// ## Args
    ///
    /// - `limit` — количество записей, должно быть больше 1
    /// - `offset` — отступ от первой записи в извлечённом списке
    ///
    /// ## Сортировка
    ///
    /// В текущей реализации выгрузка производится по дате создания, от самой
    /// молодой (поле `created_at` индексировано).
    ///
    /// ## Returns
    ///
    /// Возвращает перечень публикаций, с учётом условий заказа, а также общее
    /// количество публикаций в базе данных.
    async fn list(&self, limit: i32, offset: i32) -> Result<(Vec<Post>, i64), SqlxError> {
        let results = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let posts = results
            .into_iter()
            .map(|row| make_post_by_row(&row))
            .collect();

        let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
            .fetch_one(&self.pool)
            .await?;

        Ok((posts, total_count))
    }

    async fn update(&self, post: &Post) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            UPDATE posts
            SET title = $1, content = $2, updated_at = $3
            WHERE id = $4
            "#,
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(post.updated_at)
        .bind(&post.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, post_id: &DataId) -> Result<(), SqlxError> {
        let result = sqlx::query(
            r#"
            DELETE FROM posts WHERE id = $1
            "#,
        )
        .bind(post_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(SqlxError::RowNotFound);
        }

        Ok(())
    }

    async fn get_author_id(&self, post_id: &DataId) -> Result<DataId, SqlxError> {
        let record = sqlx::query(
            r#"
            SELECT author_id FROM posts WHERE id = $1
            "#,
        )
        .bind(post_id)
        .fetch_one(&self.pool)
        .await?;

        let author_id = record.get("author_id");
        Ok(author_id)
    }
}

/// Создать [`Post`] на основе выгрузки строки из базы данных.
fn make_post_by_row(record: &PgRow) -> Post {
    Post::new(
        record.get("id"),
        record.get("title"),
        record.get("content"),
        record.get("author_id"),
        record.get("created_at"),
        record.get("updated_at"),
    )
}
