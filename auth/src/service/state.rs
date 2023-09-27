use crate::{dao, model};
use empty_utils::{
    diesel::db,
    errors::{ErrorConvert, Result},
};
use futures_util::future::BoxFuture;
use http::StatusCode;
use hyper::{Request, Response};
use proto::pb;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::body::BoxBody;
use tonic::codegen::empty_body;
use tower_http::auth::AsyncAuthorizeRequest;

pub struct Service {
    pub(super) db: db::DbPool,
    resources: Arc<Mutex<model::resource::Resource>>,
    configs: Arc<Mutex<Vec<model::config::Config>>>,
}

impl Service {
    pub async fn new() -> Result<Self> {
        let db = db::DbPool::new("auth_v2");
        Self::new_by_db(db).await
    }
    pub async fn new_by_db(db: db::DbPool) -> Result<Self> {
        let value = Self {
            db,
            resources: Arc::new(Mutex::new(Default::default())),
            configs: Arc::new(Mutex::new(vec![])),
        };
        value.refresh_configs().await?;
        value.refresh_resources().await?;
        Ok(value)
    }
    pub async fn refresh_configs(&self) -> Result {
        let mut conn = self.db.get_conn()?;
        let configs = dao::config::query_all(&mut conn)?;
        let configs = configs
            .into_iter()
            .rev()
            .map(model::config::Config::try_from)
            .collect::<Result<Vec<model::config::Config>>>()?;
        log::info!("configs: {:?}", &configs);
        let mut value = self.configs.lock().await;
        *value = configs;
        Ok(())
    }
    pub async fn refresh_resources(&self) -> Result {
        let mut conn = self.db.get_conn()?;
        let resources = dao::resource::query_all(&mut conn)?.try_into()?;
        log::info!("resources: {:?}", &resources);
        let mut value = self.resources.lock().await;
        *value = resources;
        Ok(())
    }
    pub async fn get_resource_by_access_token(
        &self,
        access_token: &String,
    ) -> Result<pb::auth::auth::Resource> {
        let value = self.resources.lock().await;
        let item = value.get(access_token).ok_or_loss()?.to_owned();
        Ok(item)
    }
    pub async fn get_resource_by_refresh_token(
        &self,
        refresh_token: &String,
    ) -> Result<pb::auth::auth::Resource> {
        let refresh_token = refresh_token.to_owned();
        let value = self.resources.lock().await;
        let item = value
            .values()
            .find(|i| i.refresh_token == refresh_token)
            .ok_or_loss()?
            .to_owned();
        Ok(item)
    }
    pub async fn get_scope_by_uri(&self, uri: String) -> model::config::Scope {
        let value = self.configs.lock().await;
        let item = value
            .iter()
            .find_map(|i| i.get_scope(uri.clone()))
            .unwrap_or_default()
            .to_owned();
        item
    }
}

// impl<B> AsyncAuthorizeRequest<B> for Service
// where
//     B: Send + Sync + 'static,
// {
//     type RequestBody = B;
//     type ResponseBody = BoxBody;
//     type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

//     fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
//         let that = self.clone();
//         Box::pin(async move {
//             let configs = that.configs.lock().await;
//             let _method = request.method();
//             let uri = request.uri().to_string();
//             let exp = regex::Regex::new("^https?://[^/]+").unwrap();
//             let uri = exp.replace(&uri, "");
//             let scope = configs
//                 .iter()
//                 .find_map(|config| config.get_scope(uri.to_string()));
//             log::info!("request uri: {},scope: {:?}", uri, scope);

//             let authorized = request
//                 .headers()
//                 .get(http::header::AUTHORIZATION)
//                 .and_then(|it| it.to_str().ok());
//             if let Some(auth) = authorized {
//                 let auth = auth.to_owned();
//                 let scope = scope.unwrap_or_default().unwrap_or("".parse().unwrap());
//                 let res = that.check_resource(auth, scope).await;
//                 return match res {
//                     Ok(res) => {
//                         let user_id = UserId::new(res.into_inner());
//                         log::info!("login user: {:?}", *user_id);
//                         request.extensions_mut().insert(user_id);
//                         Ok(request)
//                     }
//                     Err(err) => {
//                         log::warn!("check resource: {}", err);
//                         let unauthorized_response = Response::builder()
//                             .status(StatusCode::UNAUTHORIZED)
//                             .body(empty_body())
//                             .unwrap();
//                         Err(unauthorized_response)
//                     }
//                 };
//             }
//             if scope.is_none() || scope.clone().unwrap().is_none() {
//                 return Ok(request);
//             }
//             let unauthorized_response = Response::builder()
//                 .status(StatusCode::UNAUTHORIZED)
//                 .body(empty_body())
//                 .unwrap();
//             Err(unauthorized_response)
//         })
//     }
// }
