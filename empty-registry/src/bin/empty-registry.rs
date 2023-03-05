use axum::{
    extract::{FromRequestParts, Path, State},
    http::{uri::Uri, Request, Response},
    routing::{any, get, post},
    Router,
};
use empty_registry::{
    proxy::{handler, Proxy},
    registry::RegistryServer,
    RegistryServiceServer, REGISTRY_ADDR,
};
use hyper::{client::HttpConnector, Body};
use std::{borrow::Borrow, convert::Infallible};
use tonic::transport::NamedService;
use tower::service_fn;
type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    empty_utils::init();
    let (_health_reporter, health_service) = tonic_health::server::health_reporter();

    let addr = REGISTRY_ADDR.parse().unwrap();
    let service = RegistryServer::default();

    log::info!("RegistryServer listening on {}", addr);

    let proxy = Proxy::default();

    let registry_service = Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .route_service(
            &format!("/{}/*rest", RegistryServiceServer::<RegistryServer>::NAME),
            RegistryServiceServer::new(service),
        )
        .route(
            "/t/:service/*rest",
            get(Proxy::handler).with_state(proxy.clone()),
        )
        .route("/:service/*rest", any(Proxy::handler).with_state(proxy))
        .route_service(
            "/:service",
            service_fn(|req: Request<Body>| async move {
                let method = req.method().clone();
                log::info!("path: {:?}", &req);
                let mut part = req.into_parts().0;
                let path = Path::<String>::from_request_parts(&mut part, &())
                    .await
                    .unwrap();
                log::info!("path: {:?}", path);
                let body = Body::from(format!("Hi from `{} /foo`", method));
                let body = axum::body::boxed(body);
                let res = Response::new(body);
                Ok::<_, Infallible>(res)
            }),
        );

    // TODO: proxy_service
    // TODO: oauth_service
    axum::Server::bind(&addr)
        .serve(registry_service.into_make_service())
        .await
        .unwrap();

    Ok(())
}
