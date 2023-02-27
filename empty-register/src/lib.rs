use std::{collections::HashMap, net::SocketAddr};

use empty_utils::tonic::{Resp, Response};
use register::{
    register_service_server::RegisterService, GetRequest, LoginRequest, LogoutRequest,
    RegisterRequest, Service, Services,
};
use tokio::sync::Mutex;
use tonic::{transport::server::TcpConnectInfo, Code, Request, Status};
use tonic_health::server::HealthReporter;
pub mod register {
    tonic::include_proto!("empty.register.v1");
}

pub use register::register_service_server::RegisterServiceServer;
pub mod client;

#[derive(Default)]
pub struct Server {
    serviceMap: Mutex<HashMap<String, Vec<Service>>>,
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
                id: String::from("create_at"),
                endpoint: addr.to_string(),
                created_at: String::from("create_at"),
            };
            service_map
                .entry(request.into_inner().name)
                .or_insert_with(Vec::new)
                .push(service.clone());
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
