use crate::schema::infos::{self};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::{diesel::timestamp, errors::ServiceError};
use serde::Serialize;
use tonic::codegen::ok;
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

pub fn query_by_mobile(conn: &mut PgConnection, mobile: String) -> Result<Info, ServiceError> {
    let info = infos::dsl::infos
        .filter(infos::mobile.eq(mobile))
        .first(conn)?;
    Ok(info)
}
