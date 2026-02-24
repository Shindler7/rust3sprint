//! Макросы для поддержи ошибок.

/// Макрос позволяет создавать однотипные конструкторы ошибок `DomainError`.
///
/// ## Пример
///
/// ```
/// use crate::domain::error::DomainError;
///
/// /// Конструктор для ошибки [`DomainError::InvalidUsername`].
//  pub(crate) fn invalid_username<S: Into<String>>(err_msg: S) -> DomainError {
//      DomainError::InvalidUsername(err_msg.into())
//  }
/// ```
#[macro_export]
macro_rules! impl_domain_error_ctor {
    (
        $(
            $(#[$doc:meta])*
            fn $fn_name:ident => $variant:ident ;
        )*
        $(
            @custom
            $(#[$custom_doc:meta])*
            fn $custom_fn:ident ( $arg:ident : $arg_ty:ty ) -> $ret:ty $body:block
        )*
    ) => {
        $(
            $(#[$doc])*
            pub(crate) fn $fn_name<S: Into<String>>(err_msg: S) -> DomainError {
                DomainError::$variant(err_msg.into())
            }
        )*

        $(
            $(#[$custom_doc])*
            pub(crate) fn $custom_fn($arg: $arg_ty) -> $ret $body
        )*
    };
}
