use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::auth::RefreshResource>> {
    let users =
        schema::user::refresh_resources::table.load::<pb::user::auth::RefreshResource>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::user::auth::RefreshResource> {
    let user = schema::user::refresh_resources::table
        .find(id)
        .first::<pb::user::auth::RefreshResource>(conn)?;
    Ok(user)
}
pub fn query_by_refresh_token(
    conn: &mut PgConnection,
    refresh_token: String,
) -> Result<pb::user::auth::RefreshResource> {
    let user = schema::user::refresh_resources::table
        .filter(schema::user::refresh_resources::refresh_token.eq(refresh_token))
        .first::<pb::user::auth::RefreshResource>(conn)?;
    Ok(user)
}
pub fn insert(
    conn: &mut PgConnection,
    resource: pb::user::auth::NewRefreshResource,
) -> Result<pb::user::auth::RefreshResource> {
    let user = diesel::insert_into(schema::user::refresh_resources::table)
        .values(resource)
        .get_result::<pb::user::auth::RefreshResource>(conn)?;
    Ok(user)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::user::refresh_resources::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
pub fn delete_by_refresh_token(conn: &mut PgConnection, refresh_token: String) -> Result {
    let value = diesel::delete(
        schema::user::refresh_resources::table
            .filter(schema::user::refresh_resources::refresh_token.eq(refresh_token)),
    )
    .execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
