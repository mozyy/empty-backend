use empty_registry::registry::registry_service_server::RegistryService;
use empty_registry::RegistryServiceServer;
use tonic::{transport::Server, Request, Response, Status};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    let addr = "127.0.0.1:50051".parse().unwrap();
    let greeter = empty_registry::server::Server::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(RegistryServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
