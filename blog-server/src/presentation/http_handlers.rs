// //! Handlers для обработки HTTP-соединений.
// 
// use crate::domain::user::AuthenticatedUser;
// use crate::{
//     application::auth_service::AuthService,
//     data::user_repo::UserRepo,
//     domain::{
//         error::DomainError,
//         user::{AuthResponse, CreateUser, LoginUser, User},
//     },
//     infrastructure::{config::BlogConfig, jwt::JwtService},
// };
// use actix_web::{get, post, web, HttpResponse, Responder, Result as ActixResult};
// use serde_json::json;
// use std::sync::Arc;
// use tracing::error;
// 
// /// Статус состояния приложения.
// ///
// /// `/api/health`
// #[get("/health")]
// async fn health() -> impl Responder {
//     HttpResponse::Ok().json(json!({"status": "ok"}))
// }
// 
// /// Регистрация пользователя.
// ///
// /// `/api/auth/register`
// #[post("/auth/register")]
// async fn register(
//     auth_service: web::Data<Arc<AuthService<UserRepo>>>,
//     body: web::Json<CreateUser>,
//     server_cfg: web::Data<Arc<BlogConfig>>,
// ) -> ActixResult<impl Responder, DomainError> {
//     let create_user = body.into_inner();
// 
//     let user = auth_service
//         .create_user(&create_user)
//         .await
//         .inspect_err(|err| {
//             error!(
//             error = %err,
//             username = %create_user.username,
//             email = %create_user.email,
//             "Ошибка регистрации {}", create_user.username)
//         })?;
// 
//     let token = get_jwt_token(&user, &server_cfg.security.jwt_service)?;
// 
//     Ok(HttpResponse::Created().json(AuthResponse {
//         token,
//         user: user.into(),
//     }))
// }
// 
// /// Авторизация пользователя.
// ///
// /// `/api/auth/login`
// #[post("/auth/login")]
// async fn login(
//     auth_service: web::Data<Arc<AuthService<UserRepo>>>,
//     body: web::Json<LoginUser>,
//     server_cfg: web::Data<Arc<BlogConfig>>,
// ) -> ActixResult<impl Responder, DomainError> {
//     let login_user = body.into_inner();
//     let user = auth_service
//         .get_user(&login_user.username)
//         .await
//         .inspect_err(|err| {
//             error!(
//                 error = %err,
//                 username = %login_user.username,
//                 "Пользователь не найден: {}", login_user.username)
//         })?;
// 
//     let verified_hash = login_user
//         .password
//         .verify_hash(&user.password_hash)
//         .map_err(|err| DomainError::invalid_credentials(format!("ошибка хеширования: {err}")))?;
// 
//     if !verified_hash {
//         return Err(DomainError::invalid_password(""));
//     }
// 
//     let token = get_jwt_token(&user, &server_cfg.security.jwt_service)?;
// 
//     Ok(HttpResponse::Ok().json(AuthResponse {
//         token,
//         user: user.into(),
//     }))
// }
// 
// /// Создание публикации (поста) в блоге. Требует аутентификации.
// ///
// /// `/api/posts`
// #[post("/posts")]
// async fn create_post(
//     user: web::ReqData<AuthenticatedUser>,
// ) -> ActixResult<impl Responder, DomainError> {
//     let user = user.into_inner();
//     Ok(HttpResponse::Ok().json(user))
// }
// 
// /// Технический метод: регистрация роутеров.
// pub(crate) fn configurate(cfg: &mut web::ServiceConfig) {
//     cfg.service(health)
//         .service(register)
//         .service(login)
//         .service(create_post);
// }
// 
// /// Поддерживающая функция. Получает JWT-токен для пользователя.
// fn get_jwt_token(user: &User, jwt_service: &JwtService) -> Result<String, DomainError> {
//     let user_id = user
//         .id
//         .as_ref()
//         .ok_or_else(|| DomainError::server_err("Пользователь не имеет ID"))?;
//     Ok(jwt_service.generate_token(&user_id, &user.username)?)
// }
