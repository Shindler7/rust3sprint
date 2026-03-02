//! Страницы взаимодействия с публикациями.

use commons::string_from_datetime;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::services::models::EditPost;
use crate::{
    domain::types::PostId,
    routes::Route,
    services::{clients::BlogClient, models::CreatePost as CreatePostResp},
    state::{blog_state::BlogAppState, models::PostsStatus},
};

/// Страница отдельной публикации.
#[function_component(Post)]
pub(crate) fn one_post(post: &PostId) -> Html {
    let app_state =
        use_context::<UseStateHandle<BlogAppState>>().expect("Отсутствует BlogAppState");
    let navigator = use_navigator().expect("Навигатор недоступен");

    let post_id = post.id;
    let post_data = use_state(|| None);
    let status = use_state(|| PostsStatus::Loading);

    let is_deleting = use_state(|| false);
    let delete_error = use_state(|| None::<String>);

    {
        let post_data = post_data.clone();
        let status = status.clone();

        use_effect_with(post_id, move |post_id| {
            status.set(PostsStatus::Loading);

            let post_data = post_data.clone();
            let status = status.clone();
            let post_id = *post_id;

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        status.set(PostsStatus::Error(e.to_string()));
                        return;
                    }
                };

                match client.get_post(post_id.into()).await {
                    Ok(p) => {
                        post_data.set(Some(p));
                        status.set(PostsStatus::Idle);
                    }
                    Err(e) => status.set(PostsStatus::Error(e.to_string())),
                }
            });

            || ()
        });
    }

    let current_user_id = app_state.get_user_blog_id();
    let token = app_state.get_token();
    let is_auth = app_state.is_authenticated();

    let can_manage = is_auth
        && matches!(
            (current_user_id, post_data.as_ref()),
            (Some(uid), Some(p)) if p.author_id == uid
        );

    let on_delete = {
        let token = token.clone();
        let navigator = navigator.clone();
        let is_deleting = is_deleting.clone();
        let delete_error = delete_error.clone();

        Callback::from(move |_| {
            is_deleting.set(true);
            delete_error.set(None);

            let token = token.clone();
            let navigator = navigator.clone();
            let is_deleting = is_deleting.clone();
            let delete_error = delete_error.clone();

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        is_deleting.set(false);
                        delete_error.set(Some(e.to_string()));
                        return;
                    }
                };

                match client.delete_post(post_id.into(), token).await {
                    Ok(_) => navigator.push(&Route::Home),
                    Err(e) => {
                        is_deleting.set(false);
                        delete_error.set(Some(e.to_string()));
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <article class="card">
                {
                    match &*status {
                        PostsStatus::Loading => html! {
                            <div class="status-message loading">{ "Загружаем публикацию..." }</div>
                        },
                        PostsStatus::Error(msg) => html! {
                            <div class="status-message error">{ msg.clone() }</div>
                        },
                        PostsStatus::Idle => {
                            if let Some(p) = &*post_data {
                                let created_at = string_from_datetime(p.created_at, true);

                                html! {
                                    <>
                                        <h1>{ p.title.clone() }</h1>

                                        if let Some(dt) = created_at {
                                            <p class="post-meta">{ dt }</p>
                                        }

                                        <p class="post-meta">{ format!("ID автора: {}", p.author_id) }</p>
                                        <p>{ p.content.clone() }</p>

                                        if let Some(err) = &*delete_error {
                                            <div class="status-message error" style="margin-top: 1rem;">
                                                { err.clone() }
                                            </div>
                                        }

                                        <div style="margin-top: 2rem; display: flex; gap: 0.75rem; flex-wrap: wrap;">
                                            <Link<Route> classes={classes!("btn", "btn-secondary")} to={Route::Home}>
                                                { "Назад" }
                                            </Link<Route>>

                                            if can_manage {
                                                <Link<Route>
                                                    classes={classes!("btn", "btn-secondary")}
                                                    to={Route::Update { id: p.id }}
                                                >
                                                    { "Редактировать" }
                                                </Link<Route>>

                                                <button
                                                    class="btn btn-danger"
                                                    onclick={on_delete}
                                                    disabled={*is_deleting}
                                                >
                                                    { if *is_deleting { "Удаляем..." } else { "Удалить" } }
                                                </button>
                                            }
                                        </div>
                                    </>
                                }
                            } else {
                                html! {
                                    <div class="status-message error">{ "Публикация не найдена." }</div>
                                }
                            }
                        }
                    }
                }
            </article>
        </div>
    }
}

/// Создание публикации.
#[function_component(CreatePost)]
pub(crate) fn create_post() -> Html {
    let app_state =
        use_context::<UseStateHandle<BlogAppState>>().expect("Отсутствует BlogAppState");
    let navigator = use_navigator().expect("Навигатор недоступен");

    let title = use_state(String::new);
    let content = use_state(String::new);
    let status = use_state(|| PostsStatus::Idle);

    // Если не авторизован — не даём работать с формой.
    let is_auth = app_state.is_authenticated();
    {
        let navigator = navigator.clone();
        use_effect_with(is_auth, move |is_auth| {
            if !*is_auth {
                navigator.push(&Route::Login);
            }
            || ()
        });
    }

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let app_state = app_state.clone();
        let navigator = navigator.clone();
        let title = title.clone();
        let content = content.clone();
        let status = status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let title_value = title.trim().to_string();
            let content_value = content.trim().to_string();

            if title_value.is_empty() || content_value.is_empty() {
                status.set(PostsStatus::Error(
                    "Заголовок и содержание не должны быть пустыми".to_string(),
                ));
                return;
            }

            let token = app_state.get_token().clone();
            status.set(PostsStatus::Loading);

            let navigator = navigator.clone();
            let status = status.clone();

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        status.set(PostsStatus::Error(e.to_string()));
                        return;
                    }
                };

                let create_post = CreatePostResp::new(&title_value, &content_value);

                match client.create_post(&create_post, token).await {
                    Ok(created_post) => {
                        status.set(PostsStatus::Idle);
                        navigator.push(&Route::Post {
                            id: created_post.id,
                        });
                    }
                    Err(e) => status.set(PostsStatus::Error(e.to_string())),
                }
            });
        })
    };

    let is_loading = matches!(&*status, PostsStatus::Loading);

    html! {
        <section class="container">
            <article class="card">
                <h1>{ "Создать публикацию" }</h1>

                {
                    match &*status {
                        PostsStatus::Loading => html! {
                            <div class="status-message loading">{ "Публикуем..." }</div>
                        },
                        PostsStatus::Error(msg) => html! {
                            <div class="status-message error">{ msg.clone() }</div>
                        },
                        PostsStatus::Idle => html! {},
                    }
                }

                <form class="form" onsubmit={on_submit}>
                    <div class="input-group">
                        <label for="title">{ "Заголовок" }</label>
                        <input
                            id="title"
                            type="text"
                            value={(*title).clone()}
                            oninput={on_title_input}
                            placeholder="Введите заголовок"
                            disabled={is_loading}
                        />
                    </div>

                    <div class="input-group">
                        <label for="content">{ "Содержание" }</label>
                        <textarea
                            id="content"
                            value={(*content).clone()}
                            oninput={on_content_input}
                            placeholder="Введите текст публикации"
                            disabled={is_loading}
                        />
                    </div>

                    <button class="btn" type="submit" disabled={is_loading}>
                        { if is_loading { "Публикуем..." } else { "Опубликовать" } }
                    </button>
                </form>
            </article>
        </section>
    }
}

/// Изменение (обновление) публикации.
#[function_component(UpdatePost)]
pub(crate) fn update_post(post: &PostId) -> Html {
    let app_state =
        use_context::<UseStateHandle<BlogAppState>>().expect("Отсутствует BlogAppState");
    let navigator = use_navigator().expect("Навигатор недоступен");

    let post_id = post.id;

    let post_data = use_state(|| None);
    let load_status = use_state(|| PostsStatus::Loading);
    let submit_status = use_state(|| PostsStatus::Idle);

    let title = use_state(String::new);
    let content = use_state(String::new);

    let is_auth = app_state.is_authenticated();
    {
        let navigator = navigator.clone();
        use_effect_with(is_auth, move |is_auth| {
            if !*is_auth {
                navigator.push(&Route::Login);
            }
            || ()
        });
    }

    // Загружаем пост и заполняем форму.
    {
        let post_data = post_data.clone();
        let title = title.clone();
        let content = content.clone();
        let load_status = load_status.clone();

        use_effect_with(post_id, move |post_id| {
            load_status.set(PostsStatus::Loading);

            let post_data = post_data.clone();
            let title = title.clone();
            let content = content.clone();
            let load_status = load_status.clone();
            let post_id = *post_id;

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        load_status.set(PostsStatus::Error(e.to_string()));
                        return;
                    }
                };

                match client.get_post(post_id.into()).await {
                    Ok(p) => {
                        title.set(p.title.clone());
                        content.set(p.content.clone());
                        post_data.set(Some(p));
                        load_status.set(PostsStatus::Idle);
                    }
                    Err(e) => load_status.set(PostsStatus::Error(e.to_string())),
                }
            });

            || ()
        });
    }

    let current_user_id = app_state.get_user_blog_id();
    let can_manage = matches!(
        (current_user_id, post_data.as_ref()),
        (Some(uid), Some(p)) if p.author_id == uid
    );

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let app_state = app_state.clone();
        let navigator = navigator.clone();
        let title = title.clone();
        let content = content.clone();
        let submit_status = submit_status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let title_value = title.trim().to_string();
            let content_value = content.trim().to_string();

            if title_value.is_empty() || content_value.is_empty() {
                submit_status.set(PostsStatus::Error(
                    "Заголовок и содержание не должны быть пустыми".to_string(),
                ));
                return;
            }

            let token = app_state.get_token().clone();
            submit_status.set(PostsStatus::Loading);

            let navigator = navigator.clone();
            let submit_status = submit_status.clone();

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        submit_status.set(PostsStatus::Error(e.to_string()));
                        return;
                    }
                };

                let update_post = EditPost::new(&title_value, &content_value);
                match client
                    .update_post(post_id.into(), &update_post, token)
                    .await
                {
                    Ok(_) => {
                        submit_status.set(PostsStatus::Idle);
                        navigator.push(&Route::Post { id: post_id });
                    }
                    Err(e) => submit_status.set(PostsStatus::Error(e.to_string())),
                }
            });
        })
    };

    let is_submitting = matches!(&*submit_status, PostsStatus::Loading);

    html! {
        <section class="container">
            <article class="card">
                <h1>{ format!("Редактирование публикации {}", post_id) }</h1>

                {
                    match &*load_status {
                        PostsStatus::Loading => html! {
                            <div class="status-message loading">{ "Загружаем публикацию..." }</div>
                        },
                        PostsStatus::Error(msg) => html! {
                            <div class="status-message error">{ msg.clone() }</div>
                        },
                        PostsStatus::Idle => html! {},
                    }
                }

                if matches!(&*load_status, PostsStatus::Idle) {
                    if !can_manage {
                        <div class="status-message error">
                            { "Редактирование доступно только автору публикации." }
                        </div>
                    } else {
                        {
                            match &*submit_status {
                                PostsStatus::Loading => html! {
                                    <div class="status-message loading">{ "Сохраняем изменения..." }</div>
                                },
                                PostsStatus::Error(msg) => html! {
                                    <div class="status-message error">{ msg.clone() }</div>
                                },
                                PostsStatus::Idle => html! {},
                            }
                        }

                        <form class="form" onsubmit={on_submit}>
                            <div class="input-group">
                                <label for="title">{ "Заголовок" }</label>
                                <input
                                    id="title"
                                    type="text"
                                    value={(*title).clone()}
                                    oninput={on_title_input}
                                    disabled={is_submitting}
                                />
                            </div>

                            <div class="input-group">
                                <label for="content">{ "Содержание" }</label>
                                <textarea
                                    id="content"
                                    value={(*content).clone()}
                                    oninput={on_content_input}
                                    disabled={is_submitting}
                                />
                            </div>

                            <button class="btn" type="submit" disabled={is_submitting}>
                                { if is_submitting { "Сохраняем..." } else { "Сохранить" } }
                            </button>
                        </form>
                    }
                }
            </article>
        </section>
    }
}
