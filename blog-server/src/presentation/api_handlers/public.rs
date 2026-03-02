//! Публичные роутеры HTTP-сервера.

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    data::{post_repo::PostRepo, user_repo::UserRepo},
    domain::{
        post::QueryPosts,
        types::DataId,
        user::{AuthResponse, CreateUser, LoginUser, UserDto},
    },
    errors::DomainError,
    infrastructure::config::BlogConfig,
    presentation::{
        api_handlers::tools::valid_query_posts_params,
        tools::{get_jwt_token, verified_user_password},
    },
};
use actix_web::{get, post, web, HttpResponse, Responder, Result as ActixResult};
use serde_json::json;
use std::sync::Arc;
use tracing::error;

/// Статус состояния приложения.
///
/// `/api/health`
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

/// Регистрация пользователя.
///
/// `/api/auth/register`
#[post("/register")]
async fn register(
    auth_service: web::Data<Arc<AuthService<UserRepo>>>,
    body: web::Json<CreateUser>,
    server_cfg: web::Data<Arc<BlogConfig>>,
) -> ActixResult<impl Responder, DomainError> {
    let create_user = body.into_inner();

    let user = auth_service
        .create_user(&create_user)
        .await
        .inspect_err(|err| {
            error!(
            error = %err,
            username = %create_user.username,
            email = %create_user.email,
            "Ошибка регистрации {}", create_user.username)
        })?;

    let user_dto: UserDto = user.into();
    let token = get_jwt_token(&user_dto, &server_cfg.security.jwt_service)?;
    let auth_response = AuthResponse::new(token, user_dto);

    Ok(HttpResponse::Created().json(auth_response))
}

/// Авторизация пользователя.
///
/// `/api/auth/login`
#[post("/login")]
async fn login(
    auth_service: web::Data<Arc<AuthService<UserRepo>>>,
    body: web::Json<LoginUser>,
    server_cfg: web::Data<Arc<BlogConfig>>,
) -> ActixResult<impl Responder, DomainError> {
    let login_user = body.into_inner();
    let user = auth_service
        .get_user(&login_user.username)
        .await
        .inspect_err(|err| {
            error!(
                error = %err,
                username = %login_user.username,
                "Пользователь не найден: {}", login_user.username)
        })?;

    verified_user_password(&login_user, &user.password_hash)?;

    let user_dto: UserDto = user.into();
    let token = get_jwt_token(&user_dto, &server_cfg.security.jwt_service)?;
    let auth_response = AuthResponse::new(token, user_dto);

    Ok(HttpResponse::Ok().json(auth_response))
}

/// Список постов (публичный, с пагинацией).
/// Извлекает query-параметры limit и offset (по умолчанию limit=10, offset=0).
///
/// `api/posts`
#[get("/posts")]
async fn get_posts(
    query: web::Query<QueryPosts>,
    blog_service: web::Data<Arc<BlogService<PostRepo>>>,
) -> ActixResult<impl Responder, DomainError> {
    let limit = query.limit.unwrap_or_default();
    let offset = query.offset.unwrap_or_default();
    let (limit_i32, offset_i32) = valid_query_posts_params(limit, offset)?;

    let posts = blog_service.list_posts(limit_i32, offset_i32).await?;

    Ok(HttpResponse::Ok().json(posts))
}

/// Возвращает публикацию по id, при наличии.
///
/// `api/posts/{id}`
#[get("/posts/{id}")]
async fn get_one_post(
    post_id: web::Path<DataId>,
    blog_service: web::Data<Arc<BlogService<PostRepo>>>,
) -> ActixResult<impl Responder, DomainError> {
    let post_id = post_id.into_inner();
    let post = blog_service.get_post(&post_id).await.inspect_err(|err| {
        error!(
            error = %err,
            post_id = %post_id,
            "Неудачная попытка чтения публикации"
        )
    })?;

    Ok(HttpResponse::Ok().json(post))
}

/// Публичные роутеры, кроме регистрации и авторизации.
pub(super) fn configure_list_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts).service(get_one_post).service(health);
}

/// Роутеры для регистрации и авторизации пользователей.
pub(super) fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login);
}
