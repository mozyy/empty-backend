use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use empty_utils::errors::ServiceError;
use uuid::Uuid;

use crate::schema::{access_tokens, refresh_tokens};

use super::{
    access_token::{AccessToken, NewAccessToken},
    refresh_token::NewRefreshToken,
};

pub struct NewToken {
    new_access_token: NewAccessToken,
    new_refresh_token: NewRefreshToken,
}

impl NewToken {
    pub fn new(info_id: Uuid, scope: String) -> Self {
        let new_refresh_token = NewRefreshToken::new(info_id);
        let new_access_token =
            NewAccessToken::new(info_id, scope, new_refresh_token.refresh_token.clone());
        Self {
            new_access_token,
            new_refresh_token,
        }
    }
}

pub fn insert(conn: &mut PgConnection, token: NewToken) -> Result<AccessToken, ServiceError> {
    let access_token = conn.transaction::<_, ServiceError, _>(|conn| {
        diesel::insert_into(refresh_tokens::dsl::refresh_tokens)
            .values(token.new_refresh_token)
            .execute(conn)?;
        let access_token = diesel::insert_into(access_tokens::dsl::access_tokens)
            .values(token.new_access_token)
            .get_result(conn)?;
        Ok(access_token)
        // todo!()
    })?;

    Ok(access_token)
}
