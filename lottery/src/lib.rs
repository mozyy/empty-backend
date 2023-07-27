pub mod configs;
pub mod demo;
pub mod model;
pub mod pb;
pub(crate) mod schema;
pub mod service;
pub mod types;
pub mod utils;

pub fn new() -> pb::lottery::lottery_service_server::LotteryServiceServer<service::lottery::Service>
{
    let service = service::lottery::Service::default();

    pb::lottery::lottery_service_server::LotteryServiceServer::new(service)
}
