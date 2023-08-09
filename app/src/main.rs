use axum::Router;
use config::ADDR;
use empty_utils::{
    diesel::db,
    errors::{Error, Result},
    tonic::{self, server},
};
use proto::pb;
use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let db = db::DbPool::new("lottery");

    let oauth_state = oauth::Service::new_by_db(db.clone()).await?;

    let url = ADDR.parse().map_err(Error::other)?;

    let lottery = pb::lottery::lottery::lottery_service_server::LotteryServiceServer::new(
        lottery::service::lottery::Service::new_by_db(db.clone()),
    );
    let oauth =
        pb::oauth::oauth::o_auth_service_server::OAuthServiceServer::new(oauth_state.clone());
    let record = pb::lottery::record::record_service_server::RecordServiceServer::new(
        lottery::service::record::Service::new_by_db(db.clone()),
    );
    let user = pb::wx::user::user_service_server::UserServiceServer::new(
        lottery::service::user::Service::new_by_db(db.clone()),
    );
    let wx = pb::wx::wx::wx_service_server::WxServiceServer::new(
        lottery::service::wx::Service::default(),
    );

    server()
        .layer(AsyncRequireAuthorizationLayer::new(oauth_state))
        // .layer(AuthLayer {})
        .add_service(lottery)
        .add_service(oauth)
        .add_service(record)
        .add_service(user)
        .add_service(wx)
        .serve(url)
        .await?;

    Ok(())
}
