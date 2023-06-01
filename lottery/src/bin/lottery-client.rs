use std::env;

use lottery::pb;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| String::from("http://0.0.0.0:50051"));
    log::info!("connect:{base_url}");
    let mut client = pb::lottery_service_client::LotteryServiceClient::connect(base_url.to_owned())
        .await
        .unwrap();
    let blogs = client
        .list(tonic::Request::new(pb::ListRequest {}))
        .await
        .unwrap();
    log::info!("connect success");
    dbg!(blogs);
}
