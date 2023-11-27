use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::auth::AuthorizationCode>> {
    let users =
        schema::user::authorization_codes::table.load::<pb::user::auth::AuthorizationCode>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::auth::AuthorizationCode> {
    let user = schema::user::authorization_codes::table
        .find(id)
        .first::<pb::user::auth::AuthorizationCode>(conn)?;
    Ok(user)
}
pub fn query_by_code(
    conn: &mut PgConnection,
    code: String,
) -> Result<pb::user::auth::AuthorizationCode> {
    let user = schema::user::authorization_codes::table
        .filter(schema::user::authorization_codes::code.eq(code))
        .first::<pb::user::auth::AuthorizationCode>(conn)?;
    Ok(user)
}
pub fn insert(
    conn: &mut PgConnection,
    auth: pb::user::auth::NewAuthorizationCode,
) -> Result<pb::user::auth::AuthorizationCode> {
    let user = diesel::insert_into(schema::user::authorization_codes::table)
        .values(auth)
        .get_result::<pb::user::auth::AuthorizationCode>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::authorization_codes::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
