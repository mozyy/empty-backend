pub mod model;
pub mod schema;
pub mod service;

pub mod pb {
    tonic::include_proto!("blog");
}

pub fn new () -> pb::blog_service_server::BlogServiceServer<service::Service>{
    let service = service::Service::default();
    pb::blog_service_server::BlogServiceServer::new(service)
}