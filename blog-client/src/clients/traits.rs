//! Трейты для обеспечения работы клиентов.

use crate::{
    clients::models::{PostCreateCmd, PostId, PostUpdateCmd, UserAuthCmd, UserRegisterCmd},
    models::{AuthResponse, Token},
};
use proto_crate::proto_blog::Post;
use tonic::async_trait;

/// Трейт для унифицирования транспортных асинхронных методов клиентов.
#[async_trait]
pub(crate) trait ClientTransportExt {
    type Error;

    /// Регистрация пользователя.
    async fn register_user(&self, cmd: UserRegisterCmd) -> Result<AuthResponse, Self::Error>;
    /// Авторизация пользователя.
    async fn login_user(&self, cmd: UserAuthCmd) -> Result<AuthResponse, Self::Error>;
    /// Создание публикации.
    async fn create_post(&self, cmd: PostCreateCmd, token: &Token) -> Result<Post, Self::Error>;
    /// Чтение публикации.
    async fn get_post(&self, post_id: PostId) -> Result<Post, Self::Error>;
    /// Обновление публикации.
    async fn update_post(&self, cmd: PostUpdateCmd, token: &Token) -> Result<Post, Self::Error>;
    /// Удаление публикации.
    async fn delete_post(&self, post_id: PostId, token: &Token) -> Result<(), Self::Error>;
    /// Просмотр публикаций с пагинацией.
    async fn list_posts(&self) -> Result<Vec<Post>, Self::Error>;
}
