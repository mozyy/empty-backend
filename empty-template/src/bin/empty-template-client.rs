use std::time::SystemTime;

use empty_registry::REGISTRY_ADDR;
use empty_template::pb::TemplateRequest;

#[tokio::main]
async fn main() {
    let t1: Vec<_> = [0; 100]
        .iter()
        .map(|_| tokio::spawn(call(REGISTRY_ADDR)))
        .collect();
    let t2: Vec<_> = [0; 100]
        .iter()
        .map(|_| tokio::spawn(call("0.0.0.0:36807")))
        .collect();
    // now await them to get the resolve's to complete

    let start = SystemTime::now();
    for t in t1 {
        t.await.unwrap();
    }
    let second = SystemTime::now();
    for t in t2 {
        t.await.unwrap();
    }
    let end = SystemTime::now();
    println!("{:?}, {:?}, {:?}", start, second, end);
}

async fn call(addr: &str) {
    let mut client = empty_template::pb::template_service_client::TemplateServiceClient::connect(
        format!("http://{addr}"),
    )
    .await
    .unwrap();
    let request = TemplateRequest {
        name: "aaaaa".into(),
    };
    let response = client.template(tonic::Request::new(request)).await.unwrap();
    log::info!("{:?}", response);
}
