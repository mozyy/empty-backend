use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub async fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::wx::user::User>> {
    let users = schema::wx::users::table.load::<pb::wx::user::User>(conn)?;
    Ok(users)
}
pub async fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::wx::user::User> {
    let user = schema::wx::users::table
        .find(id)
        .first::<pb::wx::user::User>(conn)?;
    Ok(user)
}
pub async fn query_by_openid(
    conn: &mut PgConnection,
    openid: String,
) -> Result<pb::wx::user::User> {
    let user = schema::wx::users::table
        .filter(schema::wx::users::openid.eq(openid))
        .first::<pb::wx::user::User>(conn)?;
    Ok(user)
}
pub async fn insert(
    conn: &mut PgConnection,
    user: pb::wx::user::NewUser,
) -> Result<pb::wx::user::User> {
    let user = diesel::insert_into(schema::wx::users::table)
        .values(user)
        .get_result::<pb::wx::user::User>(conn)?;
    Ok(user)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    user: pb::wx::user::NewUser,
) -> Result<pb::wx::user::User> {
    let user = diesel::update(schema::wx::users::table)
        .filter(schema::wx::users::dsl::id.eq(id))
        .set(user)
        .get_result::<pb::wx::user::User>(conn)?;
    Ok(user)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(schema::wx::users::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
