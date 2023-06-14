use empty_utils::{errors::ServiceResult, tonic::server};
use lottery::{service, pb};

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let addr = "0.0.0.0:50051".parse().unwrap();
    let lottery =  pb::lottery::lottery_service_server::LotteryServiceServer::new(service::lottery::Service::default());
    let user = pb::user::user_service_server::UserServiceServer::new(service::user::Service::default());
    server()
    .add_service(lottery)
    .add_service(user)
    .serve(addr)
    .await?;

    Ok(())
}
