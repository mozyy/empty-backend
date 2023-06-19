use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use empty_utils::{errors::ServiceResult, tonic::server};
use lottery::{
    configs::ADDR,
    pb::{
        self, lottery::lottery_service_server::LotteryServiceServer,
        record::record_service_server::RecordServiceServer,
        user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
    },
    service::{
        self,
        oauth::{handler, state::State},
    },
};

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let app = Router::new()
        .route("/authorize", get(handler::authorize_get))
        .route("/token", post(handler::token))
        .with_state(State::new());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

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
