use empty_utils::{errors::ServiceResult, tonic::server};
use lottery::{pb::lottery_service_server::LotteryServiceServer, service::Service, new};
use tonic::transport::NamedService;

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let addr = "0.0.0.0:50051".parse().unwrap();

    server()
        .add_service(lottery::new())
        .serve(addr)
        .await?;

    Ok(())
}
