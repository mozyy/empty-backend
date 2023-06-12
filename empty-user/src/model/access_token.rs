use crate::{pb, schema::access_tokens};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::{convert::naive_date_time_to_timestamp, errors::ServiceResult};

use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(primary_key(access_token), belongs_to(super::info::Info))]
pub struct AccessToken {
    pub access_token: String,
    pub info_id: Uuid,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub refresh_expires_in: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AccessToken {
    pub fn refresh(self) -> NewAccessToken {
        let Self {
            info_id,
            scope,
            expires_in,
            refresh_expires_in,
            ..
        } = self;
        NewAccessToken {
            access_token: Uuid::new_v4().to_string(),
            refresh_token: Uuid::new_v4().to_string(),
            info_id,
            scope,
            expires_in,
            refresh_expires_in,
        }
    }
}

impl From<AccessToken> for pb::AccessToken {
    fn from(value: AccessToken) -> Self {
        let AccessToken {
            access_token,
            expires_in,
            refresh_token,
            refresh_expires_in,
            created_at,
            updated_at,
            ..
        } = value;
        Self {
            access_token,
            expires_in,
            refresh_token,
            refresh_expires_in,
            created_at: Some(naive_date_time_to_timestamp(created_at)),
            updated_at: Some(naive_date_time_to_timestamp(updated_at)),
        }
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = access_tokens)]
pub struct NewAccessToken {
    pub access_token: String,
    pub info_id: Uuid,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub refresh_expires_in: i32,
}

impl NewAccessToken {
    pub fn new(info_id: Uuid, scope: String) -> Self {
        Self {
            access_token: Uuid::new_v4().to_string(),
            info_id,
            scope,
            expires_in: 1 * 60 * 60, // one hour
            refresh_token: Uuid::new_v4().to_string(),
            refresh_expires_in: 7 * 24 * 60 * 60, // 7 days
        }
    }
}

pub fn insert(conn: &mut PgConnection, access_token: NewAccessToken) -> ServiceResult<AccessToken> {
    let access_token = diesel::insert_into(access_tokens::dsl::access_tokens)
        .values(access_token)
        .get_result(conn)?;
    Ok(access_token)
}

pub fn query_by_access_token(
    conn: &mut PgConnection,
    access_token: String,
) -> ServiceResult<AccessToken> {
    let access_token = access_tokens::table.find(access_token).first(conn)?;
    Ok(access_token)
}

pub fn query_by_refresh_token(
    conn: &mut PgConnection,
    refresh_token: String,
) -> ServiceResult<AccessToken> {
    let refresh_token = access_tokens::table.find(refresh_token).first(conn)?;
    Ok(refresh_token)
}

pub fn delete_by_access_token(conn: &mut PgConnection, access_token: String) -> ServiceResult {
    diesel::delete(access_tokens::table.find(access_token)).execute(conn)?;
    Ok(())
}

pub fn delete_by_refresh_token(conn: &mut PgConnection, refresh_token: String) -> ServiceResult {
    diesel::delete(access_tokens::table.filter(access_tokens::refresh_token.eq(refresh_token)))
        .execute(conn)?;
    Ok(())
}
