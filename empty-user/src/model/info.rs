use crate::{
    pb,
    schema::infos::{self},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::{convert::naive_date_time_to_timestamp, diesel::timestamp, errors::ServiceError};
use serde::Serialize;

use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Info {
    pub id: Uuid,
    pub mobile: String,
    pub password: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}

impl From<Info> for pb::Info {
    fn from(value: Info) -> Self {
        let Info {
            id,
            mobile,
            username,
            avatar,
            created_at,
            updated_at,
            ..
        } = value;
        Self {
            id: id.to_string(),
            mobile,
            username: username.unwrap_or_default(),
            avatar: avatar.unwrap_or_default(),
            created_at: Some(naive_date_time_to_timestamp(created_at)),
            updated_at: Some(naive_date_time_to_timestamp(updated_at)),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name =infos)]
pub struct NewInfo {
    pub mobile: String,
    pub password: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}
impl NewInfo {
    pub fn new(mobile: String, password: String) -> Self {
        Self {
            mobile,
            password,
            username: None,
            avatar: None,
        }
    }
}

pub fn insert(conn: &mut PgConnection, info: NewInfo) -> Result<Uuid, ServiceError> {
    let id = diesel::insert_into(infos::dsl::infos)
        .values(info)
        .returning(infos::id)
        .get_result(conn)?;
    Ok(id)
}

pub fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Info, ServiceError> {
    let info = infos::dsl::infos.find(id).first(conn)?;
    Ok(info)
}

pub fn query_by_mobile(conn: &mut PgConnection, mobile: String) -> Result<Info, ServiceError> {
    let info = infos::dsl::infos
        .filter(infos::mobile.eq(mobile))
        .first(conn)?;
    Ok(info)
}
