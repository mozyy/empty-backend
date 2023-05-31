// use blog::service::Service;
use empty_utils::{errors::ServiceResult, tonic::server};
// use proto::pb::blog::blog_service_server::BlogServiceServer;
// use tonic::transport::{NamedService, Server};

#[tokio::main]
async fn main() -> ServiceResult {
    // empty_utils::init();

    // let addr = "0.0.0.0:50051".parse().unwrap();
    // let greeter = Service::default();

    // log::info!(
    //     "GreeterServer listening on {}, name: {}",
    //     addr,
    //     BlogServiceServer::<Service>::NAME
    // );

    // server()
    //     .add_service(BlogServiceServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
