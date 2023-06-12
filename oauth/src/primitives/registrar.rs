use async_trait::async_trait;
use oxide_auth::{
    endpoint::{PreGrant, Registrar, Scope},
    primitives::registrar::{BoundClient, ClientUrl, RegistrarError},
};
use oxide_auth_async::primitives::Registrar as RegistrarAsync;

use super::Guard;

#[async_trait]
impl<T> RegistrarAsync for Guard<'_, T>
where
    T: Registrar + Send + Sync,
{
    async fn bound_redirect<'a>(
        &self,
        bound: ClientUrl<'a>,
    ) -> Result<BoundClient<'a>, RegistrarError> {
        Registrar::bound_redirect(&**self, bound)
    }

    async fn negotiate<'a>(
        &self,
        client: BoundClient<'a>,
        scope: Option<Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        Registrar::negotiate(&**self, client, scope)
    }

    async fn check(
        &self,
        client_id: &str,
        passphrase: Option<&[u8]>,
    ) -> Result<(), RegistrarError> {
        Registrar::check(&**self, client_id, passphrase)
    }
}
