use crate::pb::user as pb;
use diesel::prelude::*;
use empty_utils::errors::{ServiceError, ServiceResult};
use uuid::Uuid;

use crate::schema::users;

pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<pb::User>> {
    let users = users::table.load::<pb::User>(conn)?;
    Ok(users)
}
pub async fn query_by_id(conn: &mut PgConnection, id: Uuid) -> ServiceResult<pb::User> {
    let user = users::table.find(id).first::<pb::User>(conn)?;
    Ok(user)
}
pub async fn query_by_openid(conn: &mut PgConnection, openid: String) -> ServiceResult<pb::User> {
    let user = users::table
        .filter(users::openid.eq(openid))
        .first::<pb::User>(conn)?;
    Ok(user)
}
pub async fn insert(conn: &mut PgConnection, user: pb::NewUser) -> ServiceResult<pb::User> {
    let user = diesel::insert_into(users::table)
        .values(user)
        .get_result::<pb::User>(conn)?;
    Ok(user)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::NewUser,
) -> ServiceResult<pb::User> {
    let user = diesel::update(users::table)
        .filter(users::dsl::id.eq(id))
        .set(user)
        .get_result::<pb::User>(conn)?;
    Ok(user)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> ServiceResult {
    let value = diesel::delete(users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(ServiceError::String(String::from("delete error")));
    }
    Ok(())
}
