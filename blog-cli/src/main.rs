//! Библиотека для взаимодействия с сервером через командную строку.
use blog_client::{BlogClient, Transport};

#[tokio::main]
async fn main() {
    let transport = Transport::http("http://127.0.0.1:8080");
    let mut client = BlogClient::new(transport.unwrap()).await.unwrap();

    let user = client
        .register("girlsweet3", "sweety3@email.ru", "John2219@Ii")
        .await
        .unwrap();

    let grpc = Transport::grpc("http://127.0.0.1:50051").unwrap();
    let grpc_client = BlogClient::new(grpc).await.unwrap();

    let posts = grpc_client.list_posts(Some(10), None).await.unwrap();
    let posts_grpc = grpc_client.list_posts(Some(10), None).await.unwrap();

    println!("{:?}, {:?} and grpc: {:?}", user, posts, posts_grpc);
}
