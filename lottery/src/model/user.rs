use crate::pb::user as pb;
use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use uuid::Uuid;

use crate::schema::wx_users;

pub async fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::WxUser>> {
    let wx_users = wx_users::table.load::<pb::WxUser>(conn)?;
    Ok(wx_users)
}
pub async fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::WxUser> {
    let user = wx_users::table.find(id).first::<pb::WxUser>(conn)?;
    Ok(user)
}
pub async fn query_by_openid(conn: &mut PgConnection, openid: String) -> Result<pb::WxUser> {
    let user = wx_users::table
        .filter(wx_users::openid.eq(openid))
        .first::<pb::WxUser>(conn)?;
    Ok(user)
}
pub async fn insert(conn: &mut PgConnection, user: pb::NewWxUser) -> Result<pb::WxUser> {
    let user = diesel::insert_into(wx_users::table)
        .values(user)
        .get_result::<pb::WxUser>(conn)?;
    Ok(user)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::NewWxUser,
) -> Result<pb::WxUser> {
    let user = diesel::update(wx_users::table)
        .filter(wx_users::dsl::id.eq(id))
        .set(user)
        .get_result::<pb::WxUser>(conn)?;
    Ok(user)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(wx_users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
