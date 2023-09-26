use chrono::{Duration, Timelike, Utc};
use oxide_auth::primitives::grant::Grant;
use prost_types::Timestamp;
use rand::{rngs::OsRng, RngCore};

use crate::pb;

impl pb::auth::auth::NewResource {
    pub fn generate(user: pb::auth::auth::User, client: pb::auth::auth::Client) -> Self {
        let mut access_token = [0; 16];
        let mut refresh_token = [0; 16];
        OsRng.fill_bytes(&mut access_token);
        OsRng.fill_bytes(&mut refresh_token);
        let access_token = base64::encode(access_token);
        let refresh_token = base64::encode(refresh_token);
        let until = Utc::now() + Duration::seconds(client.default_expires_in as i64);

        let until = prost_types::Timestamp::from(until);
        let until = prost_types::datetime::DateTime
        Self {
            user_id: user.id,
            client_id: client.id,
            access_token: access_token,
            refresh_token: refresh_token,
            scope: client.default_scope,
            token_type: "Bearer".into(),
            until: Some(until),
        }
    }
}
