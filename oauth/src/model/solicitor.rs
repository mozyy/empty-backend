use async_trait::async_trait;
use oxide_auth::endpoint::{OwnerConsent, Solicitation};
use oxide_auth_async::endpoint::OwnerSolicitor;

use super::grpc::{request::OAuthRequest, response::OAuthResponse};

pub struct Solicitor;

#[async_trait]
impl OwnerSolicitor<OAuthRequest> for Solicitor {
    async fn check_consent(
        &mut self,
        _req: &mut OAuthRequest,
        _solicitation: Solicitation<'_>,
    ) -> OwnerConsent<OAuthResponse> {
        todo!()
    }
}
