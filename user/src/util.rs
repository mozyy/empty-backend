use std::{cmp, env};

use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};
use empty_utils::errors::{ErrorConvert, Result};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use proto::pb;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

pub fn get_jwt_key() -> Result<Hmac<Sha256>> {
    Hmac::new_from_slice(env::var("JWT_KEY").ok_or_loss()?.as_bytes()).ok_or_invalid()
}

pub fn gen_resource_token(
    user_id: String,
    client_id: String,
    client_name: String,
    client_expires_in: i32,
    scope: String,
) -> Result<(pb::user::auth::NewRefreshResource, pb::user::auth::Token)> {
    let mut refresh_token = [0; 16];
    OsRng.fill_bytes(&mut refresh_token);
    let refresh_token = general_purpose::STANDARD.encode(refresh_token);
    let now = Utc::now();
    let until = now + Duration::seconds(client_expires_in as i64);
    let until = Some(prost_types::Timestamp {
        seconds: until.timestamp(),
        nanos: 0,
    });
    let token_type = String::from("Bearer");
    let refresh_resource = pb::user::auth::NewRefreshResource {
        user_id: user_id.clone(),
        client_id,
        refresh_token: refresh_token.clone(),
        scope: scope.clone(),
        token_type: token_type.clone(),
        until,
    };
    let exp_expires_in = cmp::max(client_expires_in / 10, 60 * 60 * 1000);
    let now = now.timestamp();
    let exp = now + exp_expires_in as i64;
    let mut jti = [0; 16];
    OsRng.fill_bytes(&mut jti);
    let jti = general_purpose::STANDARD.encode(jti);
    let jwt_payload = pb::user::auth::JwtPayload {
        iss: String::from("zuoyinyun.com"),
        aud: client_name,
        sub: user_id,
        sco: scope.clone(),
        exp,
        nbf: now,
        iat: now,
        jti,
    };

    let key = get_jwt_key()?;
    let access_token = jwt_payload.sign_with_key(&key).ok_or_invalid()?;

    let token = pb::user::auth::Token {
        access_token,
        token_type,
        expires_in: client_expires_in,
        refresh_token,
        scope,
    };
    Ok((refresh_resource, token))
}
