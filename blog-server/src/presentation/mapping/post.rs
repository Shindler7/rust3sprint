//! Конвертеры для постов (публикаций) HTTP - gRPC.

use crate::domain::{
    post::{CreatePost, EditPost, EditPostCommand, ListPosts, Post},
    types::{DataId, PostContent, PostTitle},
};
use proto_crate::proto_blog::{
    CreatePostRequest, ListPostsResponse, Post as ProtoPost, UpdatePostRequest,
};
use tonic::Status;
use tracing::error;

impl TryFrom<CreatePostRequest> for CreatePost {
    type Error = Status;

    fn try_from(cpr: CreatePostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: cpr.title.try_into()?,
            content: cpr.content.try_into()?,
        })
    }
}

impl TryFrom<Post> for ProtoPost {
    type Error = Status;
    fn try_from(p: Post) -> Result<Self, Self::Error> {
        let id: i64 =
            p.id.ok_or_else(|| {
                error!(
                title=%p.title,
                author_id=%p.author_id,
                "Ошибка преобразования Post/ProtoPost: отсутствует ID");
                Status::internal("Отсутствует ID публикации (поста)")
            })?
            .into();

        let created_at = p.created_at.timestamp();
        let updated_at = p.updated_at.map(|dt| dt.timestamp());

        Ok(Self {
            id,
            title: p.title.to_string(),
            content: p.content.to_string(),
            author_id: p.author_id.into(),
            created_at,
            updated_at,
        })
    }
}

impl TryFrom<UpdatePostRequest> for EditPostCommand {
    type Error = Status;
    fn try_from(u: UpdatePostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            post_id: DataId::from(u.id),
            edit_post: EditPost {
                title: u.title.map(PostTitle::try_from).transpose()?,
                content: u.content.map(PostContent::try_from).transpose()?,
            },
        })
    }
}

impl TryFrom<ListPosts> for ListPostsResponse {
    type Error = Status;
    fn try_from(posts: ListPosts) -> Result<Self, Self::Error> {
        let grpc_posts: Vec<ProtoPost> = posts
            .posts
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<_, _>>()?;

        Ok(ListPostsResponse {
            posts: grpc_posts,
            total: posts.total,
            limit: posts.limit,
            offset: posts.offset,
        })
    }
}
