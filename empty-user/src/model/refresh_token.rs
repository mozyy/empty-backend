use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::refresh_tokens;

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(refresh_token))]
pub struct RefreshToken {
    pub refresh_token: String,
    pub info_id: Uuid,
    pub expires_in: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name =refresh_tokens,primary_key(refresh_token))]
pub struct NewRefreshToken {
    pub refresh_token: String,
    pub info_id: Uuid,
    pub expires_in: i32,
}

impl NewRefreshToken {
    pub fn new(info_id: Uuid) -> Self {
        Self {
            refresh_token: Uuid::new_v4().to_string(),
            info_id,
            expires_in: 7 * 24 * 60 * 60, // 7 days
        }
    }
}
