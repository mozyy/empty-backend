use crate::schema::{clients, registered_urls};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::diesel::timestamp;
use empty_utils::errors::ServiceError;
use oxide_auth::{
    endpoint::{Authorizer, Issuer, OwnerConsent, OwnerSolicitor, Solicitation, WebRequest},
    primitives::{grant::Grant, issuer::RefreshedToken, prelude::IssuedToken},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

pub struct Auth {}

impl Auth {
    pub fn new() -> Self {
        Auth {}
    }
}

impl Authorizer for Auth {
    fn authorize(&mut self, _: Grant) -> Result<String, ()> {
        todo!()
    }

    fn extract(&mut self, _token: &str) -> Result<Option<Grant>, ()> {
        todo!()
    }
}

#[derive(Queryable, Identifiable, Serialize, ToSchema, Associations)]
#[diesel(belongs_to(RegisteredUrl, foreign_key = redirect_uri_id))]
pub struct Client {
    pub id: Uuid,
    pub redirect_uri_id: i32,
    // pub name: String,
    // pub desc: String,
    pub default_scope: Vec<Option<String>>,
    pub client_type: Option<String>,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}
#[derive(Queryable, Identifiable, Serialize, ToSchema, Associations, Clone)]
#[diesel(belongs_to(Client))]
pub struct RegisteredUrl {
    pub id: i32,
    pub client_id: Option<Uuid>,
    pub url: String,
    pub r#type: i16,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}
pub struct ClientUrl {
    pub client: Client,
    pub redirect_uri: RegisteredUrl,
    pub additional_redirect_uris: Vec<RegisteredUrl>,
}

pub struct NewClient {
    pub name: String,
    pub desc: String,
    pub passphrase: Option<String>,
}
pub struct NewRedirectUri {
    pub url: String,
}

pub struct NewClientUrl {
    pub new_client: NewClient,
    pub new_redirect_uris: Vec<NewRedirectUri>,
}

impl Client {
    pub fn insert(conn: &mut PgConnection, _req: NewClientUrl) -> Result<Uuid, ServiceError> {
        let _clients = clients::table.load::<Client>(conn)?;
        todo!();
    }
    pub fn select_all(conn: &mut PgConnection) -> Result<Vec<ClientUrl>, ServiceError> {
        let clients = clients::table.load::<Client>(conn)?;
        let redirect_uris = registered_urls::table.load::<RegisteredUrl>(conn)?;

        let resp = clients
            .into_iter()
            .map(|client| {
                let id = client.id;
                let redirect_uri_id = client.redirect_uri_id;
                let mut redirect_uri: Option<RegisteredUrl> = None;
                let mut additional_redirect_uris: Vec<RegisteredUrl> = vec![];
                for uri in redirect_uris.clone().into_iter() {
                    if redirect_uri.is_none() && uri.id == redirect_uri_id {
                        redirect_uri = Some(uri.clone())
                    }
                    match uri.client_id {
                        Some(client_id) if id == client_id => additional_redirect_uris.push(uri),
                        _ => {}
                    }
                }

                ClientUrl {
                    client,
                    redirect_uri: redirect_uri.expect(""),
                    additional_redirect_uris,
                }
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
    fn issue(&mut self, _grant: Grant) -> Result<IssuedToken, ()> {
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
    pub fn new() -> Self {
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
