

pub mod model;
pub mod schema;
pub mod service;
pub mod types;
pub mod pb {
    tonic::include_proto!("lottery");
}

pub fn new () -> pb::lottery_service_server::LotteryServiceServer<service::Service> {
    let service = service::Service::default();

    pb::lottery_service_server::LotteryServiceServer::new(service)
}