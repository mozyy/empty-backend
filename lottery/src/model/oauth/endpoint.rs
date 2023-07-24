use super::{diesel::client_query_all, primitives::Guard};

use oxide_auth::{
    endpoint::{OAuthError, Scope, WebRequest},
    frontends::simple::endpoint::Vacant,
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::{Client, ClientMap},
    },
};
use oxide_auth_async::endpoint;
use std::sync::Arc;

use url::Url;

use std::collections::HashMap;
use tokio::sync::Mutex;

use empty_utils::{diesel::db, errors::Result};
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use oxide_auth_async::endpoint::{access_token::AccessTokenFlow, authorization::AuthorizationFlow};
use uuid::Uuid;

use crate::{
    model::oauth::grpc::{
        request::{Auth, OAuthRequest},
        response::{OAuthResponse, ResponseStatus},
    },
    pb::oauth as pb,
};

#[derive(Clone)]
pub struct EndpointState {
    client_map: Arc<Mutex<ClientMap>>,
    auth_map: Arc<Mutex<AuthMap<RandomGenerator>>>,
    token_map: Arc<Mutex<TokenMap<RandomGenerator>>>,
}

impl EndpointState {
    pub async fn new(db: db::DbPool) -> Result<Self> {
        let mut conn = db.get_conn()?;
        let clients = client_query_all(&mut conn).await?;
        log::info!("clients: {:?}", clients);
        let clients = clients
            .into_iter()
            .map(|client| {
                let pb::Client {
                    id,
                    redirect_uri,
                    default_scope,
                    passdata: passphrase,
                    ..
                } = client;
                let redirect_uri = redirect_uri.parse::<Url>().unwrap().into();
                let default_scope = default_scope.parse().unwrap();
                match passphrase {
                    Some(passphrase) => Client::confidential(
                        &id,
                        redirect_uri,
                        default_scope,
                        passphrase.as_bytes(),
                    ),
                    None => Client::public(&id, redirect_uri, default_scope),
                }
            })
            .collect();
        Ok(Self {
            client_map: Arc::new(Mutex::new(clients)),
            auth_map: Arc::new(Mutex::new(AuthMap::new(RandomGenerator::new(16)))),
            token_map: Arc::new(Mutex::new(TokenMap::new(RandomGenerator::new(16)))),
        })
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
    pub async fn authorize_by_id(
        &self,
        user_id: Uuid,
        request: OAuthRequest,
        client: pb::Client,
    ) -> Result<OAuthResponse> {
        let endpoint = self.endpoint().await;
        let endpoint =
            endpoint.with_solicitor(FnSolicitor(|_: &mut OAuthRequest, _: Solicitation| {
                OwnerConsent::Authorized(user_id.to_string())
            }));

        let resp = AuthorizationFlow::prepare(endpoint)?
            .execute(request)
            .await?;
        let mut code = if let ResponseStatus::Redirect(url) = resp.status {
            url.query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new)
        } else {
            HashMap::new()
        };
        code.insert("grant_type".into(), "authorization_code".into());
        // TODO: from query
        code.insert("client_id".into(), client.id);
        code.insert("redirect_uri".into(), client.redirect_uri);
        let res =
            AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)?
                .execute(OAuthRequest {
                    auth: Auth(None),
                    query: code.clone(),
                    body: code,
                })
                .await?;
        Ok(res)
    }
}

pub struct Endpoint<'a, Solicitor> {
    /// The registrar implementation, or `Vacant` if it is not necesary.
    pub(crate) registrar: Guard<'a, ClientMap>,

    /// The authorizer implementation, or `Vacant` if it is not necesary.
    pub(crate) authorizer: Guard<'a, AuthMap<RandomGenerator>>,

    /// The issuer implementation, or `Vacant` if it is not necesary.
    pub(crate) issuer: Guard<'a, TokenMap<RandomGenerator>>,

    // extension: Extension,
    /// A solicitor implementation fit for the request types, or `Vacant` if it is not necesary.
    pub(crate) solicitor: Solicitor,

    /// Determine scopes for the request types, or `Vacant` if this does not protect resources.
    pub(crate) scopes: Vec<Scope>,
    // / Creates responses, or `Vacant` if `Default::default` is applicable.
    // response: Vacant,
}

impl<'a, Solicitor> Endpoint<'a, Solicitor> {
    pub fn with_scopes(self, scopes: Vec<Scope>) -> Endpoint<'a, Solicitor> {
        Endpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            solicitor: self.solicitor,
            scopes,
        }
    }
    pub fn with_solicitor<Request, S>(self, solicitor: S) -> Endpoint<'a, S>
    where
        Request: WebRequest,
        Request::Response: Default,
        Request::Error: From<OAuthError>,
        S: endpoint::OwnerSolicitor<Request>,
    {
        Endpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            solicitor,
            scopes: self.scopes,
        }
    }
}

impl<'a, Request, Solicitor> endpoint::Endpoint<Request> for Endpoint<'a, Solicitor>
where
    Request: WebRequest,
    Request::Response: Default,
    Request::Error: From<OAuthError>,
    Solicitor: endpoint::OwnerSolicitor<Request> + Send,
{
    type Error = Request::Error;

    fn registrar(&self) -> Option<&(dyn oxide_auth_async::primitives::Registrar + Sync)> {
        Some(&self.registrar)
    }

    fn authorizer_mut(
        &mut self,
    ) -> Option<&mut (dyn oxide_auth_async::primitives::Authorizer + Send)> {
        Some(&mut self.authorizer)
    }

    fn issuer_mut(&mut self) -> Option<&mut (dyn oxide_auth_async::primitives::Issuer + Send)> {
        Some(&mut self.issuer)
    }

    fn owner_solicitor(&mut self) -> Option<&mut (dyn endpoint::OwnerSolicitor<Request> + Send)> {
        Some(&mut self.solicitor)
    }

    fn scopes(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Scopes<Request>> {
        Some(&mut self.scopes)
    }

    fn response(
        &mut self,
        _request: &mut Request,
        kind: oxide_auth::endpoint::Template,
    ) -> Result<Request::Response, Self::Error> {
        log::info!("response");
        dbg!(kind);
        Ok(Default::default())
    }

    fn error(&mut self, err: oxide_auth::endpoint::OAuthError) -> Self::Error {
        log::error!("error");
        err.into()
    }

    fn web_error(&mut self, err: Request::Error) -> Self::Error {
        log::error!("web_error");
        err
    }
}
