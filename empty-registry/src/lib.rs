pub use pb::health_service_server;
pub mod pb {
    tonic::include_proto!("empty.registry.v1");
}

pub use pb::registry_service_server::RegistryServiceServer;
pub mod client;
pub mod proxy;
pub mod registry;
pub mod schema;

pub const REGISTRY_ADDR: &str = env!("REGISTRY_ADDR");
