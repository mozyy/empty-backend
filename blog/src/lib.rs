use proto::pb::blog::blog::blog_service_server::BlogServiceServer;

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> BlogServiceServer<service::Service> {
    BlogServiceServer::new(service::Service::default())
}
