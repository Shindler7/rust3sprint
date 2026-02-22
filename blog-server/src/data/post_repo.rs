//! Репозиторий постов.

use crate::repo_pg_pool;
use sqlx::PgPool;
use tonic::async_trait;

#[async_trait]
pub(crate) trait PostRepository: Send + Sync {
    /// Создать публикацию (пост).
    async fn create(&self) {}
}

repo_pg_pool!(
    #[derive(Clone)]
    pub(crate) struct PostRepo;
);

#[async_trait]
impl PostRepository for PostRepo {
    async fn create(&self) {}
}
