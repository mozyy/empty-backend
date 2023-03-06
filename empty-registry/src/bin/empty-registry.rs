use axum::{
    extract::{FromRequestParts, Path},
    http::{Request, Response},
    routing::{any, get},
    Router,
};
use empty_registry::{
    proxy::Proxy,
    registry::{self, model::Registry, service::Service},
    RegistryServiceServer, REGISTRY_ADDR,
};
use hyper::{client::HttpConnector, Body};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tonic::transport::NamedService;
use tower::service_fn;
type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    empty_utils::init();
    let (_health_reporter, _health_service) = tonic_health::server::health_reporter();

    let addr = REGISTRY_ADDR.parse().unwrap();
    let registry = Arc::new(Mutex::new(Registry::default()));
    let service = Service::new(registry.clone());

    log::info!("RegistryServer listening on {}", addr);

    let proxy = Proxy::new(registry.clone());

    let registry_service = Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .route_service(
            &format!("/{}/*rest", RegistryServiceServer::<Service>::NAME),
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
