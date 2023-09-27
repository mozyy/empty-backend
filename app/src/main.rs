use config::ADDR;
use empty_utils::{
    diesel::db,
    errors::{Error, Result},
    tonic::server,
};
use proto::pb::{self};
use tokio::signal;
use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let oauth_state = oauth::Service::new().await?;

    let addr = ADDR.parse().map_err(Error::other)?;

    let oauth =
        pb::oauth::oauth::o_auth_service_server::OAuthServiceServer::new(oauth_state.clone());

    log::info!("start ...");
    let auth = auth::get_service().await?;
    let blog = blog::get_service();
    let (lottery, lottery_record) = lottery::get_service();
    let oss = oss::get_service();
    let (wx, wx_user) = wx::get_service();
    server()
        .layer(AsyncRequireAuthorizationLayer::new(oauth_state))
        // .layer(AuthLayer {})
        .add_service(auth)
        .add_service(blog)
        .add_service(lottery)
        .add_service(lottery_record)
        .add_service(oauth)
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
