use axum::{routing::any, Router};
use empty_registry::{
    get_registry_addr,
    proxy::Proxy,
    registry::{model::Registry, service::Service},
    RegistryServiceServer,
};
use hyper::{client::HttpConnector, Body};
use std::sync::{Arc, Mutex};
use tonic::transport::NamedService;

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    empty_utils::init();
    let (_health_reporter, _health_service) = tonic_health::server::health_reporter();

    let addr = get_registry_addr().parse().unwrap();

    let registry = Arc::new(Mutex::new(Registry::default()));

    let service = Service::new(registry.clone());

    log::info!("RegistryServer listening on {}", addr);

    let proxy = Proxy::new(registry.clone());

    let registry_service = Router::new()
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .route_service(
            &format!("/{}/*rest", RegistryServiceServer::<Service>::NAME),
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
