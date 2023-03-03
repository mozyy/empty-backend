use tonic::transport::NamedService;

use crate::pb::{registry_service_client::RegistryServiceClient, RegisterRequest};

// trait MicroService {
// }

pub struct MicroService {}

impl MicroService {
    pub fn register() {}
}

pub async fn register<S: NamedService>() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RegistryServiceClient::connect(crate::REGISTRY_ADDR).await?;
    let name = S::NAME;
    let request = tonic::Request::new(RegisterRequest {
        name: name.to_string(),
        endpoint: "".to_string(),
    });

    client.register(request).await?;

    log::info!("register: {:?}", name);

    Ok(())
}
