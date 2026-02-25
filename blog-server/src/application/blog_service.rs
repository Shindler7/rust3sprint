//! Бизнес-логика блога.

use crate::{
    data::post_repo::PostRepository,
    domain::{
        post::{CreatePost, EditPostCommand, Post},
        types::DataId,
    },
    errors::{DomainError, RepoErrorMap, SqlxResultExt},
};
use std::sync::Arc;
use tracing::{error, info, instrument};

/// Сервисы для взаимодействия с записями блога.
pub(crate) struct BlogService<R: PostRepository + 'static> {
    repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: PostRepository + 'static,
{
    /// Создать сервис [`AuthService`] с репозиторием пользователей.
    pub(crate) fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Получить публикацию по id.
    ///
    /// Метод открытый, проверки на авторство не требуется.
    #[instrument(skip(self), level = "debug")]
    pub(crate) async fn get_post(&self, post_id: &DataId) -> Result<Post, DomainError> {
        let post = self.repo.get(post_id).await.map_repo_err(RepoErrorMap {
            not_found: DomainError::PostNotFound,
            unique_violations: None,
        })?;

        Ok(post)
    }

    /// Создание публикации (поста).
    ///
    /// Данные принимаются в обёртке новых типов (например, [`CreatePost`],
    /// которые обеспечивают базовую валидацию и гарантируют типы.
    #[instrument(skip(self, new_post), level = "debug", fields(title=%new_post.title, author=%author))]
    pub(crate) async fn create_post(
        &self,
        new_post: &CreatePost,
        author: &DataId,
    ) -> Result<Post, DomainError> {
        let post = Post::new_by_create(new_post.clone(), author.clone());
        let post = self.repo.create(&post).await.map_err(|err| {
            error!(
                error=%err,
                title=%new_post.title,
                author=%author,
                "Ошибка создания записи в базе данных"
            );
            DomainError::server_err(err.to_string())
        })?;

        info!(
            title = %post.title,
            post_id = ?post.id,
            author_id = %post.author_id,
            "Создана новая запись в блоге");

        Ok(post)
    }

    /// Предоставить список опубликованных постов всех авторов.
    #[instrument(skip(self), level = "debug")]
    pub(crate) async fn list_posts(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Post>, DomainError> {
        let posts = self.repo.list(limit, offset).await.map_err(|err| {
            error!(
                error=%err,
                "Не удалось получить из БД список постов"
            );
            DomainError::server_err(err.to_string())
        })?;

        Ok(posts)
    }

    /// Отредактировать существующую публикацию.
    ///
    /// Проводится проверка, что автор публикации совпадает с авторизованным
    /// пользователем.
    #[instrument(skip(self, edit_command), level = "debug", fields(post_id=%edit_command.post_id, user_id=%user_id))]
    pub(crate) async fn update_post(
        &self,
        edit_command: &EditPostCommand,
        user_id: &DataId,
    ) -> Result<Post, DomainError> {
        let mut post = self.get_post(&edit_command.post_id).await?;

        if !post.is_author(user_id) {
            return Err(DomainError::Forbidden);
        }

        post.update(&edit_command.edit_post);
        self.repo.update(&post).await.map_err(|err| {
            error!(
                error=%err,
                post_id=%edit_command.post_id,
                user_id=%user_id,
                "Ошибка обновления публикации"
            );
            DomainError::server_err(err.to_string())
        })?;

        Ok(post)
    }

    /// Удалить публикацию.
    ///
    /// Обязательно проводится проверка, что пользователь является автором.
    #[instrument(skip(self), level = "debug")]
    pub(crate) async fn delete_post(
        &self,
        post_id: &DataId,
        user_id: &DataId,
    ) -> Result<(), DomainError> {
        let author_id = self
            .repo
            .get_author_id(post_id)
            .await
            .map_repo_err(RepoErrorMap {
                not_found: DomainError::PostNotFound,
                unique_violations: None,
            })?;

        if !author_id.eq(user_id) {
            return Err(DomainError::Forbidden);
        }

        self.repo.delete(post_id).await.map_repo_err(RepoErrorMap {
            not_found: DomainError::PostNotFound,
            unique_violations: None,
        })?;

        Ok(())
    }
}
