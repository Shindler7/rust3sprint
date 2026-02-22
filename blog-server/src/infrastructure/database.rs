//! Взаимодействие с базой данных.

use crate::infrastructure::config::DBCfg;
use anyhow::{Context, Result as AnyhowResult};
use sqlx::{
    migrate, migrate::Migrate, migrate::Migrator, postgres::PgPoolOptions, Database, PgPool, Pool,
};

/// Подсказка при ошибках миграции базы данных.
const DB_MIGRATE_HELP: &str = r#"
Ошибка может быть вызвана несоответствием миграций структуре таблиц. В этом
случае необходимо отредактировать файлы миграций. В худшем случае откатить
прошлые миграциии или сбросить базу данных до начального состояния.

Если проблема связана с доступом пользователя к публичным схемам, необходимо 
проверить предоставленные ему права.

postgres=# GRANT ALL PRIVILEGES ON DATABASE <db_name> TO <user>;
Команда предоставляет все основные права пользователю, но для CREATE на схему
может быть недостаточно.

Минимально нужно:
postgres=# ALTER DATABASE <db_name> OWNER TO <user>;
postgres=# GRANT USAGE, CREATE ON SCHEMA public TO <user>;
"#;

static MIGRATOR: Migrator = migrate!("./migrations");

/// Создать асинхронный пул подключений к базе данных PostgresSQL. Успешный
/// результат возвращает экземпляр [`PgPool`].
///
/// ## Args
/// - `db_param` — экземпляр конфигурации БД [`DBCfg`] с параметрами
/// - `migrate` — если True, вызывается метод миграций
pub(crate) async fn get_pool_postgres(db_param: &DBCfg, migrate: bool) -> AnyhowResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(db_param.max_conn)
        .connect(&db_param.db_url)
        .await
        .with_context(|| {
            format!(
                "не удалось подключиться к БД PostgresSQL ({})",
                db_param.db_url
            )
        })?;

    if migrate {
        migrations(&pool).await?;
    }

    Ok(pool)
}

/// Осуществить миграции через пул.
///
/// Принимаются объекты [`Pool`] и производные, например, [`PgPool`], которая
/// является обёрткой для `Pool`, специально для `postgres`.
pub(crate) async fn migrations<DB>(pool: &Pool<DB>) -> AnyhowResult<()>
where
    DB: Database,
    <DB as Database>::Connection: Migrate,
{
    MIGRATOR
        .run(pool)
        .await
        .with_context(|| format!("не удалось применить миграции.\n{DB_MIGRATE_HELP}"))?;

    Ok(())
}
