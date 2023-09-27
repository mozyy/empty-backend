use crate::{dao, model};
use chrono::Utc;
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

#[derive(Clone)]
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

impl<B> AsyncAuthorizeRequest<B> for Service
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let uri = request.uri().to_string();
        let uri_exp: regex::Regex = regex::Regex::new("^https?://[^/]+").unwrap();
        let uri = uri_exp.replace(&uri, "").to_string();

        let Self {
            configs, resources, ..
        } = self;
        let configs = configs.clone();
        let resources = resources.clone();
        Box::pin(async move {
            let configs = configs.lock().await;
            let scope_uri = configs
                .iter()
                .find_map(|config| config.get_scope(uri.clone()))
                .unwrap_or_default();
            log::info!("request uri: {},scope: {:?}", uri, scope_uri);
            let authorized = request
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|it| it.to_str().ok());
            if let Some(auth) = authorized {
                let resources = resources.lock().await;
                let resource = resources.get(auth).map(ToOwned::to_owned);
                let resource = match resource {
                    Some(resource) => {
                        if let Some(until) = resource.until.clone() {
                            if until.seconds > Utc::now().timestamp() {
                                log::warn!("token timeout: {}", until);
                                let unauthorized_response = Response::builder()
                                    .status(StatusCode::UNAUTHORIZED)
                                    .body(empty_body())
                                    .unwrap();
                                return Err(unauthorized_response);
                            }
                        };
                        resource
                    }
                    None => {
                        log::warn!("resource is none: {}", auth);
                        let unauthorized_response = Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(empty_body())
                            .unwrap();
                        return Err(unauthorized_response);
                    }
                };
                let scope_token = resource
                    .scope
                    .parse::<model::config::Scope>()
                    .unwrap_or_default();
                return if scope_token < scope_uri {
                    log::warn!("PermissionDenied: {}<{}", scope_token, scope_uri);
                    let unauthorized_response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(empty_body())
                        .unwrap();
                    Err(unauthorized_response)
                } else {
                    log::info!("login user: {:?}", resource);
                    request.extensions_mut().insert(resource);
                    Ok(request)
                };
            }
            if scope_uri.is_empty() {
                return Ok(request);
            }
            let unauthorized_response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(empty_body())
                .unwrap();
            Err(unauthorized_response)
        })
    }
}
