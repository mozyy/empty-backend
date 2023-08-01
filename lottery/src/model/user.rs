use crate::pb;
use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use uuid::Uuid;

use crate::schema::wx_users;

pub async fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::WxUser>> {
    let wx_users = wx_users::table.load::<pb::user::WxUser>(conn)?;
    Ok(wx_users)
}
pub async fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::user::WxUser> {
    let user = wx_users::table.find(id).first::<pb::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn query_by_openid(conn: &mut PgConnection, openid: String) -> Result<pb::user::WxUser> {
    let user = wx_users::table
        .filter(wx_users::openid.eq(openid))
        .first::<pb::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn insert(
    conn: &mut PgConnection,
    user: pb::user::NewWxUser,
) -> Result<pb::user::WxUser> {
    let user = diesel::insert_into(wx_users::table)
        .values(user)
        .get_result::<pb::user::WxUser>(conn)?;
    Ok(user)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::user::NewWxUser,
) -> Result<pb::user::WxUser> {
    let user = diesel::update(wx_users::table)
        .filter(wx_users::dsl::id.eq(id))
        .set(user)
        .get_result::<pb::user::WxUser>(conn)?;
    Ok(user)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(wx_users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
