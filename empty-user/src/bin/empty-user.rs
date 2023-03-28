use empty_user::{pb::user_server::UserServer, service::Service};

#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_registry::client::register(UserServer::new(Service::new()))
        .await
        .unwrap();
}
