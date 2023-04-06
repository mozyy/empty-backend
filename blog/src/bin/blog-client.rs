use proto::blog;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let mut client =
        blog::blog_service_client::BlogServiceClient::connect("http://127.0.0.1:51051")
            .await
            .unwrap();
    log::info!("connect");
    let blogs = client
        .list(tonic::Request::new(blog::ListRequest {}))
        .await
        .unwrap();
    log::info!("connect success");
    dbg!(blogs);
}
