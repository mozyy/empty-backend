use std::env;

use blog::pb;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| String::from("127.0.0.1:50051"));
    let mut client = pb::blog_service_client::BlogServiceClient::connect(base_url.to_owned())
        .await
        .unwrap();
    log::info!("connect");
    let blogs = client
        .list(tonic::Request::new(pb::ListRequest {}))
        .await
        .unwrap();
    log::info!("connect success");
    dbg!(blogs);
}
