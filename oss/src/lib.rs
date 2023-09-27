use proto::pb::oss::oss::oss_service_server::OssServiceServer;

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> OssServiceServer<service::Service> {
    OssServiceServer::new(service::Service::default())
}
