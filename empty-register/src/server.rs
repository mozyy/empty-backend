use crate::register::health_service_client::HealthServiceClient;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use crate::register::{
    register_service_server::RegisterService, GetRequest, LoginRequest, LogoutRequest,
    RegisterRequest, Service, Services,
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
    serviceMap: Mutex<HashMap<String, Vec<Service>>>,
}
impl Server {
    pub async fn start() {
        loop {
            tokio::time::sleep(Duration::from_secs(5 * 60)).await;
        }
    }
}
#[tonic::async_trait]
impl RegisterService for Server {
    async fn register(&self, request: Request<RegisterRequest>) -> Resp<()> {
        todo!()
    }
    async fn login(&self, request: Request<LoginRequest>) -> Resp<()> {
        dbg!(&request);
        if let Some(addr) = &request.remote_addr() {
            let mut service_map = self.serviceMap.lock().await;
            let service = Service {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                endpoint: addr.to_string(),
                created_at: Some(SystemTime::now().into()),
            };
            service_map
                .entry(service.name.clone())
                .or_insert_with(Vec::new)
                .push(service);
            Response(()).into()
        } else {
            Err(Status::new(Code::InvalidArgument, "name is invalid"))
        }
    }
    async fn logout(&self, request: Request<LogoutRequest>) -> Resp<()> {
        todo!()
    }
    async fn get(&self, request: Request<GetRequest>) -> Resp<Service> {
        todo!()
    }
    async fn list(&self, request: Request<()>) -> Resp<Services> {
        todo!()
    }
}
