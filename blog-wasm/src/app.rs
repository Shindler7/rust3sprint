//! Родительский уровень сервировки приложения.
//! Определяются базовые параметры приложения (app, роутеры и т.п.).

use crate::{
    components::{footer::Footer, nav::Nav},
    routes::{switch, Route},
    state::blog_state::BlogAppState,
};

use yew::prelude::*;
use yew_router::prelude::*;

/// Главная страница и первая точка входа во фронт.
#[component(Main)]
pub(crate) fn main_app() -> Html {
    let blog_state = use_state(BlogAppState::default);

    html! {
        <ContextProvider<BlogAppState> context={(*blog_state).clone()}>
            <ContextProvider<UseStateHandle<BlogAppState>> context={blog_state}>
                <BrowserRouter>
                    <div class="page">
                        <Nav />

                        <main class="container">
                            <Switch<Route> render={switch} />
                        </main>

                        <Footer />
                    </div>
                </BrowserRouter>
            </ContextProvider<UseStateHandle<BlogAppState>>>
        </ContextProvider<BlogAppState>>
    }
}
