use crate::registry::{health_service_client::HealthServiceClient, ListRequest};
use crate::schema::micro_services;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

use crate::registry::{
    self, registry_service_server::RegistryService, GetRequest, LoginRequest, MicroService,
    MicroServices, RegisterRequest, UnregisterRequest,
};
use empty_utils::{
    diesel::db,
    tonic::{Resp, Response},
};
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

struct Registry {
    dbPool: db::DbPool,
}
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Insertable)]
#[diesel(table_name = micro_services)]
struct NewService {
    name: String,
    endpoint: String,
}
#[derive(Queryable)]
#[diesel(table_name = micro_services)]
struct Service {
    id: Uuid,
    name: String,
    endpoint: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Service> for MicroService {
    fn from(value: Service) -> Self {
        value.into()
    }
}

impl Registry {
    fn new() -> Self {
        Registry { dbPool: db::get() }
    }

    fn register_service(&mut self, name: String, endpoint: String) {
        let service = NewService { name, endpoint };
        let mut conn = self.dbPool.get().unwrap();
        diesel::insert_into(micro_services::dsl::micro_services)
            .values(service)
            .execute(&mut conn)
            .unwrap();
    }

    fn unregister_service(&mut self, id: Uuid) {
        let mut conn = self.dbPool.get().unwrap();
        diesel::delete(micro_services::dsl::micro_services.find(id))
            .execute(&mut conn)
            .unwrap();
    }

    fn get_service(&mut self, name: String) -> Option<Service> {
        let mut conn = self.dbPool.get().unwrap();
        micro_services::dsl::micro_services
            .filter(micro_services::name.eq(name))
            .first::<Service>(&mut conn)
            .ok()
    }
}

#[derive(Default)]
pub struct Server {
    registry: std::sync::Mutex<Registry>,
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
        let remote_addr = request.remote_addr();
        let mut registry = self.registry.lock().unwrap();
        registry.register_service(request.into_inner().name, remote_addr.unwrap().to_string());
        Response(()).into()
    }
    async fn unregister(&self, request: Request<UnregisterRequest>) -> Resp<()> {
        let mut registry = self.registry.lock().unwrap();
        registry.unregister_service(request.into_inner().id.parse().unwrap());
        Response(()).into()
    }
    async fn get(&self, request: Request<GetRequest>) -> Resp<MicroService> {
        let mut registry = self.registry.lock().unwrap();
        let service = registry
            .get_service(request.into_inner().name)
            .ok_or(Status::out_of_range("未找到服务"))?;
        Ok(tonic::Response::new(service.into()))
    }
    async fn list(&self, request: Request<ListRequest>) -> Resp<MicroServices> {
        todo!()
    }
    async fn all(&self, request: Request<()>) -> Resp<MicroServices> {
        todo!()
    }
}
