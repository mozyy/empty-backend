use crate::{
    schema::{clients, redirect_uris},
    utils::timestamp,
};
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};
use oxide_auth::{
    endpoint::{Authorizer, Issuer, OwnerConsent, OwnerSolicitor, Scope, Solicitation, WebRequest},
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::{
        grant::Grant,
        issuer::RefreshedToken,
        prelude::{IssuedToken, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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

#[derive(Queryable, Identifiable, Serialize, ToSchema)]
pub struct Client {
    pub id: Uuid,
    pub redirect_uri_id: i32,
    pub name: String,
    pub desc: String,
    pub passphrase: Option<String>,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}
#[derive(Queryable, Identifiable, Serialize, ToSchema, Associations)]
#[diesel(belongs_to(Client))]
pub struct RedirectUri {
    pub id: i32,
    pub url: String,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
    pub client_id: Option<Uuid>,
}
pub struct ClientUrl {
    pub client: Client,
    pub redirect_uris: Vec<RedirectUri>,
}

impl Client {
    pub fn select_all(conn: &mut PgConnection) -> Result<Vec<ClientUrl>, Error> {
        let clients = clients::table.load::<Client>(conn)?;
        let redirect_uris: Vec<Vec<RedirectUri>> = RedirectUri::belonging_to(&clients)
            .load(conn)?
            .grouped_by(&clients);
        let resp = clients
            .into_iter()
            .zip(redirect_uris)
            .map(|(client, redirect_uris)| ClientUrl {
                client,
                redirect_uris,
            })
            .collect::<Vec<_>>();
        Ok(resp)
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
