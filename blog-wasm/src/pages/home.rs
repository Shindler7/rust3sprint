//! Домашняя (главная) страница.

use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::Route, services::clients::BlogClient, state::models::PostsStatus};

use commons::string_from_datetime;

/// Главная страница проекта.
#[function_component(Home)]
pub(crate) fn home() -> Html {
    let posts = use_state(Vec::new);
    let total = use_state(|| 0_i32);
    let limit = use_state(|| 10_i32);
    let offset = use_state(|| 0_i32);
    let status = use_state(|| PostsStatus::Idle);

    // Подгрузка постов при изменении offset.
    {
        let posts = posts.clone();
        let total = total.clone();
        let limit_state = limit.clone();
        let status = status.clone();

        use_effect_with(*offset, move |offset_dep| {
            status.set(PostsStatus::Loading);

            let posts = posts.clone();
            let total = total.clone();
            let limit_state = limit_state.clone();
            let status = status.clone();
            let offset = *offset_dep;
            let limit = *limit_state;

            spawn_local(async move {
                let client = match BlogClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        status.set(PostsStatus::Error(e.to_string()));
                        return;
                    }
                };

                match client.load_posts(limit, offset).await {
                    Ok(resp) => {
                        posts.set(resp.posts);
                        total.set(resp.total);
                        limit_state.set(resp.limit);
                        status.set(PostsStatus::Idle);
                    }
                    Err(e) => status.set(PostsStatus::Error(e.to_string())),
                }
            });

            || ()
        });
    }

    let on_prev = {
        let offset = offset.clone();
        let limit = limit.clone();
        Callback::from(move |_| {
            let next = (*offset - *limit).max(0);
            offset.set(next);
        })
    };

    let on_next = {
        let offset = offset.clone();
        let limit = limit.clone();
        let total = total.clone();
        Callback::from(move |_| {
            let next = *offset + *limit;
            if next < *total {
                offset.set(next);
            }
        })
    };

    let can_prev = *offset > 0;
    let can_next = (*offset + *limit) < *total;

    html! {
        <section class="container">
            <div class="card">
                <h1>{ "Публикации" }</h1>

                {
                    match &*status {
                        PostsStatus::Loading => html! {
                            <div class="status-message loading">{ "Загружаем посты..." }</div>
                        },
                        PostsStatus::Error(msg) => html! {
                            <div class="status-message error">{ msg.clone() }</div>
                        },
                        PostsStatus::Idle => html! {},
                    }
                }

                <div class="post-list">
                    {
                        if posts.is_empty() && matches!(&*status, PostsStatus::Idle) {
                            html! { <p>{ "Пока нет публикаций." }</p> }
                        } else {
                            html! {
                                <>
                                    { for posts.iter().map(|post| {
                                        let created_at = string_from_datetime(post.created_at, true);

                                        html! {
                                            <article class="card post-card">
                                                <h2>{ post.title.clone() }</h2>
                                                   if let Some(dt) = created_at {
                                                   <p class="post-meta">
                                                        { format!("{dt}") }
                                                   </p>
                                                    }
                                                <p class="post-meta">
                                                    { format!("ID автора: {}", post.author_id) }
                                                </p>
                                                <p>{ post.content.clone() }</p>

                                                <div style="margin-top: 0.8rem;">
                                                    <Link<Route>
                                                        classes={classes!("btn-secondary", "btn")}
                                                        to={Route::Post { id: post.id }}
                                                    >
                                                        { "Открыть" }
                                                    </Link<Route>>
                                                </div>
                                            </article>
                                        }
                                    }) }
                                </>
                            }
                        }
                    }
                </div>

                <div class="pagination">
                    <button class="btn btn-secondary" onclick={on_prev} disabled={!can_prev}>
                        { "Назад" }
                    </button>
                    <button class="btn btn-secondary" onclick={on_next} disabled={!can_next}>
                        { "Вперёд" }
                    </button>
                </div>
            </div>
        </section>
    }
}
