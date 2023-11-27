use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::auth::Config>> {
    let users = schema::user::configs::table.load::<pb::user::auth::Config>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::auth::Config> {
    let user = schema::user::configs::table
        .find(id)
        .first::<pb::user::auth::Config>(conn)?;
    Ok(user)
}

pub fn insert(
    conn: &mut PgConnection,
    config: pb::user::auth::NewConfig,
) -> Result<pb::user::auth::Config> {
    let user = diesel::insert_into(schema::user::configs::table)
        .values(config)
        .get_result::<pb::user::auth::Config>(conn)?;
    Ok(user)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    config: pb::user::auth::NewConfig,
) -> Result<pb::user::auth::Config> {
    let user = diesel::update(schema::user::configs::table.find(id))
        .set(config)
        .get_result::<pb::user::auth::Config>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::configs::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
