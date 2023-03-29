use crate::{pb, schema::access_tokens};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::{convert::naive_date_time_to_timestamp, errors::ServiceError};

use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(primary_key(access_token), belongs_to(super::info::Info))]
pub struct AccessToken {
    pub access_token: String,
    pub info_id: Uuid,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<AccessToken> for pb::Token {
    fn from(value: AccessToken) -> Self {
        let AccessToken {
            access_token,
            expires_in,
            refresh_token,
            created_at,
            updated_at,
            ..
        } = value;
        Self {
            access_token,
            expires_in,
            refresh_token,
            created_at: Some(naive_date_time_to_timestamp(created_at)),
            updated_at: Some(naive_date_time_to_timestamp(updated_at)),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = access_tokens)]
pub struct NewAccessToken {
    pub access_token: String,
    pub info_id: Uuid,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
}

impl NewAccessToken {
    pub fn new(info_id: Uuid, scope: String, refresh_token: String) -> Self {
        Self {
            access_token: Uuid::new_v4().to_string(),
            info_id,
            scope,
            expires_in: 1 * 60 * 60, // one hour
            refresh_token,
        }
    }
}

pub fn query_by_access_token(
    conn: &mut PgConnection,
    access_token: String,
) -> Result<AccessToken, ServiceError> {
    let access_token = access_tokens::table.find(access_token).first(conn)?;
    Ok(access_token)
}

pub fn delete_by_access_token(
    conn: &mut PgConnection,
    access_token: String,
) -> Result<(), ServiceError> {
    diesel::delete(access_tokens::table.find(access_token)).execute(conn)?;
    Ok(())
}
