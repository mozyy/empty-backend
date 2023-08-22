use config::ADDR;
use empty_utils::{
    diesel::db,
    errors::{Error, Result},
    tonic::server,
};
use proto::pb;
use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let oauth_state = oauth::Service::new().await?;

    let url = ADDR.parse().map_err(Error::other)?;

    let blog = pb::blog::blog::blog_service_server::BlogServiceServer::new(
        blog::service::Service::default(),
    );
    let db_lottery = db::DbPool::new("lottery_v2");
    let lottery = pb::lottery::lottery::lottery_service_server::LotteryServiceServer::new(
        lottery::service::lottery::Service::new_by_db(db_lottery.clone()),
    );
    let record = pb::lottery::record::record_service_server::RecordServiceServer::new(
        lottery::service::record::Service::new_by_db(db_lottery),
    );

    let oauth =
        pb::oauth::oauth::o_auth_service_server::OAuthServiceServer::new(oauth_state.clone());

    let db_wx = db::DbPool::new("wx_v2");
    let wx =
        pb::wx::wx::wx_service_server::WxServiceServer::new(wx::service::wx::Service::default());
    let user = pb::wx::user::user_service_server::UserServiceServer::new(
        wx::service::user::Service::new_by_db(db_wx),
    );
    log::info!("start ...");

    server()
        .layer(AsyncRequireAuthorizationLayer::new(oauth_state))
        // .layer(AuthLayer {})
        .add_service(lottery)
        .add_service(oauth)
        .add_service(record)
        .add_service(user)
        .add_service(wx)
        .add_service(blog)
        .serve(url)
        .await?;

    Ok(())
}
