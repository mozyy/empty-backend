use chrono::Timelike;
use oxide_auth::primitives::grant::Grant;
use prost_types::Timestamp;

use crate::pb::oauth as pb;

pub mod error;
pub mod request;
pub mod response;

impl From<Grant> for pb::ResourceResponse {
    fn from(value: Grant) -> Self {
        let Grant {
            owner_id,
            client_id,
            scope,
            redirect_uri,
            until,
            extensions,
        } = value;
        Self {
            owner_id,
            client_id,
            scope: scope.to_string(),
            redirect_uri: redirect_uri.to_string(),
            until: Some(Timestamp {
                seconds: until.timestamp(),
                nanos: until.nanosecond() as i32,
            }),
            extensions: extensions
                .public()
                .filter_map(|(key, value)| value.map(|value| (key.to_owned(), value.to_owned())))
                .collect(),
        }
    }
}
