use chrono::Utc;
use diesel::prelude::*;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb;

use uuid::Uuid;

use proto::{schema, utils::diesel::Paginate};

pub fn query_by_id(conn: &mut PgConnection, id: String) -> Result<pb::auth::auth::User> {
    let id = id.parse::<Uuid>().ok_or_invalid()?;
    let user: pb::auth::auth::User = schema::auth::users::table.find(id).first(conn)?;
    Ok(user)
}
pub fn query_all(conn: &mut PgConnection) -> Result<Vec<pb::auth::auth::User>> {
    let users = schema::auth::users::table.load(conn)?;
    Ok(users)
}
pub fn insert(conn: &mut PgConnection) -> Result<pb::auth::auth::User> {
    let user = diesel::insert_into(schema::auth::users::table)
        .default_values()
        .get_result(conn)?;
    Ok(user)
}
