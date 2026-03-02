//! Механизация парсинга аргументов командной строки.

use anyhow::Result as AnyhowResult;
use clap::{Parser, Subcommand};

/// Supported server commands.
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Register a new user (returns a JWT token).
    Register {
        /// Username. Length between 3 and 32 characters.
        /// Allowed characters: Latin letters, digits (except as first character), and '_'.
        #[arg(short, long)]
        username: String,

        /// Email address.
        #[arg(short, long)]
        email: String,

        /// Password. Minimum length of 10 characters, must contain
        /// both uppercase and lowercase letters, and at least one special
        /// character.
        #[arg(short, long)]
        password: String,
    },

    /// Authenticate an existing user (returns a JWT token).
    Login {
        /// Username.
        #[arg(short, long)]
        username: String,

        /// Password.
        #[arg(short, long)]
        password: String,
    },

    /// Create a new post (token required).
    Create {
        /// Post title. Maximum length of 100 characters.
        #[arg(short, long)]
        title: String,
        /// Post content. Please follow ethical guidelines and show respect for readers.
        #[arg(short, long)]
        content: String,
    },

    /// Retrieve a specific post.
    Get {
        /// Post ID.
        #[arg(short, long, value_parser=validate_post_id)]
        post_id: i64,
    },

    /// Update an existing post (token required).
    Update {
        /// Post ID.
        #[arg(short, long, value_parser=validate_post_id)]
        post_id: i64,

        /// New post title. Maximum length of 100 characters. Optional.
        #[arg(short, long)]
        title: Option<String>,

        /// New post content. Please follow ethical guidelines and show respect for readers.
        /// Optional.
        #[arg(short, long)]
        content: Option<String>,
    },

    /// Delete a post (token required).
    Delete {
        /// Post ID.
        #[arg(short, long, value_parser=validate_post_id)]
        post_id: i64,
    },

    /// List posts with pagination support.
    List {
        /// Number of records to return. If not provided, the default value
        /// is used.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Number of records to skip. Optional.
        #[arg(short, long)]
        offset: Option<u32>,
    },
}

/// Валидировать значение `post_id`: корректность типа и значения.
fn validate_post_id(post_id: &str) -> Result<i64, String> {
    let id = post_id
        .parse::<i64>()
        .map_err(|_| format!("Post ID must be a positive integer: {post_id}"))?;

    if id < 0 {
        return Err("Post ID less than 0".into());
    }

    Ok(id)
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    /// Supported server commands.
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Use gRPC protocol.
    #[arg(long)]
    pub(crate) grpc: bool,
}

/// Получить от пользователя задачу из командной строки.
pub(crate) fn read_args() -> AnyhowResult<CliArgs> {
    let cli_args = CliArgs::parse();

    Ok(cli_args)
}
