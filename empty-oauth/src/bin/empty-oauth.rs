use empty_oauth::pb::oauth_server::OauthServer;

#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_registry::client::register(OauthServer::default())
        .await
        .unwrap();
}
