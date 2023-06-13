pub mod model;
pub mod schema;
pub mod service;
pub mod types;
pub mod pb {
    pub mod lottery {
        tonic::include_proto!("lottery");
    }
    pub mod user {
        tonic::include_proto!("user");
    }
}

pub fn new() -> pb::lottery::lottery_service_server::LotteryServiceServer<service::lottery::Service> {
    let service = service::lottery::Service::default();

    pb::lottery::lottery_service_server::LotteryServiceServer::new(service)
}
