//! Макросы

/// Создаёт структуру для слоя данных, работающего с базой данной PostgresSQL.
#[macro_export]
macro_rules! repo_pg_pool {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$meta])*
        $vis struct $name {
            pool: PgPool,
        }

        impl $name {
            pub(crate) fn new(pool: &PgPool) -> Self {
                Self { pool: pool.clone() }
            }
        }
    };
}
