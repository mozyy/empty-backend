use crate::pb::{
    registry_server::Registry as RegistryService, GetRequest, RegisterRequest, RegisterResponse, UnregisterRequest,
};
use crate::pb::{
    AllRequest, AllResponse, GetResponse, ListRequest, ListResponse, UnregisterResponse,
};

use empty_utils::tonic::{Resp, Response};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tonic::{Request, Status};

use super::model::Registry;

#[derive(Default)]
pub struct Service {
    pub registry: Arc<Mutex<Registry>>,
}
impl Service {
    pub async fn start() {
        loop {
            tokio::time::sleep(Duration::from_secs(5 * 60)).await;
        }
    }
    pub fn new(registry: Arc<Mutex<Registry>>) -> Self {
        Self { registry }
    }
}
#[tonic::async_trait]
impl RegistryService for Service {
    async fn register(&self, request: Request<RegisterRequest>) -> Resp<RegisterResponse> {
        let mut registry = self.registry.lock().unwrap();
        let request = request.into_inner();
        let req = request.clone();
        let id = registry.register_service(request.name, request.endpoint);
        log::info!("registry: {:?}", req);
        Response(RegisterResponse { id: id.to_string() }).into()
    }
    async fn unregister(&self, request: Request<UnregisterRequest>) -> Resp<UnregisterResponse> {
        let mut registry = self.registry.lock().unwrap();
        registry.unregister_service(request.into_inner().id.parse().unwrap());
        Response(UnregisterResponse {}).into()
    }
    async fn get(&self, request: Request<GetRequest>) -> Resp<GetResponse> {
        let mut registry = self.registry.lock().unwrap();
        let service = registry
            .get_service(request.into_inner().name)
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(GetResponse {
            service: Some(service.into()),
        })
        .into()
    }
    async fn list(&self, request: Request<ListRequest>) -> Resp<ListResponse> {
        let mut registry = self.registry.lock().unwrap();
        let services = registry
            .list_service(request.into_inner().name)
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(ListResponse {
            services: services.into_iter().map(|s| s.into()).collect(),
        })
        .into()
    }
    async fn all(&self, _request: Request<AllRequest>) -> Resp<AllResponse> {
        let mut registry = self.registry.lock().unwrap();
        let services = registry
            .all_service()
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(AllResponse {
            services: services.into_iter().map(|s| s.into()).collect(),
        })
        .into()
    }
}
