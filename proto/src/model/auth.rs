use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};

use empty_utils::errors::ErrorConvert;
use rand::{rngs::OsRng, RngCore};

use crate::pb;

impl pb::auth::auth::NewResource {
    pub fn generate(user: &pb::auth::auth::User, client: &pb::auth::auth::Client) -> Self {
        let mut access_token = [0; 16];
        let mut refresh_token = [0; 16];
        OsRng.fill_bytes(&mut access_token);
        OsRng.fill_bytes(&mut refresh_token);
        let access_token = general_purpose::STANDARD.encode(access_token);
        let refresh_token = general_purpose::STANDARD.encode(refresh_token);
        let until = Utc::now() + Duration::seconds(client.default_expires_in as i64);

        let until = prost_types::Timestamp {
            seconds: until.timestamp(),
            nanos: 0,
        };
        Self {
            user_id: user.id.to_owned(),
            client_id: client.id.to_owned(),
            access_token,
            refresh_token,
            scope: client.default_scope.to_owned(),
            token_type: "Bearer".into(),
            until: Some(until),
        }
    }
}

impl From<&pb::auth::auth::Resource> for pb::auth::auth::User {
    fn from(value: &pb::auth::auth::Resource) -> Self {
        Self {
            id: value.user_id.to_owned(),
            created_at: value.created_at.to_owned(),
            updated_at: value.updated_at.to_owned(),
        }
    }
}
impl From<&pb::auth::auth::Resource> for pb::auth::auth::Token {
    fn from(value: &pb::auth::auth::Resource) -> Self {
        let expires_in =
            value.until.to_owned().ok_or_loss().unwrap().seconds - Utc::now().timestamp();
        Self {
            access_token: value.access_token.to_owned(),
            refresh_token: value.refresh_token.to_owned(),
            scope: value.scope.to_owned(),
            token_type: value.token_type.to_owned(),
            expires_in: expires_in as i32,
        }
    }
}
