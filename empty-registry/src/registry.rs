use crate::pb::{health_service_client::HealthServiceClient, ListRequest};
use crate::pb::{
    registry_service_server::RegistryService, GetRequest, MicroService, MicroServices,
    RegisterRequest, UnregisterRequest,
};
use crate::schema::micro_services;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::convert::naive_date_time_to_timestamp;
use empty_utils::{
    diesel::db,
    tonic::{Resp, Response},
};
use std::time::Duration;
use tonic::{Request, Status};
use uuid::Uuid;

pub async fn health_check(dts: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HealthServiceClient::connect(dts).await?;

    let request = tonic::Request::new(());

    let response = client.health_check(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[derive(Clone)]
pub struct Registry {
    db_pool: db::DbPool,
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
#[derive(Queryable, Debug)]
#[diesel(table_name = micro_services)]
pub struct Service {
    id: Uuid,
    name: String,
    pub endpoint: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Service> for MicroService {
    fn from(value: Service) -> Self {
        MicroService {
            id: value.id.to_string(),
            name: value.name,
            endpoint: value.endpoint,
            created_at: Some(naive_date_time_to_timestamp(value.created_at)),
            updated_at: Some(naive_date_time_to_timestamp(value.updated_at)),
        }
    }
}
impl From<Vec<Service>> for MicroServices {
    fn from(value: Vec<Service>) -> Self {
        let services: Vec<MicroService> = value.into_iter().map(MicroService::from).collect();
        MicroServices { services }
    }
}

impl Registry {
    fn new() -> Self {
        Registry { db_pool: db::get() }
    }

    fn register_service(&mut self, name: String, endpoint: String) {
        let service = NewService { name, endpoint };
        let mut conn = self.db_pool.get().unwrap();
        diesel::insert_into(micro_services::dsl::micro_services)
            .values(service)
            .execute(&mut conn)
            .unwrap();
    }

    fn unregister_service(&mut self, id: Uuid) {
        let mut conn = self.db_pool.get().unwrap();
        diesel::delete(micro_services::dsl::micro_services.find(id))
            .execute(&mut conn)
            .unwrap();
    }

    fn get_service(&mut self, name: String) -> Option<Service> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .filter(micro_services::name.eq(name))
            .first::<Service>(&mut conn)
            .ok()
    }
    pub fn list_service(&mut self, name: String) -> Option<Vec<Service>> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .filter(micro_services::name.eq(name))
            .load::<Service>(&mut conn)
            .ok()
    }
    fn all_service(&mut self) -> Option<Vec<Service>> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .load::<Service>(&mut conn)
            .ok()
    }
}

#[derive(Default)]
pub struct RegistryServer {
    registry: std::sync::Mutex<Registry>,
}
impl RegistryServer {
    pub async fn start() {
        loop {
            tokio::time::sleep(Duration::from_secs(5 * 60)).await;
        }
    }
}
#[tonic::async_trait]
impl RegistryService for RegistryServer {
    async fn register(&self, request: Request<RegisterRequest>) -> Resp<()> {
        let mut registry = self.registry.lock().unwrap();
        let request = request.into_inner();
        registry.register_service(request.name, request.endpoint);
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
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(service.into()).into()
    }
    async fn list(&self, request: Request<ListRequest>) -> Resp<MicroServices> {
        let mut registry = self.registry.lock().unwrap();
        let services = registry
            .list_service(request.into_inner().name)
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(services.into()).into()
    }
    async fn all(&self, _request: Request<()>) -> Resp<MicroServices> {
        let mut registry = self.registry.lock().unwrap();
        let services = registry
            .all_service()
            .ok_or_else(|| Status::out_of_range("未找到服务"))?;
        Response(services.into()).into()
    }
}
