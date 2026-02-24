//! Конвертеры типов из HTTP сервера и gRPC.

use crate::{
    domain::{
        types::{Email, UserPassword, Username},
        user::{CreateUser, LoginUser, User},
    },
    errors::DomainError,
};
use proto_crate::proto_blog::{LoginRequest, RegisterRequest, User as ProtoUser};
use tonic::Status;
use tracing::error;

impl TryFrom<RegisterRequest> for CreateUser {
    type Error = DomainError;

    fn try_from(r: RegisterRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            username: Username::try_from(r.username)?,
            email: Email::try_from(r.email)?,
            password: UserPassword::try_from(r.password)?,
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

impl TryFrom<User> for ProtoUser {
    type Error = Status;

    fn try_from(user: User) -> Result<Self, Self::Error> {
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
