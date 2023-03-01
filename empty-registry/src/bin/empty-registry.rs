
use empty_registry::RegistryServiceServer;
use tonic::{transport::Server};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_health_reporter, health_service) = tonic_health::server::health_reporter();

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
