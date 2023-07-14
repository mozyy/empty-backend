use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use empty_utils::{
    diesel::db,
    errors::{Error, Result},
    tonic::server,
};
use lottery::{
    configs::ADDR,
    model::oauth::UserId,
    pb::{
        lottery::lottery_service_server::LotteryServiceServer,
        oauth::o_auth_service_server::OAuthServiceServer,
        user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
    },
    service::{self, oauth::handler},
};
use tonic::{body::BoxBody, codegen::empty_body};
use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();

    let db = db::DbPool::new("lottery");

    let oauth_state = service::oauth::Service::new_by_db(db.clone()).await?;
    let app = Router::new()
        .route("/authorize", get(handler::authorize_get))
        .route("/token", post(handler::token))
        .with_state(oauth_state.clone());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    axum::Server::bind(&addr).serve(app.into_make_service());

    let url = ADDR.parse().map_err(Error::other)?;

    let lottery = LotteryServiceServer::new(service::lottery::Service::new_by_db(db.clone()));
    let oauth = OAuthServiceServer::new(oauth_state.clone());
    // let record = RecordServiceServer::new(service::record::Service::new_by_db(db));
    let user = UserServiceServer::new(service::user::Service::new_by_db(db.clone()));
    let wx = WxServiceServer::new(service::wx::Service::default());

    server()
        .layer(AsyncRequireAuthorizationLayer::new(oauth_state))
        // .layer(AuthLayer {})
        .add_service(lottery)
        .add_service(oauth)
        // .add_service(record)
        .add_service(user)
        .add_service(wx)
        .serve(url)
        .await?;

    Ok(())
}

use futures_util::future::BoxFuture;
use http::StatusCode;
use hyper::{Body, Request, Response};
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Clone)]
struct MyAuth;

impl<B> AsyncAuthorizeRequest<B> for MyAuth
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let that = self.clone();
        Box::pin(async {
            if let Some(user_id) = check_auth(&request).await {
                // Set `user_id` as a request extension so it can be accessed by other
                // services down the stack.
                request.extensions_mut().insert(user_id);
                let _s = that;
                Ok(request)
            } else {
                let unauthorized_response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(empty_body())
                    .unwrap();

                Err(unauthorized_response)
            }
        })
    }
}

async fn check_auth<B>(_request: &Request<B>) -> Option<UserId> {
    // ...
    todo!()
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Access the `UserId` that was set in `on_authorized`. If `handle` gets called
    // request was authorized and `UserId` will be present.
    let _user_id = request
        .extensions()
        .get::<UserId>()
        .expect("UserId will be there if request was authorized");

    // println!("request from {:?}", user_id);

    Ok(Response::new(Body::empty()))
}
