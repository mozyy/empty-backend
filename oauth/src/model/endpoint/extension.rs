use oxide_auth::{
    code_grant::accesstoken::Request,
    primitives::grant::Extensions,
};
use oxide_auth_async::endpoint;

use uuid::Uuid;


use async_trait::async_trait;

pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl endpoint::Extension for UserId {
    fn access_token(&mut self) -> Option<&mut (dyn endpoint::AccessTokenExtension + Send)> {
        return Some(self);
    }
}

#[async_trait]
impl endpoint::AccessTokenExtension for UserId {
    async fn extend(
        &mut self,
        _request: &(dyn Request + Sync),
        _data: Extensions,
    ) -> std::result::Result<Extensions, ()> {
        let mut result = Extensions::new();
        result.set_raw(
            "user_id".into(),
            oxide_auth::primitives::grant::Value::Public(Some(self.0.to_string())),
        );
        Ok(result)
    }
}
