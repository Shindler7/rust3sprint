//! BlogState один из ключевых компонентов представления.
//!
//! Обеспечивает хранение и обработку токена, взаимодействие с Api.

use crate::domain::types::JwtToken;
use tracing::error;
use web_sys::{window, Storage};

/// ID пользователя на сервере блога.
const USER_BLOG_ID_FIELD: &str = "blog_user_id";

/// Имя поля для хранения в local storage JWT-токена.
const TOKEN_FIELD: &str = "blog_token";

/// Имя поля для хранения в local storage username.
const USERNAME_FIELD: &str = "blog_username";

/// Структура состояния взаимодействия с API Blog.
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct BlogAppState {
    /// ID пользователя на сервере блога.
    user_blog_id: Option<u64>,
    /// Имя авторизованного пользователя.
    username: Option<String>,
    /// Токен доступа к API.
    token: Option<JwtToken>,
}

impl Default for BlogAppState {
    fn default() -> Self {
        let user_blog_id: Option<u64> = Self::load_user_id();
        let username = Self::load_username();
        let token = Self::load_token();

        Self {
            user_blog_id,
            username,
            token,
        }
    }
}

impl BlogAppState {
    /// Статус аутентификации пользователя.
    pub(crate) fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Сохранить предоставленный токен в `Local Storage` и в структуре.
    pub(crate) fn save_user_data(
        &self,
        user_blog_id: u64,
        username: String,
        token: &JwtToken,
    ) -> Self {
        /// Внутренняя функция сохранения в local storage.
        fn save_to_storage(cell: &str, elem: &str, storage: &Storage) {
            if let Err(err) = storage.set_item(cell, elem) {
                error!(
                    error=?err,
                    cell=?cell,
                    elem=?elem,
                    "Ошибка сохранения токена в local storage"
                );
            }
        }

        let mut s = self.clone();

        s.username = Some(username.clone());
        s.token = Some(token.clone());
        s.user_blog_id = Some(user_blog_id);

        if let Some(storage) = Self::get_storage() {
            for (cell, elem) in [
                (USERNAME_FIELD, &username),
                (TOKEN_FIELD, &token.to_string()),
                (USER_BLOG_ID_FIELD, &user_blog_id.to_string()),
            ] {
                save_to_storage(cell, elem, &storage)
            }
        }

        s
    }

    pub(crate) fn get_user_blog_id(&self) -> Option<u64> {
        self.user_blog_id
    }

    /// Получить сохранённый токен.
    pub(crate) fn get_token(&self) -> Option<JwtToken> {
        self.token.clone()
    }

    /// Получить сохранённый username.
    pub(crate) fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    /// Удаление данных о пользователе (logout).
    pub(crate) fn clear_user_data(&self) -> Self {
        let mut s = self.clone();
        s.username = None;
        s.token = None;
        s.user_blog_id = None;

        if let Some(storage) = Self::get_storage() {
            let _ = storage.remove_item(TOKEN_FIELD);
            let _ = storage.remove_item(USERNAME_FIELD);
            let _ = storage.remove_item(USER_BLOG_ID_FIELD);
        }

        s
    }

    /// Загрузить токен из `localStorage`.
    fn load_token() -> Option<JwtToken> {
        Self::load_field(TOKEN_FIELD).map(JwtToken::new)
    }

    /// Загрузить имя пользователя из `localStorage`.
    fn load_username() -> Option<String> {
        Self::load_field(USERNAME_FIELD)
    }

    fn load_user_id() -> Option<u64> {
        if let Some(id) = Self::load_field(USER_BLOG_ID_FIELD) {
            id.parse::<u64>().ok()
        } else {
            None
        }
    }

    /// Получить доступ к экземпляру `Storage`.
    fn get_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    /// Прочитать строку из `localStorage` по ключу.
    fn load_field(key: &str) -> Option<String> {
        Self::get_storage()?.get_item(key).ok()?
    }
}
