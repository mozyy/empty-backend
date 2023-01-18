use empty_register::register::{register_service_client::RegisterServiceClient, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RegisterServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(RegisterRequest {
        name: "Tonic".into(),
    });

    let response = client.register(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
