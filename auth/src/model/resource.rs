use std::collections::HashMap;

use chrono::Utc;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb;

type ResourceItem = (pb::auth::auth::User, pb::auth::auth::Token);
type ResourceMap = HashMap<String, ResourceItem>;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Resource(ResourceMap);

impl TryFrom<Vec<pb::auth::auth::Resource>> for Resource {
    type Error = Error;

    fn try_from(value: Vec<pb::auth::auth::Resource>) -> Result<Self> {
        let hash_map = value
            .into_iter()
            .map(|item| {
                let user = pb::auth::auth::User {
                    id: item.user_id,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                };
                let expires_in = item.until.ok_or_loss()?.seconds - Utc::now().timestamp();
                let token = pb::auth::auth::Token {
                    access_token: item.access_token.clone(),
                    refresh_token: item.refresh_token,
                    scope: item.scope,
                    token_type: item.token_type,
                    expires_in: expires_in as i32,
                };
                Ok((item.access_token, (user, token)))
            })
            .collect::<Result<ResourceMap>>()?;
        Ok(Self(hash_map))
    }
}
