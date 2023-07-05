use std::sync::Arc;

use empty_utils::diesel::db;
use oxide_auth::{
    frontends::simple::endpoint::Vacant,
    primitives::{
        prelude::{AuthMap, Client, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};

use tokio::sync::Mutex;

use crate::{
    model::oauth::{endpoint::Endpoint, UserId},
    pb,
};

#[derive(Clone)]
pub struct State {
    pub db: db::DbPool,
    client_map: Arc<Mutex<ClientMap>>,
    auth_map: Arc<Mutex<AuthMap<RandomGenerator>>>,
    token_map: Arc<Mutex<TokenMap<RandomGenerator>>>,
    // solicitor: Vacant,
}

impl State {
    pub fn new() -> Self {
        Self {
            db: db::DbPool::new("lottery"),
            client_map: Arc::new(Mutex::new(
                vec![
                    Client::confidential(
                        "zuoyi",
                        "http://localhost:8021/endpoint"
                            .parse::<url::Url>()
                            .unwrap()
                            .into(),
                        "default-scope".parse().unwrap(),
                        "SecretSecret".as_bytes(),
                    ),
                    Client::public(
                        "zuoyin",
                        "http://localhost:8021/endpoint"
                            .parse::<url::Url>()
                            .unwrap()
                            .into(),
                        "default-scope".parse().unwrap(),
                    ),
                ]
                .into_iter()
                .collect(),
            )),
            auth_map: Arc::new(Mutex::new(AuthMap::new(RandomGenerator::new(16)))),
            token_map: Arc::new(Mutex::new(TokenMap::new(RandomGenerator::new(16)))),
        }
    }

    pub async fn endpoint(&self) -> Endpoint<'_, Vacant> {
        Endpoint {
            registrar: self.client_map.lock().await.into(),
            authorizer: self.auth_map.lock().await.into(),
            issuer: self.token_map.lock().await.into(),
            solicitor: Vacant,
            scopes: vec![],
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

use futures_util::future::BoxFuture;
use http::{StatusCode};
use hyper::{Request, Response};
use tonic::{body::BoxBody, codegen::empty_body};
use tower::{ServiceExt};
use tower_http::auth::{AsyncAuthorizeRequest};

impl<B> AsyncAuthorizeRequest<B> for State
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let mut that = self.clone();
        Box::pin(async move {
            let authorized = request
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|it| it.to_str().ok())
                .and_then(|it| it.strip_prefix("Bearer "));
            if let Some(auth) = authorized {
                let auth = auth.to_owned();
                let res = that
                    .check_resource(pb::oauth::ResourceRequest {
                        auth,
                        uri: "".into(),
                    })
                    .await;
                match res {
                    Ok(res) => {
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
                }
            } else {
                Ok(request)
            }
        })
    }
}
