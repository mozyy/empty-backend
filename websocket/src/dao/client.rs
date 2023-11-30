use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::websocket::client::User>> {
    let users = schema::websocket::users::table.load::<pb::websocket::client::User>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::websocket::client::User> {
    let user = schema::websocket::users::table
        .find(id)
        .first::<pb::websocket::client::User>(conn)?;
    Ok(user)
}
pub fn query_by_user_id(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<pb::websocket::client::User> {
    let user = schema::websocket::users::table
        .filter(schema::websocket::users::user_id.eq(user_id))
        .first::<pb::websocket::client::User>(conn)?;
    Ok(user)
}

pub fn insert(
    conn: &mut PgConnection,
    user: pb::websocket::client::NewUser,
) -> Result<pb::websocket::client::User> {
    let user = diesel::insert_into(schema::websocket::users::table)
        .values(user)
        .get_result::<pb::websocket::client::User>(conn)?;
    Ok(user)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    user: pb::websocket::client::NewUser,
) -> Result<pb::websocket::client::User> {
    let user = diesel::update(schema::websocket::users::table.find(id))
        .set(user)
        .get_result::<pb::websocket::client::User>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::websocket::users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
