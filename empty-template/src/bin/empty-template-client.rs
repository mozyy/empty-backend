use empty_registry::REGISTRY_ADDR;
use empty_template::pb::TemplateRequest;

#[tokio::main]
async fn main() {
    let mut client = empty_template::pb::template_service_client::TemplateServiceClient::connect(
        format!("http://{REGISTRY_ADDR}"),
    )
    .await
    .unwrap();
    let request = TemplateRequest {
        name: "aaaaa".into(),
    };
    let response = client.template(tonic::Request::new(request)).await.unwrap();
    println!("{:?}", response);
}
