use oxide_auth::{
    endpoint::{OAuthError, Scope, WebRequest},
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};
use oxide_auth_async::endpoint;

use super::primitives::Guard;

#[derive(Clone)]
pub struct Vacant;

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
        log::info!("error");
        dbg!(err);
        err.into()
    }

    fn web_error(&mut self, err: Request::Error) -> Self::Error {
        log::info!("web_error");
        err
    }
}
