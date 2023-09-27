use config::{ADDR, ADDR_CLIENT};
use empty_utils::{
    diesel::db,
    errors::{Error, Result},
    tonic::server,
};
use futures_util::future::BoxFuture;
use http::StatusCode;
use hyper::{Body, Request, Response};
use proto::pb::{self};
use std::future::Future;
use tokio::signal;
use tonic::{body::BoxBody, codegen::empty_body};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let addr = ADDR.parse().map_err(Error::other)?;

    log::info!("start ...");
    let (auth, auth_state) = auth::get_service().await?;
    let blog = blog::get_service();
    let (lottery, lottery_record) = lottery::get_service();
    let oss = oss::get_service();
    let (wx, wx_user) = wx::get_service();
    server()
        .layer(AsyncRequireAuthorizationLayer::new(auth_state))
        // .layer(AuthLayer {})
        .add_service(auth)
        .add_service(blog)
        .add_service(lottery)
        .add_service(lottery_record)
        // .add_service(oauth)
        .add_service(oss)
        .add_service(wx)
        .add_service(wx_user)
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log::info!("signal received, starting graceful shutdown");
}
