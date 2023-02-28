use tonic::transport::NamedService;

use crate::registry::{
    health_service_client::HealthServiceClient, registry_service_client::RegistryServiceClient,
    RegisterRequest,
};

pub async fn register<S: NamedService>() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RegistryServiceClient::connect(crate::REGISTRY_ADDR).await?;
    let name = S::NAME;
    let request = tonic::Request::new(RegisterRequest {
        name: name.to_string(),
    });

    client.register(request).await?;

    log::info!("register: {:?}", name);

    Ok(())
}
