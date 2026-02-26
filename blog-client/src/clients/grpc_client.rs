//! Транспорт для взаимодействия с gRPC-сервером.

use crate::{
    clients::{
        models::{PostCreateCmd, PostId, PostUpdateCmd, UserAuthCmd, UserRegisterCmd},
        traits::ClientTransportExt,
    },
    models::{AuthResponse, Token},
    BlogClientError,
};
use proto_crate::proto_blog::{
    blog_service_client::BlogServiceClient, CreatePostRequest, GetPostRequest, ListPostsResponse, LoginRequest, Post,
    RegisterRequest,
};
use reqwest::Url;
use tonic::{async_trait, metadata::MetadataValue, transport::Channel, Request, Status};

/// Перечисление для адаптера регистрации и аутентификации.
enum AuthCmd {
    /// Регистрация пользователя.
    Register(RegisterRequest),
    /// Авторизация пользователя.
    Login(LoginRequest),
}

/// Клиент для взаимодействия с gRPC-сервером.
#[derive(Debug)]
pub(crate) struct GrpcClient {
    /// Ссылка для доступа к API сервера.
    server_url: Url,
}

impl GrpcClient {
    pub(crate) fn new(server_url: Url) -> Self {
        Self { server_url }
    }

    /// Создание канала [`Channel`] с gRPC-сервером для обмена данными.
    async fn get_client(&self) -> Result<BlogServiceClient<Channel>, BlogClientError> {
        BlogServiceClient::connect(self.server_url.to_string())
            .await
            .map_err(BlogClientError::TonicError)
    }

    /// Адаптер для регистрации и аутентификации пользователя на сервере.
    async fn authenticate(&self, cmd: AuthCmd) -> Result<AuthResponse, BlogClientError> {
        let mut client = self.get_client().await?;

        let auth_user = match cmd {
            AuthCmd::Register(req) => client.register(req).await?.into_inner(),
            AuthCmd::Login(req) => client.login(req).await?.into_inner(),
        };

        Ok(auth_user.into())
    }

    /// Добавить токен в [`Request`] унифицировано.
    ///
    /// Производится преобразование в [`MetadataValue`] и добавляется
    /// в метаданные запроса.
    ///
    /// ## Errors
    ///
    /// Если возникнет ошибка при преобразовании `String` в `MetadataValue`.
    fn add_token_to_req<T>(
        &self,
        req: &mut Request<T>,
        token: &Token,
    ) -> Result<(), BlogClientError> {
        let req_token: MetadataValue<_> = token.bearer().parse().map_err(|_| {
            BlogClientError::GrpcError(Status::internal("Ошибка преобразования токена"))
        })?;
        req.metadata_mut().insert("authorization", req_token);

        Ok(())
    }
}

#[async_trait]
impl ClientTransportExt for GrpcClient {
    type Error = BlogClientError;

    async fn register_user(&self, cmd: UserRegisterCmd) -> Result<AuthResponse, Self::Error> {
        self.authenticate(AuthCmd::Register(cmd.into())).await
    }

    async fn login_user(&self, cmd: UserAuthCmd) -> Result<AuthResponse, Self::Error> {
        self.authenticate(AuthCmd::Login(cmd.into())).await
    }

    async fn create_post(&self, cmd: PostCreateCmd, token: &Token) -> Result<Post, Self::Error> {
        let mut client = self.get_client().await?;

        let mut request = Request::new(CreatePostRequest::from(cmd));
        self.add_token_to_req(&mut request, token)?;

        let post = client.create_post(request).await?.into_inner();

        post.post.ok_or_else(|| {
            BlogClientError::GrpcError(Status::data_loss("Данные о публикации отсутствуют"))
        })
    }

    async fn get_post(&self, post_id: PostId) -> Result<Post, Self::Error> {
        let mut client = self.get_client().await?;

        let post_req = GetPostRequest { id: post_id.into() };
        let post = client.get_post(post_req).await?.into_inner();

        post.post
            .ok_or_else(|| BlogClientError::GrpcError(Status::not_found("")))
    }

    async fn update_post(&self, _cmd: PostUpdateCmd, _token: &Token) -> Result<Post, Self::Error> {
        todo!()
    }

    async fn delete_post(&self, _post_id: PostId, _token: &Token) -> Result<(), Self::Error> {
        todo!()
    }

    async fn list_posts(
        &self,
        _limit: Option<u32>,
        _offset: Option<u32>,
    ) -> Result<ListPostsResponse, Self::Error> {
        todo!()
    }
}
