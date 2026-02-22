//! Конфигурация веб-сервера.

use crate::infrastructure::jwt::JwtService;
use crate::settings::{DB_MAX_CONN, DB_URL_TEMPLATE};
use anyhow::{anyhow, Context, Result as AnyhowResult};
use std::{
    env,
    fmt::Display,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

trait Cfg {
    /// Сформировать данные конфигурации.
    fn collect() -> AnyhowResult<Self>
    where
        Self: Sized;
}

/// Конфигурационные данные для сервера Blog.
#[derive(Clone)]
pub(crate) struct BlogConfig {
    /// Настройки сервера.
    pub server: ServerCfg,
    /// Настройки безопасности.
    pub security: SecurityCfg,
    /// Настройки для базы данных.
    pub db: DBCfg,
}

impl BlogConfig {
    /// Загрузить параметры конфигурации.
    pub(crate) fn load() -> AnyhowResult<Self> {
        let server = ServerCfg::collect()?;
        let security = SecurityCfg::collect()?;
        let db = DBCfg::collect()?;

        Ok(Self {
            server,
            security,
            db,
        })
    }
}

/// Настройки сервера.
#[derive(Clone)]
pub(crate) struct ServerCfg {
    /// IP-адрес сервера.
    pub host: Ipv4Addr,
    /// Порт сервера.
    pub port: u16,
    /// Порт сервера gRPC.
    pub port_grpc: u16,
}

impl ServerCfg {
    /// Ленивая генерация адреса сервера в экземпляре `SocketAddr`.
    pub(crate) fn server_addr(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(self.host, self.port))
    }

    /// Ленивая генерация адреса сервера gRPC в `SocketAddr`.
    pub(crate) fn grpc_addr(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(self.host, self.port_grpc))
    }
}

impl Cfg for ServerCfg {
    fn collect() -> AnyhowResult<Self> {
        let host: Ipv4Addr = load_from_env("SERVER_HOST")?;
        let port: u16 = load_from_env("SERVER_PORT")?;
        let port_grpc: u16 = load_from_env("GRPC_PORT")?;

        Ok(Self {
            host,
            port,
            port_grpc,
        })
    }
}

/// Настройки безопасности.
#[derive(Clone)]
pub(crate) struct SecurityCfg {
    /// Разрешённый origin для CORS (или "*" для всех).
    pub cors_url: String,
    /// Таймаут запроса в секундах.
    pub cors_max_age: usize,
    /// Инфраструктура для обработки JWT-токенов.
    pub jwt_service: JwtService,
}

impl Cfg for SecurityCfg {
    fn collect() -> AnyhowResult<Self> {
        let cors_url = load_from_env("CORS_URL")?;
        let cors_max_age = load_from_env("CORS_MAX_AGE")?;

        // Создание JWT-механизации.
        let jwt_secret: String = load_from_env("JWT_SECRET_KEY")?;
        let jwt_service = JwtService::from_secret(jwt_secret);

        Ok(Self {
            cors_url,
            cors_max_age,
            jwt_service,
        })
    }
}

/// Настройки для базы данных.
#[derive(Clone)]
pub(crate) struct DBCfg {
    /// Ссылка для доступа к базе данных.
    pub db_url: String,
    /// Максимальное количество соединений, поддерживаемых пулом.
    pub max_conn: u32,
}

impl DBCfg {
    /// Создать ссылку для доступа к базе данных PostgresSQL.
    ///
    /// ## Args
    /// - `user` — имя пользователя
    /// - `pass` — пароль пользователя
    /// - `host` — хост базы данных
    /// - `port` — порт базы данных
    /// - `db_name` — название базы данных
    fn postgres_url(user: &str, pass: &str, host: &str, port: u16, db_name: &str) -> String {
        DB_URL_TEMPLATE
            .replace("{user}", user)
            .replace("{password}", pass)
            .replace("{host}", host)
            .replace("{port}", &port.to_string())
            .replace("{db}", db_name)
    }
}

impl Cfg for DBCfg {
    fn collect() -> AnyhowResult<Self> {
        let db_user: String = load_from_env("DB_USERNAME")?;
        let db_pwd: String = load_from_env("DB_PASSWORD")?;
        let db_host: String = load_from_env("DB_HOST")?;
        let db_port: u16 = load_from_env("DB_PORT")?;
        let db_name: String = load_from_env("DB_NAME")?;

        let db_url = DBCfg::postgres_url(&db_user, &db_pwd, &db_host, db_port, &db_name);
        let max_conn = DB_MAX_CONN;

        Ok(Self { db_url, max_conn })
    }
}

/// Загрузить указанный параметр из окружения.
///
/// Дженерик преобразует значение из файла в требуемый тип, если возможно.
fn load_from_env<T>(name: &str) -> AnyhowResult<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = env::var(name).with_context(|| format!("Отсутствует значение `{}` в env", name))?;

    s.parse::<T>()
        .map_err(|e| anyhow!("Ошибка преобразования {name}: {e}"))
}
