use oxide_auth::{
    endpoint::Scope,
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};
use oxide_auth_async::endpoint;
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

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

impl<'a, Solicitor> Endpoint<'a, Solicitor>
where
    Solicitor: endpoint::OwnerSolicitor<OAuthRequest> + Send + Sync,
{
    pub fn with_scopes(self, scopes: Vec<Scope>) -> Endpoint<'a, Solicitor> {
        Endpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            solicitor: self.solicitor,
            scopes,
        }
    }
    pub fn with_solicitor<S>(self, solicitor: S) -> Endpoint<'a, S>
    where
        S: endpoint::OwnerSolicitor<OAuthRequest>,
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

impl<'a, Solicitor> endpoint::Endpoint<OAuthRequest> for Endpoint<'a, Solicitor>
where
    Solicitor: endpoint::OwnerSolicitor<OAuthRequest> + Send,
{
    type Error = WebError;

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

    fn owner_solicitor(
        &mut self,
    ) -> Option<&mut (dyn endpoint::OwnerSolicitor<OAuthRequest> + Send)> {
        Some(&mut self.solicitor)
    }

    fn scopes(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Scopes<OAuthRequest>> {
        Some(&mut self.scopes)
    }

    fn response(
        &mut self,
        request: &mut OAuthRequest,
        kind: oxide_auth::endpoint::Template,
    ) -> Result<<OAuthRequest as oxide_auth::endpoint::WebRequest>::Response, Self::Error> {
        log::info!("response");
        dbg!(request);
        dbg!(kind);
        Ok(OAuthResponse::default())
    }

    fn error(&mut self, err: oxide_auth::endpoint::OAuthError) -> Self::Error {
        log::info!("error");
        dbg!(err);
        err.into()
    }

    fn web_error(
        &mut self,
        err: <OAuthRequest as oxide_auth::endpoint::WebRequest>::Error,
    ) -> Self::Error {
        log::info!("web_error");
        err
    }
}
