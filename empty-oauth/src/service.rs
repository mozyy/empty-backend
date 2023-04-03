use std::sync::{Arc, Mutex};

use empty_utils::diesel::db;
use oxide_auth::{
    endpoint::{Authorizer, Endpoint, Issuer, Registrar},
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope, TokenMap},
};
use oxide_auth_axum::OAuthResponse;

pub type ServiceEndpoint = Generic<
    ClientMap,
    AuthMap<RandomGenerator>,
    TokenMap<RandomGenerator>,
    Vacant,
    Vec<Scope>,
    fn() -> OAuthResponse,
>;

#[derive(Clone)]
pub struct Service {
    db: db::DbPool,
    registrar: Arc<Mutex<ClientMap>>,
    authorizer: Arc<Mutex<AuthMap<RandomGenerator>>>,
    issuer: Arc<Mutex<TokenMap<RandomGenerator>>>,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: Default::default(),
            registrar: Arc::new(Mutex::new(
                vec![Client::confidential(
                    "LocalClient",
                    "http://localhost:8021/endpoint"
                        .parse::<url::Url>()
                        .unwrap()
                        .into(),
                    "default-scope".parse().unwrap(),
                    "SecretSecret".as_bytes(),
                )]
                .into_iter()
                .collect(),
            )),
            // Authorization tokens are 16 byte random keys to a memory hash map.
            authorizer: Arc::new(Mutex::new(AuthMap::new(RandomGenerator::new(16)))),
            // Bearer tokens are also random generated but 256-bit tokens, since they live longer
            // and this example is somewhat paranoid.
            //
            // We could also use a `TokenSigner::ephemeral` here to create signed tokens which can
            // be read and parsed by anyone, but not maliciously created. However, they can not be
            // revoked and thus don't offer even longer lived refresh tokens.
            issuer: Arc::new(Mutex::new(TokenMap::new(RandomGenerator::new(16)))),
            // solicitor: Vacant,

            // // A single scope that will guard resources for this endpoint
            // scopes: vec!["default-scope".parse().unwrap()],

            // response: OAuthResponse::default,
        }
    }
}

impl Service {
    pub async fn get_clients(&self) -> Result<(), ()> {
        todo!()
    }
    pub fn endpoint(&self) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
        Generic {
            registrar: self.registrar.lock().unwrap(),
            authorizer: self.authorizer.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            // Solicitor configured later.
            solicitor: Vacant,
            // Scope configured later.
            scopes: Vacant,
            // `rocket::Response` is `Default`, so we don't need more configuration.
            response: Vacant,
        }
    }
}
