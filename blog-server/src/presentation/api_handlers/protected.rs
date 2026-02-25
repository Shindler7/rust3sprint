//! Защищённые `endpoints` HTTP-сервера.

use crate::{
    application::blog_service::BlogService,
    data::post_repo::PostRepo,
    domain::{
        post::{CreatePost, EditPost, EditPostCommand},
        types::DataId,
        user::AuthenticatedUser,
    },
    errors::DomainError,
};
use actix_web::{
    delete, post, put, web, web::{Json, ReqData}, HttpResponse, Responder,
    Result as ActixResult,
};
use std::sync::Arc;
use tracing::error;

/// Создание публикации (требует аутентификации).
///
/// `/api/posts`
#[post("")]
async fn create_post(
    user: ReqData<AuthenticatedUser>,
    body: Json<CreatePost>,
    blog_service: web::Data<Arc<BlogService<PostRepo>>>,
) -> ActixResult<impl Responder, DomainError> {
    let author = user.into_inner();
    let post = body.into_inner();

    let post = blog_service
        .create_post(&post, &author.id)
        .await
        .inspect_err(|err| {
            error!(
                error = %err,
                author_id = %author.id,
                username = %author.username,
                "Ошибка создания новой записи в блоге"
            )
        })?;

    Ok(HttpResponse::Created().json(post))
}

/// Обновление публикации (поста) (требует аутентификации).
///
/// `/api/posts/{id}`
#[put("/{id}")]
async fn update_post(
    user: ReqData<AuthenticatedUser>,
    body: Json<EditPost>,
    post_id: web::Path<DataId>,
    blog_service: web::Data<Arc<BlogService<PostRepo>>>,
) -> ActixResult<impl Responder, DomainError> {
    let user = user.into_inner();

    let edit_command = EditPostCommand::new(post_id.into_inner(), body.into_inner());

    let post = blog_service
        .update_post(&edit_command, &user.id)
        .await
        .inspect_err(|err| {
            error!(
                error = %err,
                post_id = %edit_command.post_id,
                user_id = %user.id,
                "Неудачная попытка внесения изменений в публикацию"
            )
        })?;

    Ok(HttpResponse::Ok().json(post))
}

/// Удаление публикации (поста) (требует аутентификации).
///
/// `/api/posts/{id}`
#[delete("/{id}")]
async fn delete_post(
    user: ReqData<AuthenticatedUser>,
    post_id: web::Path<DataId>,
    blog_service: web::Data<Arc<BlogService<PostRepo>>>,
) -> ActixResult<impl Responder, DomainError> {
    let user = user.into_inner();
    let post_id = post_id.into_inner();

    blog_service
        .delete_post(&post_id, &user.id)
        .await
        .inspect_err(|err| {
            error!(
                error = %err,
                post_id = %post_id,
                user_id = %user.id,
                "Ошибка при попытке удаления публикации"
            )
        })?;

    Ok(HttpResponse::NoContent().finish())
}

/// Конфигурация роутеров.
pub(super) fn configure_posts_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(create_post)
            .service(update_post)
            .service(delete_post),
    );
}
