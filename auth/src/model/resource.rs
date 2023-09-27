use std::collections::HashMap;

use chrono::Utc;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb;

type ResourceMap = HashMap<String, pb::auth::auth::Resource>;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Resource(ResourceMap);

impl TryFrom<Vec<pb::auth::auth::Resource>> for Resource {
    type Error = Error;

    fn try_from(value: Vec<pb::auth::auth::Resource>) -> Result<Self> {
        let hash_map = value
            .into_iter()
            .map(|item| Ok((item.access_token.clone(), item)))
            .collect::<Result<ResourceMap>>()?;
        Ok(Self(hash_map))
    }
}

impl std::ops::Deref for Resource {
    type Target = ResourceMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
