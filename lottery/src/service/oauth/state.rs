use std::sync::Arc;

use oxide_auth::{
    frontends::simple::endpoint::Vacant,
    primitives::{
        prelude::{AuthMap, Client, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};

use tokio::sync::Mutex;

use crate::model::oauth::endpoint::Endpoint;

#[derive(Clone)]
pub struct State {
    client_map: Arc<Mutex<ClientMap>>,
    auth_map: Arc<Mutex<AuthMap<RandomGenerator>>>,
    token_map: Arc<Mutex<TokenMap<RandomGenerator>>>,
    // solicitor: Vacant,
}

impl State {
    pub fn new() -> Self {
        Self {
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
