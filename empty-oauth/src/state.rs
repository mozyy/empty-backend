use empty_utils::diesel::db::{self};
use oxide_auth::{
    endpoint::Scope,
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::{
        prelude::{RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};
use tokio::sync::Mutex;

use crate::model::{self, Auth, Solicitor};

type Endpoing = Generic<
    // Client,
    ClientMap,
    Auth,
    TokenMap<RandomGenerator>,
    Solicitor,
    Vec<Scope>,
    Vacant,
>;
pub struct OAuthState {
    pub endpoint: Mutex<Endpoing>,
    pub db: db::DbPool,
}

impl OAuthState {
    pub fn new() -> Self {
        let db = db::DbPool::new("empty_oauth");
        let mut conn = db.get_conn().unwrap();
        let clients = model::ClientUrl::select_all(&mut conn).unwrap();
        let clients = Vec::from_iter(clients);
        let client_map = ClientMap::from_iter(clients.into_iter());
        Self {
            endpoint: Mutex::new(Generic {
                authorizer: Auth::new(),
                // registrar: Client::new(),
                registrar: client_map,
                issuer: TokenMap::new(RandomGenerator::new(16)),
                scopes: vec!["default-scope".parse().unwrap()],
                solicitor: Solicitor::new(),
                response: Vacant,
            }),
            db,
        }
    }
    pub fn refresh_endpoint(&mut self) {}
    pub fn refresh_client(&mut self) {}
}
impl Default for OAuthState {
    fn default() -> Self {
        Self::new()
    }
}
