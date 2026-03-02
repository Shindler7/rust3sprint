//! Навигационные панели.

use crate::{state::blog_state::BlogAppState, routes::Route};
use yew::prelude::*;
use yew_router::prelude::*;

/// Верхняя навигационная панель с основным меню.
#[function_component(Nav)]
pub(crate) fn nav_top() -> Html {
    let state = use_context::<BlogAppState>().expect("Отсутствует BlogAppState");

    let is_auth = state.is_authenticated();
    let username = state.get_username().cloned().unwrap_or("Noname".to_string());

    html! {
        <header class="navbar">
            <div class="container">
                <div class="nav-brand">
                    <Link<Route> to={Route::Home}>
                        {"Darwin's Blog"}
                    </Link<Route>>
                </div>

                <nav class="nav-links">
                    // Основные ссылки, доступные всем
                    <Link<Route> classes={classes!("nav-link")} to={Route::Home}>
                        {"Главная"}
                    </Link<Route>>

                    // Ссылки для неавторизованных пользователей
                    if !is_auth {
                        <Link<Route> classes={classes!("nav-link")} to={Route::Register}>
                            {"Регистрация"}
                        </Link<Route>>

                        <Link<Route> classes={classes!("nav-link")} to={Route::Login}>
                            {"Авторизация"}
                        </Link<Route>>
                    }

                    // Ссылки для авторизованных пользователей
                    if is_auth {
                        <Link<Route> classes={classes!("nav-link")} to={Route::Create}>
                            {"Создать запись"}
                        </Link<Route>>

                        <Link<Route> classes={classes!("nav-link")} to={Route::Logout}>
                            {format!("Выйти, {}", username)}
                        </Link<Route>>
                    }
                </nav>
            </div>
        </header>
    }
}
