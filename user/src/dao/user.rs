use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::user::User>> {
    let users = schema::user::users::table.load::<pb::user::user::User>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::user::user::User> {
    let user = schema::user::users::table
        .find(id)
        .first::<pb::user::user::User>(conn)?;
    Ok(user)
}
pub fn insert(
    conn: &mut PgConnection,
    user: pb::user::user::NewUser,
) -> Result<pb::user::user::User> {
    let user = diesel::insert_into(schema::user::users::table)
        .values(user)
        .get_result::<pb::user::user::User>(conn)?;
    Ok(user)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::user::user::NewUser,
) -> Result<pb::user::user::User> {
    let user = diesel::update(schema::user::users::table.find(id))
        .set(user)
        .get_result::<pb::user::user::User>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(schema::user::users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
