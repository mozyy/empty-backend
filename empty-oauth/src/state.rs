use oxide_auth::{
    endpoint::Scope,
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::{
        prelude::{RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};

use crate::model::{Auth, Solicitor};

pub struct OAuthState {
    pub endpoint: Generic<
        // Client,
        ClientMap,
        Auth,
        TokenMap<RandomGenerator>,
        Solicitor,
        Vec<Scope>,
        Vacant,
    >,
}

impl OAuthState {
    pub fn new() -> Self {
        OAuthState {
            endpoint: Generic {
                authorizer: Auth::new(),
                // registrar: Client::new(),
                registrar: ClientMap::new(),
                issuer: TokenMap::new(RandomGenerator::new(16)),
                scopes: vec!["default-scope".parse().unwrap()],
                solicitor: Solicitor::new(),
                response: Vacant,
            },
        }
    }
}
impl Default for OAuthState {
    fn default() -> Self {
        Self::new()
    }
}
