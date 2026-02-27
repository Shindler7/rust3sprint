//! Выполнение команды пользователя на взаимодействие с серверами.

use crate::{
    cli::Commands,
    client::tools::{print_one_post, print_success, print_token_not_set, read_token, save_token},
    config::Settings,
};
use anyhow::Result as AnyhowResult;
use blog_client::BlogClient;

/// Исполнитель пользовательских заданий.
///
/// ## Args
///
/// - `server` — созданный и настроенный экземпляр клиента [`BlogClient`]
/// - `command` — команда пользователя с телом запроса
/// - `grpc` — если `true` запросы осуществляются через сервер `gRPC`, в ином
///   случае используется `http`.
///
/// ## Errors
///
/// Возвращает ошибку при неудачных запросах.
pub(crate) async fn execute_request(
    server: &mut BlogClient,
    command: &Commands,
    config: &Settings,
) -> AnyhowResult<()> {
    let token_from_file = read_token(&config.app_state.token_full_path).await;

    match command {
        Commands::Register {
            username,
            email,
            password,
        } => {
            let auth = server.register(username, email, password).await?;
            save_token(&config.app_state.token_full_path, auth.token.as_str()).await?;
            println!("Пользователь зарегистрирован: {}, {}", username, email);
        }

        Commands::Login { username, password } => {
            let auth = server.login(username, password).await?;
            save_token(&config.app_state.token_full_path, auth.token.as_str()).await?;
            println!("Пользователь прошёл авторизацию: {}", username);
        }

        Commands::Get { post_id } => {
            let post = server.get_post(*post_id).await?;
            print_success();
            print_one_post(&post);
        }

        Commands::List { limit, offset } => {
            let posts = server.list_posts(*limit, *offset).await?;
            print_success();
            posts.posts.into_iter().for_each(|post| {
                print_one_post(&post);
            });
        }

        Commands::Create { title, content } => {
            if let Some(token) = token_from_file {
                server.set_token(token.into());
                let post = server.create_post(title, content).await?;
                print_success();
                print_one_post(&post);
            } else {
                print_token_not_set();
            }
        }

        Commands::Update {
            post_id,
            title,
            content,
        } => {
            if let Some(token) = token_from_file {
                server.set_token(token.into());
                let post = server
                    .update_post(*post_id, title.as_deref(), content.as_deref())
                    .await?;
                print_success();
                print_one_post(&post);
            } else {
                print_token_not_set();
            }
        }

        Commands::Delete { post_id } => {
            if let Some(token) = token_from_file {
                server.set_token(token.into());
                server.delete_post(*post_id).await?;
                print_success();
                println!("Пост # {} удалён", post_id);
            } else {
                print_token_not_set();
            }
        }
    }

    Ok(())
}
