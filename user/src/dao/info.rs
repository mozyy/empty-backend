use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::user::Info>> {
    let users = schema::user::infos::table.load::<pb::user::user::Info>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::user::Info> {
    let user = schema::user::infos::table
        .find(id)
        .first::<pb::user::user::Info>(conn)?;
    Ok(user)
}
pub fn insert(
    conn: &mut PgConnection,
    info: pb::user::user::NewInfo,
) -> Result<pb::user::user::Info> {
    let user = diesel::insert_into(schema::user::infos::table)
        .values(info)
        .get_result::<pb::user::user::Info>(conn)?;
    Ok(user)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    info: pb::user::user::NewInfo,
) -> Result<pb::user::user::Info> {
    let user = diesel::update(schema::user::infos::table.find(id))
        .set(info)
        .get_result::<pb::user::user::Info>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::infos::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
