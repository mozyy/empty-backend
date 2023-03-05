use axum::{
    extract::{Path, State},
    http::{uri::Uri, Request, Response},
    routing::{any, post},
    Router,
};
use empty_registry::{
    proxy::{handler, Proxy},
    registry::RegistryServer,
    RegistryServiceServer, REGISTRY_ADDR,
};
use hyper::{client::HttpConnector, Body};
use tonic::transport::NamedService;

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
        .route_service(
            &format!("/{}/*rest", RegistryServiceServer::<RegistryServer>::NAME),
            RegistryServiceServer::new(service),
        )
        .route("/:service/*rest", any(Proxy::handler).with_state(proxy));

    // TODO: proxy_service
    // TODO: oauth_service
    axum::Server::bind(&addr)
        .serve(registry_service.into_make_service())
        .await
        .unwrap();

    Ok(())
}
