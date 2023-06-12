use std::{ops::DerefMut, sync::Arc};

use async_trait::async_trait;
use oxide_auth::{
    endpoint::Authorizer,
    primitives::{
        grant::Grant,
        prelude::{AuthMap, TagGrant},
    },
};
use oxide_auth_async::primitives::Authorizer as AuthorizerAsync;

use super::Guard;

#[async_trait]
impl<T> AuthorizerAsync for Guard<'_, T>
where
    T: Authorizer + Send,
{
    async fn authorize(&mut self, grant: Grant) -> Result<String, ()> {
        Authorizer::authorize(&mut **self, grant)
    }

    async fn extract(&mut self, grant: &str) -> Result<Option<Grant>, ()> {
        Authorizer::extract(&mut **self, grant)
    }
}
