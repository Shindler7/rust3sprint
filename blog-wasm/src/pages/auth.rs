//! Страницы авторизации (создание учётной записи, логин).

use crate::services::models::LoginUser;
use crate::{
    routes::Route,
    services::{clients::BlogClient, models::RegisterUser},
    state::blog_state::BlogAppState,
    state::models::AuthStatus,
};
use tracing::error;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

/// Регистрация пользователя.
#[function_component(SignUp)]
pub(crate) fn sign_up() -> Html {
    let Some(blog_state) = use_context::<UseStateHandle<BlogAppState>>() else {
        error!("Недоступно состояние приложения в SignUp.");
        return html! { <Redirect<Route> to={Route::Home} /> };
    };

    let navigator = use_navigator().expect("Навигатор не доступен");
    let auth_status = use_state(|| AuthStatus::Logout);

    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let blog_state = blog_state.clone();
        let username = username.clone();
        let password = password.clone();
        let email = email.clone();
        let navigator = navigator.clone();

        let auth_status = auth_status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let blog_state = blog_state.clone();
            let username = (*username).clone();
            let password = (*password).clone();
            let email = (*email).clone();
            let navigator = navigator.clone();

            let auth_status = auth_status.clone();
            auth_status.set(AuthStatus::InProcess);

            spawn_local(async move {
                let reg_user = RegisterUser::new(&username, &email, &password);

                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(err) => {
                        auth_status.set(AuthStatus::Error(err.to_string()));
                        return;
                    }
                };

                match client.register(&reg_user).await {
                    Ok(r) => {
                        blog_state.set((*blog_state).save_user_data(
                            r.user.id,
                            r.user.username,
                            &r.token.into(),
                        ));
                        auth_status.set(AuthStatus::Signin);
                        navigator.push(&Route::Home);
                    }
                    Err(error) => auth_status.set(AuthStatus::auth_error(error.to_string())),
                }
            })
        })
    };

    // Эффект для редиректа при успешной регистрации с авторизацией.
    {
        let navigator = navigator.clone();
        let status = (*auth_status).clone();

        use_effect_with(status, move |status| {
            if matches!(status, AuthStatus::Signin) {
                navigator.push(&Route::Home);
            }
            || ()
        });
    }

    let status_view = match &*auth_status {
        AuthStatus::Signin => {
            html! {
                <div class="status-message success">
                    <span>{"Регистрация успешна! Перенаправляем..."}</span>
                </div>
            }
        }
        AuthStatus::Error(msg) => {
            html! {
                <div class="status-message error">
                    <span>{ msg }</span>
                </div>
            }
        }
        AuthStatus::InProcess => {
            html! {
                <div class="status-message loading">
                    <span>{ "Вход..." }</span>
                </div>
            }
        }
        _ => {
            html! {}
        }
    };

    html! {
        <form class="form" onsubmit={on_submit}>
            <div class="input-group">
                <label>{"Логин"}</label>
                <input
                    type="text"
                    value={(*username).clone()}
                    oninput={on_username_input}
                    placeholder="jenny_sweet"
                    required={true}
                />
            </div>

            <div class="input-group">
                <label>{"Электропочта"}</label>
                <input
                    type="email"
                    value={(*email).clone()}
                    oninput={on_email_input}
                    placeholder="your_name@yandex.ru"
                    required={true}
                />
            </div>

            <div class="input-group">
                <label>{"Пароль"}</label>
                <input
                    type="password"
                    value={(*password).clone()}
                    oninput={on_password_input}
                    placeholder=""
                    required={true}
                />
            </div>

            <button class="btn" type="submit" disabled={matches!(&*auth_status, AuthStatus::InProcess)}>{"Регистрация"}</button>

            { status_view }

        </form>
    }
}

/// Авторизация пользователя.
#[function_component(Login)]
pub(crate) fn login() -> Html {
    let Some(blog_state) = use_context::<UseStateHandle<BlogAppState>>() else {
        error!("Недоступно состояние приложения в Logout.");
        return html! { <Redirect<Route> to={Route::Home} /> };
    };

    let navigator = use_navigator().expect("Навигатор не доступен");
    let auth_status = use_state(|| AuthStatus::Logout);

    let username = use_state(String::new);
    let password = use_state(String::new);

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let blog_state = blog_state.clone();
        let username = username.clone();
        let password = password.clone();
        let navigator = navigator.clone();

        let auth_status = auth_status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let blog_state = blog_state.clone();
            let username = (*username).clone();
            let password = (*password).clone();
            let navigator = navigator.clone();

            let auth_status = auth_status.clone();
            auth_status.set(AuthStatus::InProcess);

            spawn_local(async move {
                let auth_user = LoginUser::new(&username, &password);

                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(err) => {
                        auth_status.set(AuthStatus::Error(err.to_string()));
                        return;
                    }
                };

                match client.login(&auth_user).await {
                    Ok(r) => {
                        blog_state.set((*blog_state).save_user_data(
                            r.user.id,
                            r.user.username,
                            &r.token.into(),
                        ));
                        auth_status.set(AuthStatus::Signin);
                        navigator.push(&Route::Home);
                    }
                    Err(err) => auth_status.set(AuthStatus::Error(err.to_string())),
                }
            })
        })
    };

    {
        let navigator = navigator.clone();
        let status = (*auth_status).clone();

        use_effect_with(status, move |status| {
            if matches!(status, AuthStatus::Signin) {
                navigator.push(&Route::Home);
            }
            || ()
        });
    }

    let status_view = match &*auth_status {
        AuthStatus::Signin => html! {
            <div class="status-message success">
                <span>{"Вход выполнен! Перенаправляем..."}</span>
            </div>
        },
        AuthStatus::Error(msg) => html! {
            <div class="status-message error">
                <span>{ msg }</span>
            </div>
        },
        AuthStatus::InProcess => html! {
            <div class="status-message loading">
                <span>{"Вход..."}</span>
            </div>
        },
        _ => html! {},
    };

    html! {
        <form class="form" onsubmit={on_submit}>
            <div class="input-group">
                <label>{"Логин"}</label>
                <input
                    type="text"
                    value={(*username).clone()}
                    oninput={on_username_input}
                    placeholder="ivanovich"
                    required={true}
                />
            </div>

            <div class="input-group">
                <label>{"Пароль"}</label>
                <input
                    type="password"
                    value={(*password).clone()}
                    oninput={on_password_input}
                    placeholder=""
                    required={true}
                />
            </div>

            <button class="btn" type="submit" disabled={matches!(&*auth_status, AuthStatus::InProcess)}>
                {"Войти"}
            </button>
            { status_view }
        </form>
    }
}

/// Сброс авторизации пользователя.
#[function_component(Logout)]
pub(crate) fn logout() -> Html {
    let Some(blog_state) = use_context::<UseStateHandle<BlogAppState>>() else {
        error!("Недоступно состояние приложения в Logout.");
        return html! { <Redirect<Route> to={Route::Home} /> };
    };

    let navigator = use_navigator().expect("Навигатор не доступен");
    let logout_completed = use_state(|| false);

    {
        let blog_state = blog_state.clone();
        let navigator = navigator.clone();
        let logout_completed = logout_completed.clone();

        use_effect_with((), move |_| {
            blog_state.set((*blog_state).clear_user_data());
            logout_completed.set(true);
            navigator.push(&Route::Home);

            || ()
        });
    }

    html! {
        <div class="status-message loading">
            <span>{"Выход из системы..."}</span>
        </div>
    }
}
