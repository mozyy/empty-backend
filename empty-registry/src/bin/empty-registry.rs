use axum::{routing::any, Router};
use empty_registry::{
    get_registry_addr,
    heartbeat::Service as HeartbeatService,
    proxy::Proxy,
    registry::{model::Registry, service::Service as RegistryService},
    HeartbeatServer, RegistryServer,
};
use hyper::{client::HttpConnector, Body};
use std::sync::{Arc, Mutex};
use tonic::transport::NamedService;
use tonic_health::{
    pb::health_server::HealthServer,
    server::{health_reporter, HealthService},
};

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    empty_utils::init();
    let (_health_reporter, health_service) = health_reporter();

    let addr = get_registry_addr().parse().unwrap();

    let registry = Arc::new(Mutex::new(Registry::default()));

    let registry_service = RegistryService::new(registry.clone());
    let heartbeat_service = HeartbeatService;

    log::info!("RegistryServer listening on {}", addr);

    let proxy = Proxy::new(registry.clone());

    let registry_service = Router::new()
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .route_service(
            &format!("/{}/*rest", RegistryServer::<RegistryService>::NAME),
            RegistryServer::new(registry_service),
        )
        .route_service(
            &format!("/{}/*rest", HeartbeatServer::<HeartbeatService>::NAME),
            HeartbeatServer::new(heartbeat_service),
        )
        .route_service(
            &format!("/{}/*rest", HealthServer::<HealthService>::NAME),
            health_service,
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
