use crate::pb::Service as PBService;
use crate::schema::micro_services;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use empty_utils::convert::naive_date_time_to_timestamp;
use empty_utils::diesel::db;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Clone)]
pub struct RegistryDB {
    db_pool: db::DbPool,
}
impl Default for RegistryDB {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Insertable)]
#[diesel(table_name = micro_services)]
struct NewMicroService {
    name: String,
    endpoint: String,
}
#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = micro_services)]
pub struct MicroService {
    pub id: Uuid,
    pub name: String,
    pub endpoint: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<MicroService> for PBService {
    fn from(value: MicroService) -> Self {
        PBService {
            id: value.id.to_string(),
            name: value.name,
            endpoint: value.endpoint,
            created_at: Some(naive_date_time_to_timestamp(value.created_at)),
            updated_at: Some(naive_date_time_to_timestamp(value.updated_at)),
        }
    }
}

impl RegistryDB {
    fn new() -> Self {
        RegistryDB { db_pool: db::get() }
    }

    pub fn register_service(&mut self, name: String, endpoint: String) {
        let service = NewMicroService { name, endpoint };
        let mut conn = self.db_pool.get().unwrap();
        diesel::insert_into(micro_services::dsl::micro_services)
            .values(service)
            .execute(&mut conn)
            .unwrap();
    }

    pub fn unregister_service(&mut self, id: Uuid) {
        let mut conn = self.db_pool.get().unwrap();
        diesel::delete(micro_services::dsl::micro_services.find(id))
            .execute(&mut conn)
            .unwrap();
    }

    pub fn get_service(&mut self, name: String) -> Option<MicroService> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .filter(micro_services::name.eq(name))
            .first::<MicroService>(&mut conn)
            .ok()
    }
    pub fn list_service(&mut self, name: String) -> Option<Vec<MicroService>> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .filter(micro_services::name.eq(name))
            .load::<MicroService>(&mut conn)
            .ok()
    }
    pub fn all_service(&mut self) -> Option<Vec<MicroService>> {
        let mut conn = self.db_pool.get().unwrap();
        micro_services::dsl::micro_services
            .load::<MicroService>(&mut conn)
            .ok()
    }
}

#[derive(Clone, Default)]
pub struct Registry {
    services: Arc<Mutex<HashMap<String, Vec<MicroService>>>>,
}

impl Registry {
    pub fn register_service(&mut self, name: String, endpoint: String) -> Uuid {
        let id = Uuid::new_v4();
        let service = MicroService {
            id,
            name: name.clone(),
            endpoint,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let mut services = self.services.lock().unwrap();
        services
            .entry(name)
            .and_modify(|s| s.push(service.clone()))
            .or_insert_with(|| vec![service]);
        id
    }

    pub fn unregister_service(&mut self, id: Uuid) {
        let mut services = self.services.lock().unwrap();
        let item = services
            .clone()
            .into_iter()
            .find(|s| s.1.iter().any(|i| i.id == id));
        if let Some((key, value)) = item {
            if value.len() <= 1 {
                services.remove(&key);
                return;
            }
            if let Some(value) = services.get_mut(&key) {
                value.retain(|s| s.id != id);
            }
        }
    }

    pub fn get_service(&mut self, name: String) -> Option<MicroService> {
        let services = self.services.lock().unwrap();
        let services = services.get(&name);
        if let Some(services) = services {
            return services.get(0).cloned();
        }
        None
    }
    pub fn list_service(&mut self, name: String) -> Option<Vec<MicroService>> {
        let services = self.services.lock().unwrap();
        services.get(&name).cloned()
    }
    pub fn all_service(&mut self) -> Option<Vec<MicroService>> {
        let services = self.services.lock().unwrap();
        let services = services
            .clone()
            .into_iter()
            .flat_map(|(_k, v)| v)
            .collect::<Vec<_>>();
        Some(services)
    }
}
