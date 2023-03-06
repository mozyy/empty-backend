

use empty_registry::{
    pb::{registry_service_client::RegistryServiceClient, RegisterRequest},
    REGISTRY_ADDR,
};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RegistryServiceClient::connect(format!("http://{REGISTRY_ADDR}")).await?;
    register(&mut client).await;

    // all(&mut client).await;

    Ok(())
}

async fn register(client: &mut RegistryServiceClient<Channel>) {
    let request = tonic::Request::new(RegisterRequest {
        name: "Tonic".into(),
        endpoint: "".into(),
    });

    let response = client.register(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}

async fn all(client: &mut RegistryServiceClient<Channel>) {
    let request = tonic::Request::new(());

    let response = client.all(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}
