//! Транспорт для взаимодействия с gRPC-сервером.

use crate::{
    BlogClientError,
    clients::{
        models::{
            PostCreateCmd, PostId, PostResponseWrap, PostUpdateCmd, UserAuthCmd, UserRegisterCmd,
        },
        traits::ClientTransportExt,
    },
    models::{AuthResponse, Token},
};
use proto_crate::proto_blog::{
    CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest, ListPostsResponse,
    LoginRequest, Post, RegisterRequest, UpdatePostRequest, blog_service_client::BlogServiceClient,
};
use tonic::{
    Request, Status, async_trait,
    metadata::MetadataValue,
    transport::{Channel, Endpoint, Uri},
};

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
    /// Объект для коммуникации с gRPS сервером.
    channel: Channel,
}

impl GrpcClient {
    /// Создание gRPC-клиента для взаимодействия с сервером.
    pub(crate) async fn new(server_url: Uri) -> Result<Self, BlogClientError> {
        let endpoint = Endpoint::from(server_url.clone());
        let channel = endpoint.connect().await?;

        Ok(Self { channel })
    }

    /// Создание канала [`Channel`] с gRPC-сервером для обмена данными.
    async fn get_service_client(&self) -> BlogServiceClient<Channel> {
        BlogServiceClient::new(self.channel.clone())
    }

    /// Адаптер для регистрации и аутентификации пользователя на сервере.
    async fn authenticate(&self, cmd: AuthCmd) -> Result<AuthResponse, BlogClientError> {
        let mut client = self.get_service_client().await;

        let auth_user = match cmd {
            AuthCmd::Register(req) => client.register(Request::new(req)).await?.into_inner(),
            AuthCmd::Login(req) => client.login(Request::new(req)).await?.into_inner(),
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
        let mut client = self.get_service_client().await;

        let mut request = Request::new(CreatePostRequest::from(cmd));
        self.add_token_to_req(&mut request, token)?;

        let post: PostResponseWrap = client.create_post(request).await?.into_inner().into();

        post.get_post_or_loss()
    }

    async fn get_post(&self, post_id: PostId) -> Result<Post, Self::Error> {
        let mut client = self.get_service_client().await;

        let post_req = Request::new(GetPostRequest { id: post_id.into() });
        let post: PostResponseWrap = client.get_post(post_req).await?.into_inner().into();

        post.get_post_or_loss()
    }

    async fn update_post(&self, cmd: PostUpdateCmd, token: &Token) -> Result<Post, Self::Error> {
        let mut client = self.get_service_client().await;

        let mut request = Request::new(UpdatePostRequest::from(cmd));
        self.add_token_to_req(&mut request, token)?;

        let post: PostResponseWrap = client.update_post(request).await?.into_inner().into();

        post.get_post_or_loss()
    }

    async fn delete_post(&self, post_id: PostId, token: &Token) -> Result<(), Self::Error> {
        let mut client = self.get_service_client().await;

        let post_req = DeletePostRequest { id: post_id.into() };
        let mut request = Request::new(post_req);
        self.add_token_to_req(&mut request, token)?;

        client.delete_post(request).await?;

        Ok(())
    }

    async fn list_posts(&self, limit: u32, offset: u32) -> Result<ListPostsResponse, Self::Error> {
        fn u32_to_i32(n: u32, name: &str) -> Result<i32, BlogClientError> {
            i32::try_from(n).map_err(|_| {
                BlogClientError::invalid_req(format!("слишком большое значение '{}': {}", name, n))
            })
        }

        let limit_i32 = u32_to_i32(limit, "limit")?;
        let offset_i32: i32 = u32_to_i32(offset, "offset")?;

        let mut client = self.get_service_client().await;
        let posts_req = ListPostsRequest {
            limit: limit_i32,
            offset: offset_i32,
        };
        let request = Request::new(posts_req);

        Ok(client.list_posts(request).await?.into_inner())
    }
}

impl PostResponseWrap {
    /// Извлечь экземпляр [`Post`] из структуры, а если `None` конвертировать
    /// в ошибку [`BlogClientError::GrpcError`] от [`Status`].
    fn get_post_or_loss(&self) -> Result<Post, BlogClientError> {
        let post = self
            .post_response
            .post
            .as_ref()
            .ok_or(BlogClientError::GrpcError(Status::data_loss(
                "Данные о публикации отсутствуют (не переданы сервером)",
            )))?
            .clone();

        Ok(post)
    }
}
