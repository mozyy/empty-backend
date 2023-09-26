use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

use empty_utils::{
    diesel::db,
    errors::{ErrorConvert, Result},
    tonic::Resp,
};
use tonic::{Request, Response};

use crate::dao;
use proto::pb;

pub struct Service {
    pub(super) db: db::DbPool,
    resources: Arc<Mutex<Vec<pb::auth::auth::Resource>>>,
    configs: Arc<Mutex<Vec<pb::auth::auth::Config>>>,
}

impl Service {
    pub async fn new() -> Result<Self> {
        let db = db::DbPool::new("auth_v2");
        Self::new_by_db(db).await
    }
    pub async fn new_by_db(db: db::DbPool) -> Result<Self> {
        let mut conn = db.get_conn()?;
        let configs = dao::config::query_all(&mut conn).await?;
        log::info!("configs: {:?}", &configs);
        todo!()
        // let configs = configs
        //     .into_iter()
        //     .rev()
        //     .filter_map(|config| {
        //         config
        //             .pattern
        //             .and_then(|pattern| pattern.pattern)
        //             .map(|pattern| match pattern {
        //                 pb::auth::auth::pattern::Pattern::Equal(value) => Pattern::Equal(value),
        //                 pb::auth::auth::pattern::Pattern::Prefix(value) => Pattern::Prefix(value),
        //                 pb::auth::auth::pattern::Pattern::Regex(value) => {
        //                     Pattern::Regex(value.parse().unwrap())
        //                 }
        //             })
        //             .map(|pattern|  pb::auth:: {
        //                 pattern,
        //                 scope: config.scope.map(|string| string.parse().unwrap()),
        //             })
        //     })
        //     .collect();
        // let mut value = self.configs.lock().await;
        // log::info!("configs: {:?}", &configs);
        // *value = configs;
        // Self { db }
    }
    pub async fn configs(&self) -> Request<Vec<pb::auth::auth::Config>> {
        todo!()
    }
}

// #[derive(Debug)]
// pub struct Config {
//     pattern: Pattern,
//     scope: HashSet<String>,
// }

// impl Config {
//     fn get_scope(&self, url: String) -> Option<Option<Scope>> {
//         let matched = self.pattern.matched(url);
//         if matched {
//             Some(self.scope.to_owned())
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug)]
enum Pattern {
    Equal(String),
    Prefix(String),
    Regex(regex::Regex),
}

impl Pattern {
    fn matched(&self, url: String) -> bool {
        match self {
            Pattern::Equal(value) => *value == url,
            Pattern::Prefix(value) => url.starts_with(value),
            Pattern::Regex(value) => value.is_match(url.as_str()),
        }
    }
}
