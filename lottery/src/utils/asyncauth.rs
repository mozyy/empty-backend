use tower_http::auth::{AsyncRequireAuthorizationLayer, AsyncAuthorizeRequest};
use hyper::{Request, Response, Body, Error};
use http::{StatusCode, header::AUTHORIZATION};
use tower::{Service, ServiceExt, ServiceBuilder, service_fn};
use futures_util::future::BoxFuture;

#[derive(Clone, Copy)]
struct MyAuth;

impl<B> AsyncAuthorizeRequest<B> for MyAuth
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        Box::pin(async {
            if let Some(user_id) = check_auth(&request).await {
                // Set `user_id` as a request extension so it can be accessed by other
                // services down the stack.
                request.extensions_mut().insert(user_id);

                Ok(request)
            } else {
                let unauthorized_response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();

                Err(unauthorized_response)
            }
        })
    }
}

async fn check_auth<B>(request: &Request<B>) -> Option<UserId> {
    todo!()
    // ...
}

#[derive(Debug)]
struct UserId(String);

async fn handle(request: Request<Body>) -> Result<Response<Body>, Error> {
    // Access the `UserId` that was set in `on_authorized`. If `handle` gets called the
    // request was authorized and `UserId` will be present.
    let user_id = request
        .extensions()
        .get::<UserId>()
        .expect("UserId will be there if request was authorized");

    println!("request from {:?}", user_id);

    Ok(Response::new(Body::empty()))
}