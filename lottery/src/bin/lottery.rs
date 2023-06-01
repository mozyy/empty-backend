use empty_utils::{errors::ServiceResult, tonic::server};
use lottery::{pb::lottery_service_server::LotteryServiceServer, service::Service};
use tonic::transport::NamedService;

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = Service::default();

    log::info!(
        "GreeterServer listening on {}, name: {}",
        addr,
        LotteryServiceServer::<Service>::NAME
    );

    server()
        .add_service(LotteryServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
