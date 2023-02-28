pub use registry::health_service_server;
pub mod registry {
    tonic::include_proto!("empty.registry.v1");
}

pub use registry::registry_service_server::RegistryServiceServer;
pub mod client;
pub mod server;

pub const REGISTRY_ADDR: &str = env!("REGISTRY_ADDR");
