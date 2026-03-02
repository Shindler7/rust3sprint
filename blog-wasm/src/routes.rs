//! Роутеры приложения.

use crate::pages::{
    auth::{Login, Logout, SignUp},
    errors::PageNotFound,
    home::Home,
    posts::{CreatePost, Post, UpdatePost},
};
use yew::{html, Html};
use yew_router::Routable;

/// Доступные направления во фронт-части.
#[derive(Routable, PartialEq, Clone, Debug)]
pub(crate) enum Route {
    /// Главная страница.
    #[at("/")]
    Home,

    /// Регистрация пользователя.
    #[at("/register")]
    Register,

    /// Авторизация пользователя.
    #[at("/login")]
    Login,

    /// Сброс авторизации пользователя.
    #[at("/logout")]
    Logout,

    /// Страница с одной публикацией.
    #[at("/post/:id")]
    Post { id: u64 },

    /// Создание публикации.
    #[at("/post_create")]
    Create,

    /// Обновление (редактирование) публикации.
    #[at("/post_edit/:id")]
    Update { id: u64 },

    /// 404. Страница не найдена.
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Переключатель роутеров.
pub(crate) fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }

        Route::Register => {
            html! { <SignUp /> }
        }

        Route::Login => {
            html! { <Login /> }
        }

        Route::Logout => {
            html! { <Logout /> }
        }

        Route::Post { id } => {
            html! { <Post {id} /> }
        }

        Route::Create => {
            html! { <CreatePost /> }
        }

        Route::Update { id } => {
            html! { <UpdatePost {id} /> }
        }

        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
