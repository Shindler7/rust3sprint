//! Страницы с ошибками приложения.

use yew::prelude::*;

/// Ошибка 404: страница не найдена.
#[function_component(PageNotFound)]
pub(crate) fn page_not_found() -> Html {
    html! {
        <div class="container">
            <div class="card" style="text-align:center;">
                <h1>{ "404. Page not found" }</h1>
                <p>{ "Такой страницы нет. Вернитесь на главную." }</p>

                <div style="margin-top:2rem;">
                    <a class="btn" href="/">{ "На главную" }</a>
                </div>
            </div>
        </div>
    }
}
