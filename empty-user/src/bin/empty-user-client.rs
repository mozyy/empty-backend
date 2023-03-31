use empty_registry::get_registry_addr;
use empty_user::pb;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let registry_addr = get_registry_addr();

    log::info!("start");
    let mut client = pb::user_client::UserClient::connect(format!("http://{registry_addr}"))
        .await
        .unwrap();
    let request = pb::LoginRequest {
        mobile: "111111".to_string(),
        password: "222222".to_string(),
    };
    let response = client.login(tonic::Request::new(request)).await.unwrap();
    log::info!("{:?}", response);
}
