pub mod model;
pub mod schema;
// pub mod temp;
// pub mod service;
pub mod pb {
    tonic::include_proto!("lottery.v1");
}
