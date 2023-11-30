use config::ADDR;
use empty_utils::{
    errors::{Error, Result},
    tonic::server,
};

use tokio::signal;

use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let addr = ADDR.parse().map_err(Error::other)?;

    log::info!("start ...");
    let (check, user, user_auth) = user::get_service().await?;
    let blog = blog::get_service();
    let (lottery, lottery_record, lottery_favorite, lottery_template) = lottery::get_service();
    let oss = oss::get_service();
    let wx = wx::get_service();
    server()
        .layer(AsyncRequireAuthorizationLayer::new(check))
        // .layer(AuthLayer {})
        .add_service(blog)
        .add_service(lottery)
        .add_service(lottery_record)
        .add_service(lottery_favorite)
        .add_service(lottery_template)
        // .add_service(oauth)
        .add_service(oss)
        .add_service(wx)
        .add_service(user)
        .add_service(user_auth)
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
