use tonic::transport::NamedService;

use crate::register::{
    health_service_client::HealthServiceClient, register_service_client::RegisterServiceClient,
    RegisterRequest,
};

pub async fn register<S: NamedService>() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RegisterServiceClient::connect(crate::REGISTER_ADDR).await?;
    let name = S::NAME;
    let request = tonic::Request::new(RegisterRequest {
        name: name.to_string(),
    });

    client.register(request).await?;

    log::info!("register: {:?}", name);

    Ok(())
}
