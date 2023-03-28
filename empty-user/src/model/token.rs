use crate::{pb, schema::tokens};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::{convert::naive_date_time_to_timestamp, diesel::timestamp, errors::ServiceError};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(primary_key(access_token), belongs_to(super::info::Info))]
pub struct Token {
    pub access_token: String,
    pub info_id: Uuid,
    pub expires_in: i32,
    pub refresh_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Token> for pb::Token {
    fn from(value: Token) -> Self {
        let Token {
            access_token,
            info_id,
            expires_in,
            refresh_token,
            created_at,
            updated_at,
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
#[diesel(table_name =tokens)]
pub struct NewToken {
    pub access_token: String,
    pub info_id: Uuid,
    pub expires_in: i32,
    pub refresh_token: String,
}
impl NewToken {
    pub fn new(info_id: Uuid) -> Self {
        Self {
            access_token: Uuid::new_v4().to_string(),
            info_id,
            expires_in: 3600,
            refresh_token: Uuid::new_v4().to_string(),
        }
    }
}

pub fn insert(conn: &mut PgConnection, token: NewToken) -> Result<Token, ServiceError> {
    let id = diesel::insert_into(tokens::dsl::tokens)
        .values(token)
        .get_result(conn)?;
    Ok(id)
}
