use async_trait::async_trait;
use oxide_auth::{
    endpoint::Issuer,
    primitives::{grant::Grant, issuer::RefreshedToken, prelude::IssuedToken},
};
use oxide_auth_async::primitives::Issuer as IssuerAsync;

use super::Guard;

#[async_trait]
impl<T> IssuerAsync for Guard<'_, T>
where
    T: Issuer + Send,
{
    async fn issue(&mut self, grant: Grant) -> Result<IssuedToken, ()> {
        Issuer::issue(&mut **self, grant)
    }

    async fn refresh(&mut self, token: &str, grant: Grant) -> Result<RefreshedToken, ()> {
        Issuer::refresh(&mut **self, token, grant)
    }

    async fn recover_token(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        Issuer::recover_token(&mut **self, token)
    }

    async fn recover_refresh(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        Issuer::recover_refresh(&mut **self, token)
    }
}
