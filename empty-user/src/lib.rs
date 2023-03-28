pub mod model;
pub mod schema;
pub mod service;
pub mod pb {
    tonic::include_proto!("empty.user.v1");
}
