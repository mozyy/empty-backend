use blog::pb::blog_service_server::BlogServiceServer;
use blog::service::Service;
use empty_utils::{
    errors::{Error, Result},
    tonic::server,
};
use tonic::transport::NamedService;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let addr = "0.0.0.0:50051".parse().map_err(Error::other)?;
    let greeter = Service::default();

    log::info!(
        "GreeterServer listening on {}, name: {}",
        addr,
        BlogServiceServer::<Service>::NAME
    );

    server()
        .add_service(BlogServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
