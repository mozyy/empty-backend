use std::{collections::HashMap, net::SocketAddr};

use register::{
    register_service_server::RegisterService, Empty, GetRequest, LogoutRequest, RegisterRequest,
    Service,
};
use tokio::sync::Mutex;
use tonic::{transport::server::TcpConnectInfo, Code, Request, Response, Status};
pub mod register {
    tonic::include_proto!("register");
}

pub use register::register_service_server::RegisterServiceServer;

#[derive(Default)]
pub struct Services {
    serviceMap: Mutex<HashMap<String, Vec<Service>>>,
}

#[tonic::async_trait]
impl RegisterService for Services {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<Service>, Status> {
        dbg!(&request);
        if let Some(addr) = &request.remote_addr() {
            let mut service_map = self.serviceMap.lock().await;
            let service = Service {
                id: 1,
                endpoint: addr.to_string(),
                created_at: String::from("create_at"),
            };
            service_map
                .entry(request.into_inner().name)
                .or_insert_with(Vec::new)
                .push(service.clone());
            Ok(Response::new(service))
        } else {
            Err(Status::new(Code::InvalidArgument, "name is invalid"))
        }
    }
    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<Empty>, Status> {
        todo!()
    }
    async fn get_service(&self, request: Request<GetRequest>) -> Result<Response<Service>, Status> {
        todo!()
    }
}
