use std::time::Instant;

use empty_registry::get_registry_addr;
use empty_oauth::pb::OauthRequest;
use rand::distributions::{Alphanumeric, DistString};

#[tokio::main]
async fn main() {
    empty_utils::init();
    let registry_addr = get_registry_addr();
    let t1: Vec<_> = [0; 1]
        .iter()
        .map(|_| tokio::spawn(call(registry_addr.clone())))
        .collect();
    let t2: Vec<_> = [0; 1]
        .iter()
        .map(|_| tokio::spawn(call("0.0.0.0:36807".to_string())))
        .collect();
    // now await them to get the resolve's to complete
    call(registry_addr.clone()).await;
    call("0.0.0.0:36807".to_string()).await;
    log::info!("start");
    let start = Instant::now();

    for t in t1 {
        t.await.unwrap();
    }
    let second = Instant::now();
    for t in t2 {
        t.await.unwrap();
    }
    let end = Instant::now();
    log::info!(
        "时间: registry: {:?}, client:{:?}",
        second.duration_since(start),
        end.duration_since(second)
    );
}

async fn call(addr: String) {
    let mut client = empty_oauth::pb::oauth_service_client::OauthServiceClient::connect(
        format!("http://{addr}"),
    )
    .await
    .unwrap();
    let start = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let request = OauthRequest { name: start };
    let response = client.oauth(tonic::Request::new(request)).await.unwrap();
    log::info!("{:?}", response);
}
