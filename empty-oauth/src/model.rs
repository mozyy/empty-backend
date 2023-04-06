use crate::schema::{clients, registered_urls};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::diesel::timestamp;
use empty_utils::errors::ServiceError;
use oxide_auth::{
    endpoint::{Authorizer, Issuer, OwnerConsent, OwnerSolicitor, Solicitation, WebRequest},
    frontends::dev::Url,
    primitives::{
        grant::Grant,
        issuer::RefreshedToken,
        prelude::IssuedToken,
        registrar::{self, ExactUrl, IgnoreLocalPortUrl},
    },
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

pub struct NewClient {
    // pub name: String,
    // pub desc: String,
    pub default_scope: Vec<Option<String>>,
    pub client_type: Option<String>,
}
#[derive(Insertable)]
#[diesel(table_name =registered_urls)]
pub struct NewRegisteredUrl {
    pub url: String,
    #[diesel(column_name = "type_")]
    pub r#type: i16,
}

pub struct NewClientUrl {
    pub new_client: NewClient,
    pub new_redirect_uris: NewRegisteredUrl,
    pub new_additional_redirect_uris: Vec<NewRegisteredUrl>,
}
pub struct ClientUrl {
    pub client: Client,
    pub redirect_uri: RegisteredUrl,
    pub additional_redirect_uris: Vec<RegisteredUrl>,
}
impl From<ClientUrl> for registrar::Client {
    fn from(value: ClientUrl) -> Self {
        let ClientUrl {
            client:
                Client {
                    id,
                    client_type,
                    default_scope,
                    ..
                },
            redirect_uri,
            additional_redirect_uris,
        } = value;
        let id = id.to_string();
        let id = id.as_str();
        let default_scope = default_scope
            .into_iter()
            .filter_map(|s| s)
            .collect::<String>()
            .parse()
            .unwrap();
        let client = if let Some(passphrase) = client_type {
            registrar::Client::confidential(
                id,
                redirect_uri.into(),
                default_scope,
                passphrase.as_bytes(),
            )
        } else {
            registrar::Client::public(id, redirect_uri.into(), default_scope)
        };

        let additional_redirect_uris = Vec::from_iter(additional_redirect_uris);
        client.with_additional_redirect_uris(additional_redirect_uris)
    }
}
impl FromIterator<ClientUrl> for Vec<registrar::Client> {
    fn from_iter<T: IntoIterator<Item = ClientUrl>>(iter: T) -> Self {
        iter.into_iter()
            .map(|c| registrar::Client::from(c))
            .collect()
    }
}
impl From<RegisteredUrl> for registrar::RegisteredUrl {
    fn from(value: RegisteredUrl) -> Self {
        let RegisteredUrl { url, r#type, .. } = value;
        match r#type {
            1 => url.parse::<Url>().unwrap().into(),
            2 => url.parse::<ExactUrl>().unwrap().into(),
            3 => IgnoreLocalPortUrl::new(url).unwrap().into(),
            _ => panic!(),
        }
    }
}
impl FromIterator<RegisteredUrl> for Vec<registrar::RegisteredUrl> {
    fn from_iter<T: IntoIterator<Item = RegisteredUrl>>(iter: T) -> Self {
        iter.into_iter()
            .map(|r| registrar::RegisteredUrl::from(r))
            .collect()
    }
}

impl ClientUrl {
    pub fn insert(conn: &mut PgConnection, req: NewClientUrl) -> ServiceResult<Uuid> {
        let client_id = conn.transaction::<_, diesel::result::Error, _>(move |conn| {
            let redirect_uri_id = diesel::insert_into(registered_urls::dsl::registered_urls)
                .values(req.new_redirect_uris)
                .returning(registered_urls::id)
                .get_result::<i32>(conn)?;
            let client_id = diesel::insert_into(clients::dsl::clients)
                .values((
                    clients::redirect_uri_id.eq(redirect_uri_id),
                    clients::default_scope.eq(req.new_client.default_scope),
                    clients::client_type.eq(req.new_client.client_type),
                ))
                .returning(clients::id)
                .get_result::<Uuid>(conn)?;

            diesel::insert_into(registered_urls::dsl::registered_urls)
                .values(
                    req.new_additional_redirect_uris
                        .iter()
                        .map(|uri| {
                            (
                                registered_urls::client_id.eq(Some(client_id)),
                                registered_urls::url.eq(uri.url.clone()),
                                registered_urls::type_.eq(uri.r#type),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .execute(conn)?;
            Ok(client_id)
        })?;
        Ok(client_id)
    }
    pub fn select_all(conn: &mut PgConnection) -> ServiceResult<Vec<ClientUrl>> {
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
