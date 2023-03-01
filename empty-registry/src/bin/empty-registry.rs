use empty_registry::{RegistryServiceServer, REGISTRY_ADDR};
use tonic::transport::{NamedService, Server};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    empty_utils::init();
    let (_health_reporter, health_service) = tonic_health::server::health_reporter();

    let addr = REGISTRY_ADDR.parse().unwrap();
    let service = empty_registry::server::Server::default();

    log::info!("RegistryServer listening on {}", addr);

    Server::builder()
        // .trace_fn(|request| {
        //     log::info!("resive request: {:?}", request);
        //     tracing::info_span!("registry_server", "{:?}", request)
        // })
        .layer(TraceLayer::new_for_http())
        // TODO: helth_service
        .add_service(health_service)
        .add_service(RegistryServiceServer::new(service))
        // TODO: proxy_service
        // TODO: oauth_service
        .serve(addr)
        .await?;

    Ok(())
}
