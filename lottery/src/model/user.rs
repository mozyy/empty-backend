use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub async fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::wx::user::WxUser>> {
    let wx_users = schema::wx::wx_users::table.load::<pb::wx::user::WxUser>(conn)?;
    Ok(wx_users)
}
pub async fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::wx::user::WxUser> {
    let user = schema::wx::wx_users::table
        .find(id)
        .first::<pb::wx::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn query_by_openid(
    conn: &mut PgConnection,
    openid: String,
) -> Result<pb::wx::user::WxUser> {
    let user = schema::wx::wx_users::table
        .filter(schema::wx::wx_users::openid.eq(openid))
        .first::<pb::wx::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn insert(
    conn: &mut PgConnection,
    user: pb::wx::user::NewWxUser,
) -> Result<pb::wx::user::WxUser> {
    let user = diesel::insert_into(schema::wx::wx_users::table)
        .values(user)
        .get_result::<pb::wx::user::WxUser>(conn)?;
    Ok(user)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::wx::user::NewWxUser,
) -> Result<pb::wx::user::WxUser> {
    let user = diesel::update(schema::wx::wx_users::table)
        .filter(schema::wx::wx_users::dsl::id.eq(id))
        .set(user)
        .get_result::<pb::wx::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(schema::wx::wx_users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
