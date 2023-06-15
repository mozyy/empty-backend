use std::net::SocketAddr;

use empty_utils::{errors::ServiceResult, tonic::server};
use lottery::{
    configs::ADDR,
    pb::{
        self, lottery::lottery_service_server::LotteryServiceServer,
        user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer, record::record_service_server::RecordServiceServer,
    },
    service,
};

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let url = ADDR.parse().unwrap();
    let lottery = LotteryServiceServer::new(service::lottery::Service::default());
    // let record = RecordServiceServer::new(service::record::Service::default());
    let user = UserServiceServer::new(service::user::Service::default());
    let wx = WxServiceServer::new(service::wx::Service::default());
    server()
        .add_service(lottery)
        // .add_service(record)
        .add_service(user)
        .add_service(wx)
        .serve(url)
        .await?;

    Ok(())
}
