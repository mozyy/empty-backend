pub mod pb {
    tonic::include_proto!("empty.registry.v1");
}

pub use pb::heartbeat_server::HeartbeatServer;
pub use pb::registry_server::RegistryServer;
pub mod client;
pub mod heartbeat;
pub mod proxy;
pub mod registry;
pub mod schema;

pub fn get_registry_addr() -> String {
    std::env::var("REGISTRY_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:50051"))
}
