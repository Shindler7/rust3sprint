//! Footer.

use yew::prelude::*;
use chrono::{Datelike, Local};

/// Футер страниц проекта.
#[function_component(Footer)]
pub(crate) fn footer() -> Html {
    let year = Local::now().year();
    
    html! {
        <footer>
            { format!("Влад Бармичев, курс 'Rust для разработчиков', {year} год") }
        </footer>
    }
}
