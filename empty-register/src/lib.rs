pub use register::health_service_server;
pub mod register {
    tonic::include_proto!("empty.register.v1");
}

pub use register::register_service_server::RegisterServiceServer;
pub mod client;
pub mod server;

pub const REGISTER_ADDR: &str = env!("REGISTER_ADDR");
