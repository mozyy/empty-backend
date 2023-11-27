use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::user::Weixin>> {
    let users = schema::user::weixins::table.load::<pb::user::user::Weixin>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::user::Weixin> {
    let user = schema::user::weixins::table
        .find(id)
        .first::<pb::user::user::Weixin>(conn)?;
    Ok(user)
}

pub fn insert(
    conn: &mut PgConnection,
    user: pb::user::user::NewWeixin,
) -> Result<pb::user::user::Weixin> {
    let user = diesel::insert_into(schema::user::weixins::table)
        .values(user)
        .get_result::<pb::user::user::Weixin>(conn)?;
    Ok(user)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    user: pb::user::user::NewWeixin,
) -> Result<pb::user::user::Weixin> {
    let user = diesel::update(schema::user::weixins::table.find(id))
        .set(user)
        .get_result::<pb::user::user::Weixin>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::weixins::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
