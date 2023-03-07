pub use pb::health_service_server;
pub mod pb {
    tonic::include_proto!("empty.registry.v1");
}

pub use pb::registry_service_server::RegistryServiceServer;
pub mod client;
pub mod proxy;
pub mod registry;
pub mod schema;

pub fn get_registry_addr() -> String {
    std::env::var("REGISTRY_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:50051"))
}
