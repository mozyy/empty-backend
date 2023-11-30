use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::user::Mobile>> {
    let users = schema::user::mobiles::table.load::<pb::user::user::Mobile>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::user::Mobile> {
    let user = schema::user::mobiles::table
        .find(id)
        .first::<pb::user::user::Mobile>(conn)?;
    Ok(user)
}
pub fn query_by_mobile(conn: &mut PgConnection, mobile: String) -> Result<pb::user::user::Mobile> {
    let user = schema::user::mobiles::table
        .filter(schema::user::mobiles::mobile.eq(mobile))
        .first::<pb::user::user::Mobile>(conn)?;
    Ok(user)
}
pub fn insert(
    conn: &mut PgConnection,
    mobild: pb::user::user::NewMobile,
) -> Result<pb::user::user::Mobile> {
    let user = diesel::insert_into(schema::user::mobiles::table)
        .values(mobild)
        .get_result::<pb::user::user::Mobile>(conn)?;
    Ok(user)
}

pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    mobile: pb::user::user::NewMobile,
) -> Result<pb::user::user::Mobile> {
    let user = diesel::update(schema::user::mobiles::table.find(id))
        .set(mobile)
        .get_result::<pb::user::user::Mobile>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::mobiles::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
