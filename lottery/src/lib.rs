pub mod demo;
pub mod model;
pub mod service;

use proto::pb;

pub fn new(
) -> pb::lottery::lottery::lottery_service_server::LotteryServiceServer<service::lottery::Service> {
    let service = service::lottery::Service::default();

    pb::lottery::lottery::lottery_service_server::LotteryServiceServer::new(service)
}
