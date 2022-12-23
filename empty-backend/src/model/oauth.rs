use axum::{
    http::{
        header::{self, HeaderMap, HeaderValue},
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use oxide_auth::{
    endpoint::{
        Authorizer, Issuer, OwnerConsent, OwnerSolicitor, PreGrant, Registrar, Scope, Solicitation,
        WebRequest,
    },
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::{
        grant::Grant,
        issuer::RefreshedToken,
        prelude::{AuthMap, IssuedToken, RandomGenerator, TokenMap},
        registrar::{BoundClient, ClientMap, ClientUrl, RegistrarError},
    },
};
use oxide_auth_axum::OAuthResponse;

pub struct OAuthState {
    pub endpoint: Generic<Client, Auth, TokenMap<RandomGenerator>, Solicitor, Vec<Scope>, Vacant>,
}

impl OAuthState {
    pub fn new() -> Self {
        OAuthState {
            endpoint: Generic {
                authorizer: Auth::new(),
                registrar: Client::new(),
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

pub struct Auth {}

impl Auth {
    fn new() -> Self {
        Auth {}
    }
}

impl Authorizer for Auth {
    fn authorize(&mut self, _: Grant) -> Result<String, ()> {
        todo!()
    }

    fn extract(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        todo!()
    }
}

pub struct Client {}
impl Client {
    fn new() -> Self {
        Client {}
    }
}
impl Registrar for Client {
    fn bound_redirect<'a>(&self, bound: ClientUrl<'a>) -> Result<BoundClient<'a>, RegistrarError> {
        todo!()
    }

    fn negotiate(
        &self,
        client: BoundClient,
        scope: Option<Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        todo!()
    }

    fn check(&self, client_id: &str, passphrase: Option<&[u8]>) -> Result<(), RegistrarError> {
        todo!()
    }
}

pub struct Issue {}
impl Issue {
    fn new() -> Self {
        Issue {}
    }
}
impl Issuer for Issue {
    fn issue(&mut self, grant: Grant) -> Result<IssuedToken, ()> {
        todo!()
    }

    fn refresh(&mut self, _refresh: &str, _grant: Grant) -> Result<RefreshedToken, ()> {
        todo!()
    }

    fn recover_token<'a>(&'a self, _: &'a str) -> Result<Option<Grant>, ()> {
        todo!()
    }

    fn recover_refresh<'a>(&'a self, _: &'a str) -> Result<Option<Grant>, ()> {
        todo!()
    }
}

pub struct Solicitor {}

impl Solicitor {
    fn new() -> Self {
        Solicitor {}
    }
}

impl<R: WebRequest> OwnerSolicitor<R> for Solicitor {
    fn check_consent(
        &mut self,
        _: &mut R,
        _: Solicitation,
    ) -> OwnerConsent<<R as WebRequest>::Response> {
        todo!()
    }
}
