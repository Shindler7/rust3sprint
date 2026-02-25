//! Конвертеры типов из HTTP сервера и gRPC.

use crate::{
    domain::user::{AuthResponse, CreateUser, LoginUser, UserDto},
    errors::DomainError,
};
use proto_crate::proto_blog::{
    AuthResponse as ProtoAuthResponse, LoginRequest, RegisterRequest, User as ProtoUser,
};
use tonic::Status;
use tracing::error;

impl TryFrom<RegisterRequest> for CreateUser {
    type Error = DomainError;

    fn try_from(r: RegisterRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            username: r.username.try_into()?,
            email: r.email.try_into()?,
            password: r.password.try_into()?,
        })
    }
}

impl TryFrom<LoginRequest> for LoginUser {
    type Error = DomainError;

    fn try_from(r: LoginRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            username: r.username.try_into()?,
            password: r.password.try_into()?,
        })
    }
}

impl TryFrom<UserDto> for ProtoUser {
    type Error = Status;

    fn try_from(user: UserDto) -> Result<Self, Self::Error> {
        let id: i64 = user
            .id
            .ok_or_else(|| {
                error!(
                username=%user.username,
                email=%user.email,
                "Ошибка преобразования User/ProtoUser: отсутствует ID");
                Status::internal("Отсутствует ID пользователя")
            })?
            .into();

        Ok(Self {
            id,
            username: user.username.to_string(),
            email: user.email.to_string(),
        })
    }
}

impl TryFrom<AuthResponse> for ProtoAuthResponse {
    type Error = Status;
    fn try_from(ar: AuthResponse) -> Result<Self, Self::Error> {
        let proto_user: ProtoUser = ar.user.try_into()?;

        Ok(Self {
            token: ar.token,
            user: Some(proto_user),
        })
    }
}
