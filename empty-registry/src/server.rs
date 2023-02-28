use crate::registry::{health_service_client::HealthServiceClient, ListRequest};
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use crate::registry::{
    registry_service_server::RegistryService, GetRequest, LoginRequest, MicroService,
    MicroServices, RegisterRequest, UnregisterRequest,
};
use empty_utils::tonic::{Resp, Response};
use tokio::sync::Mutex;
use tonic::{codegen::http::request, transport::server::TcpConnectInfo, Code, Request, Status};
use tonic_health::server::HealthReporter;
pub async fn health_check(dts: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HealthServiceClient::connect(dts).await?;

    let request = tonic::Request::new(());

    let response = client.health_check(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[derive(Default)]
pub struct Server {
    serviceMap: Mutex<HashMap<String, Vec<MicroService>>>,
}
impl Server {
    pub async fn start() {
        loop {
            tokio::time::sleep(Duration::from_secs(5 * 60)).await;
        }
    }
}
#[tonic::async_trait]
impl RegistryService for Server {
    async fn register(&self, request: Request<RegisterRequest>) -> Resp<()> {
        todo!()
    }
    async fn unregister(&self, request: Request<UnregisterRequest>) -> Resp<()> {
        todo!()
    }
    async fn get(&self, request: Request<GetRequest>) -> Resp<MicroService> {
        todo!()
    }
    async fn list(&self, request: Request<ListRequest>) -> Resp<MicroServices> {
        todo!()
    }
    async fn all(&self, request: Request<()>) -> Resp<MicroServices> {
        todo!()
    }
}
