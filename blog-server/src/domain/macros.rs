//! Поддерживающие доменные макросы.

/// Макрос обеспечивает унифицированную валидацию нового типа с помощью
/// предоставленной функции.
///
/// Предоставляет методы `try_from`, а также `as_str` и `as_ref` и `display`.
#[macro_export]
macro_rules! validated_newtype {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
        validate = $validate:path;
        error = $err:path;
    ) => {
        $(#[$meta])*
        $vis struct $name(String);

        impl TryFrom<String> for $name {
            type Error = DomainError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match $validate(&value) {
                    Ok(_) => Ok(Self(value)),
                    Err(e) => Err($err(e)),
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        // impl $name {
        //     pub(crate) fn as_str(&self) -> &str {
        //         self.0.as_str()
        //     }
        // }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

    };
}
