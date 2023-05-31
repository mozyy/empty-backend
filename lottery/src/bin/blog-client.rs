// use std::env;

// use proto::pb::blog;

// #[tokio::main]
// async fn main() {
//     empty_utils::init();
//     let base_url = env::var("BASE_URL").unwrap_or_else(|_| String::from("127.0.0.1:50051"));
//     let mut client = blog::blog_service_client::BlogServiceClient::connect(base_url.to_owned())
//         .await
//         .unwrap();
//     log::info!("connect");
//     let blogs = client
//         .list(tonic::Request::new(blog::ListRequest {}))
//         .await
//         .unwrap();
//     log::info!("connect success");
//     dbg!(blogs);
// }
fn main(){}