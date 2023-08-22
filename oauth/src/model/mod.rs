use async_trait::async_trait;
use oxide_auth::{
    code_grant::accesstoken::Request,
    endpoint::{OwnerConsent, Solicitation},
    primitives::grant::Extensions,
};
use oxide_auth_async::endpoint::OwnerSolicitor;

use grpc::{request::OAuthRequest, response::OAuthResponse};

pub mod diesel;
pub mod endpoint;
pub mod grpc;
pub mod primitives;
pub mod solicitor;

pub struct Vacant;

#[async_trait]
impl OwnerSolicitor<OAuthRequest> for Vacant {
    async fn check_consent(
        &mut self,
        _req: &mut OAuthRequest,
        _solicitation: Solicitation<'_>,
    ) -> OwnerConsent<OAuthResponse> {
        OwnerConsent::Denied
    }
}

impl oxide_auth_async::endpoint::Extension for Vacant {
    fn access_token(
        &mut self,
    ) -> Option<&mut (dyn oxide_auth_async::endpoint::AccessTokenExtension + Send)> {
        return Some(self);
    }
}

#[async_trait]
impl oxide_auth_async::endpoint::AccessTokenExtension for Vacant {
    async fn extend(
        &mut self,
        _request: &(dyn Request + Sync),
        _data: Extensions,
    ) -> std::result::Result<Extensions, ()> {
        let result = Extensions::new();
        Ok(result)
    }
}
