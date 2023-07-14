use std::sync::Arc;

use empty_utils::{diesel::db, errors::Result, tonic::Resp};
use oxide_auth::{endpoint::Scope, frontends::simple::endpoint::Vacant};

use oxide_auth_async::endpoint::resource::ResourceFlow;
use tokio::sync::Mutex;

use crate::{
    configs::ADDR_CLIENT,
    model::oauth::{
        diesel::config_query_all,
        endpoint::{Endpoint, EndpointState},
        grpc::request::OAuthRequest,
    },
    pb::oauth as pb,
};
use futures_util::future::BoxFuture;
use http::StatusCode;
use hyper::{Request, Response};

use tonic::{body::BoxBody, codegen::empty_body};
use tower_http::auth::AsyncAuthorizeRequest;

use crate::model::oauth::UserId;

#[derive(Clone)]
pub struct State {
    pub db: db::DbPool,
    pub endpoint_state: EndpointState,
    pub configs: Arc<Mutex<Vec<Config>>>,
}

impl State {
    pub async fn new() -> Result<Self> {
        Self::new_by_db(db::DbPool::new("lottery")).await
    }

    pub async fn new_by_db(db: db::DbPool) -> Result<Self> {
        let mut value = Self {
            db: db.clone(),
            endpoint_state: EndpointState::new(db).await?,
            configs: Arc::new(Mutex::new(vec![])),
        };
        value.get_configs().await?;
        Ok(value)
    }
    pub async fn get_configs(&mut self) -> Result {
        let mut conn = self.db.get_conn()?;
        let configs = config_query_all(&mut conn).await?;
        log::info!("configs: {:?}", &configs);
        let configs = configs
            .into_iter()
            .rev()
            .filter_map(|config| {
                config
                    .pattern
                    .and_then(|pattern| pattern.pattern)
                    .map(|pattern| match pattern {
                        pb::pattern::Pattern::Equal(value) => Pattern::Equal(value),
                        pb::pattern::Pattern::Prefix(value) => Pattern::Prefix(value),
                        pb::pattern::Pattern::Regex(value) => {
                            Pattern::Regex(value.parse().unwrap())
                        }
                    })
                    .map(|pattern| Config {
                        pattern,
                        scope: config.scope.map(|string| string.parse().unwrap()),
                    })
            })
            .collect();
        let mut value = self.configs.lock().await;
        log::info!("configs: {:?}", &configs);
        *value = configs;
        Ok(())
    }
    pub async fn check_resource(&self, auth: String, scope: Scope) -> Resp<pb::ResourceResponse> {
        let endpoint = self
            .endpoint_state
            .endpoint()
            .await
            .with_scopes(vec![scope]);
        let res = ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(endpoint)
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
            .execute(OAuthRequest::default().with_auth(auth))
            .await;
        let res = match res {
            Ok(r) => r,
            Err(e) => match e {
                Ok(r) => {
                    log::warn!("{:?}", r);
                    return Err(tonic::Status::unauthenticated("r.into()"));
                }
                Err(e) => return Err(tonic::Status::unauthenticated(e.0.to_string())),
            },
        };
        Ok(tonic::Response::new(res.into()))
    }
}

impl<B> AsyncAuthorizeRequest<B> for State
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let that = self.clone();
        Box::pin(async move {
            let configs = that.configs.lock().await;
            let _method = request.method();
            let uri = request.uri().to_string();
            log::info!("request uri: {:?}, addr:{}", uri, ADDR_CLIENT);
            let exp = regex::Regex::new("^https?://[^/]+").unwrap();
            let uri = exp.replace(&uri, "");
            log::info!("request uri: {:?}", uri);
            let scope = configs
                .iter()
                .find_map(|config| config.get_scope(uri.to_string()));
            log::info!("request uri: {},scope: {:?}", uri, scope);

            let authorized = request
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|it| it.to_str().ok());
            if let Some(auth) = authorized {
                let auth = auth.to_owned();
                let scope = scope.unwrap_or_default().unwrap_or("".parse().unwrap());
                let res = that.check_resource(auth, scope).await;
                return match res {
                    Ok(res) => {
                        log::info!("oauth res: {:?}", res);
                        request
                            .extensions_mut()
                            .insert(UserId(res.into_inner().owner_id));
                        Ok(request)
                    }
                    Err(err) => {
                        log::warn!("check resource: {}", err);
                        let unauthorized_response = Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(empty_body())
                            .unwrap();
                        Err(unauthorized_response)
                    }
                };
            }
            if scope.is_none() || scope.clone().unwrap().is_none() {
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

#[derive(Debug)]
pub struct Config {
    pattern: Pattern,
    scope: Option<Scope>,
}

impl Config {
    fn get_scope(&self, url: String) -> Option<Option<Scope>> {
        let matched = self.pattern.matched(url);
        if matched {
            Some(self.scope.to_owned())
        } else {
            None
        }
    }
}

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
