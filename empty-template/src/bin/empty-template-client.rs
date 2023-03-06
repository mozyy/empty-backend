use std::time::{Instant, SystemTime};

use empty_registry::REGISTRY_ADDR;
use empty_template::pb::TemplateRequest;
use rand::distributions::{Alphanumeric, DistString};

#[tokio::main]
async fn main() {
    empty_utils::init();
    let t1: Vec<_> = [0; 1]
        .iter()
        .map(|_| tokio::spawn(call(REGISTRY_ADDR)))
        .collect();
    let t2: Vec<_> = [0; 1]
        .iter()
        .map(|_| tokio::spawn(call("0.0.0.0:36807")))
        .collect();
    // now await them to get the resolve's to complete
    call(REGISTRY_ADDR).await;
    call("0.0.0.0:36807").await;
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

async fn call(addr: &str) {
    let mut client = empty_template::pb::template_service_client::TemplateServiceClient::connect(
        format!("http://{addr}"),
    )
    .await
    .unwrap();
    let start = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let request = TemplateRequest { name: start };
    let response = client.template(tonic::Request::new(request)).await.unwrap();
    log::info!("{:?}", response);
}
