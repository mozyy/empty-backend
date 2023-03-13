use empty_oauth::pb::oauth_service_server::OauthServiceServer;

#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_registry::client::register(OauthServiceServer::default())
        .await
        .unwrap();
}
