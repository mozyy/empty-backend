use std::convert::Infallible;

use hyper::{Body, Request, Response};
use tokio::time::{sleep, Duration};
use tonic::{
    body::BoxBody,
    transport::{NamedService, Server},
};
use tower::Service;

use crate::{
    get_registry_addr,
    pb::{
        heartbeat_client::HeartbeatClient, registry_client::RegistryClient, HeartbeatRequest,
        RegisterRequest,
    },
};

pub async fn register<S>(svc: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: Service<Request<Body>, Response = Response<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
    let local_addr = listener.local_addr()?;
    log::info!("RegistryServer listening on {}", local_addr);

    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let registry_addr = get_registry_addr();

    let mut client_registry = RegistryClient::connect(format!("http://{registry_addr}")).await?;
    let mut client_heartbeat = HeartbeatClient::connect(format!("http://{registry_addr}")).await?;
    let service_name = S::NAME.to_string();

    let request = tonic::Request::new(RegisterRequest {
        name: service_name.clone(),
        endpoint: local_addr.to_string(),
    });

    let response = client_registry.register(request).await?;
    let request = HeartbeatRequest {
        name: service_name,
        id: response.into_inner().id,
    };

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            client_heartbeat
                .heartbeat(tonic::Request::new(request.clone()))
                .await
                .unwrap();
        }
    });

    Server::builder()
        // .trace_fn(|request| {
        //     log::info!("resive request: {:?}", request);
        //     tracing::info_span!("registry_server", "{:?}", request)
        // })
        // .layer(TraceLayer::new_for_http())
        // TODO: helth_service
        // .add_service(health_service)
        .add_service(svc)
        .serve_with_incoming(incoming)
        .await?;
    Ok(())
}
