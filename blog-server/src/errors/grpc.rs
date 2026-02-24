//! Ошибки для gRPC-сервера.

use crate::errors::DomainError;
use tonic::{Code, Status};

impl From<DomainError> for Status {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidEmail(_)
            | DomainError::InvalidUsername(_)
            | DomainError::InvalidPostContent(_)
            | DomainError::ApiError(_) => Status::new(Code::InvalidArgument, err.to_string()),

            DomainError::Forbidden => Status::new(Code::PermissionDenied, err.to_string()),

            DomainError::UserNotFound | DomainError::PostNotFound => {
                Status::new(Code::NotFound, err.to_string())
            }

            DomainError::UserAlreadyExists | DomainError::EmailAlreadyExists => {
                Status::new(Code::AlreadyExists, err.to_string())
            }

            DomainError::InvalidCredentials(_) | DomainError::InvalidPassword(_) => {
                Status::new(Code::Unauthenticated, err.to_string())
            }

            DomainError::ServerError(_) => Status::new(Code::Internal, err.to_string()),
        }
    }
}
