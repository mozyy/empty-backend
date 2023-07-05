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
        oauth::o_auth_service_server::OAuthServiceServer,
        user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
    },
    service::{
        self,
        oauth::{handler, state::State},
    },
    utils::AuthLayer,
};
use tonic::{body::BoxBody, codegen::empty_body};
use tower_http::auth::AsyncRequireAuthorizationLayer;

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let app = Router::new()
        .route("/authorize", get(handler::authorize_get))
        .route("/token", post(handler::token))
        .with_state(State::new());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    axum::Server::bind(&addr).serve(app.into_make_service());

    let url = ADDR.parse().unwrap();
    let oauthState = service::oauth::Service::default();
    let lottery = LotteryServiceServer::new(service::lottery::Service::default());
    let oauth = OAuthServiceServer::new(oauthState.clone());
    // let record = RecordServiceServer::new(service::record::Service::default());
    let user = UserServiceServer::new(service::user::Service::default());
    let wx = WxServiceServer::new(service::wx::Service::default());
    let mut oauthState2 = oauthState.clone();
    let mut handler = |mut request: hyper::Request<hyper::Body>| async move {
        let authorized = request
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|it| it.to_str().ok())
            .and_then(|it| it.strip_prefix("Bearer "))
            .unwrap_or("test");
        let authorized = authorized.to_owned();
        request.extensions_mut().insert(authorized);
        // request.extensions_mut().insert(12345);
        Ok(request)
        // let resp = oauthState
        //     .clone()
        //     .check_resource(pb::oauth::ResourceRequest {
        //         auth,
        //         uri: "".into(),
        //     })
        //     .await;
        // match resp {
        //     Ok(resp) => todo!(),
        //     Err(err) => Err(err.to_owned().to_http()),
        // }
    };
    server()
        .layer(AsyncRequireAuthorizationLayer::new(handler))
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
use http::{header::AUTHORIZATION, StatusCode};
use hyper::{Body, Error, Request, Response};
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
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
                let s = that;
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

async fn check_auth<B>(request: &Request<B>) -> Option<UserId> {
    // ...
    todo!()
}

#[derive(Debug)]
struct UserId(String);

async fn handle(request: Request<Body>) -> Result<Response<Body>, Error> {
    // Access the `UserId` that was set in `on_authorized`. If `handle` gets called
    // request was authorized and `UserId` will be present.
    let user_id = request
        .extensions()
        .get::<UserId>()
        .expect("UserId will be there if request was authorized");

    println!("request from {:?}", user_id);

    Ok(Response::new(Body::empty()))
}
