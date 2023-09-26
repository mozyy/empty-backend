use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

use empty_utils::{
    diesel::db,
    errors::{ErrorConvert, Result},
    tonic::Resp,
};
use tonic::{Request, Response};

use crate::{
    dao,
    model::{config::Config, resource::Resource},
};
use proto::pb;

pub struct Service {
    pub(super) db: db::DbPool,
    resources: Arc<Mutex<Resource>>,
    configs: Arc<Mutex<Vec<Config>>>,
}

impl Service {
    pub async fn new() -> Result<Self> {
        let db = db::DbPool::new("auth_v2");
        Self::new_by_db(db).await
    }
    pub async fn new_by_db(db: db::DbPool) -> Result<Self> {
        let mut value = Self {
            db,
            resources: Arc::new(Mutex::new(Default::default())),
            configs: Arc::new(Mutex::new(vec![])),
        };
        value.refresh_configs().await?;
        value.refresh_resources().await?;
        Ok(value)
    }
    pub async fn refresh_configs(&mut self) -> Result {
        let mut conn = self.db.get_conn()?;
        let configs = dao::config::query_all(&mut conn)?;
        let configs = configs
            .into_iter()
            .rev()
            .map(Config::try_from)
            .collect::<Result<Vec<Config>>>()?;
        log::info!("configs: {:?}", &configs);
        let mut value = self.configs.lock().await;
        *value = configs;
        Ok(())
    }
    pub async fn refresh_resources(&mut self) -> Result {
        let mut conn = self.db.get_conn()?;
        let resources = dao::resource::query_all(&mut conn)?.try_into()?;
        log::info!("resources: {:?}", &resources);
        let mut value = self.resources.lock().await;
        *value = resources;
        Ok(())
    }
}
