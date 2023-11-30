use crate::{dao, model, util::get_jwt_key};
use chrono::Utc;
use empty_utils::{
    diesel::db,
    errors::{ErrorConvert, Result},
};
use futures_util::future::BoxFuture;
use http::StatusCode;
use hyper::{Request, Response};
use jwt::VerifyWithKey;
use proto::pb;
use std::{env, sync::Arc, time::Instant};
use tokio::sync::Mutex;
use tonic::body::BoxBody;
use tonic::codegen::empty_body;
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Clone)]
pub struct Service {
    pub(super) db: db::DbPool,
    configs: Arc<Mutex<Vec<model::config::Config>>>,
}

impl Service {
    pub async fn new_by_db(db: db::DbPool) -> Result<Self> {
        let value = Self {
            db,
            configs: Arc::new(Mutex::new(vec![])),
        };
        value.refresh_configs().await?;
        let now = Instant::now();
        let _client = value.get_client().await?;
        log::info!("get client time: {:?}", now.elapsed());
        Ok(value)
    }
    pub async fn refresh_configs(&self) -> Result {
        let mut conn = self.db.get_conn()?;
        let configs = dao::config::query_list(&mut conn)?;
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

    pub async fn get_client(&self) -> Result<pb::user::auth::Client> {
        let mut conn = self.db.get_conn()?;
        let client = dao::client::query_by_id(
            &mut conn,
            String::from("f2e69885-951a-4538-b0c8-67385f0c1420")
                .parse()
                .ok_or_invalid()?,
        )?;
        Ok(client)
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

        let configs = self.configs.clone();
        Box::pin(async move {
            let configs = configs.lock().await;
            let scope_uri = configs
                .iter()
                .find_map(|config| config.get_scope(uri.clone()))
                .unwrap_or_default();
            log::info!("request uri: {}, scope uri: {:?}", uri, scope_uri);
            let authorized = request
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|it| it.to_str().ok());

            let key = get_jwt_key().map_err(|e| get_unauthorized_response())?;

            if let Some(pb::user::auth::JwtPayload { exp, sco, sub, .. }) = authorized
                .and_then(|auth| auth.strip_prefix("Bearer "))
                .and_then(|auth| auth.verify_with_key(&key).ok())
            {
                if exp < Utc::now().timestamp() {
                    log::warn!("token timeout: {}, authorized: {:?}", exp, authorized);
                    Err(get_unauthorized_response())?;
                }

                let scope_token = sco.parse::<model::config::Scope>().unwrap_or_default();

                if scope_token < scope_uri {
                    log::warn!("PermissionDenied: {}<{}", scope_token, scope_uri);
                    Err(get_unauthorized_response())?
                }
                log::info!("login user: {}", &sub);
                request.extensions_mut().insert(sub);
                return Ok(request);
            }
            if !scope_uri.is_empty() {
                Err(get_unauthorized_response())?
            }
            Ok(request)
        })
    }
}

fn get_unauthorized_response() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(empty_body())
        .unwrap()
}
