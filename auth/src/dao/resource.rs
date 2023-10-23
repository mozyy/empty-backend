use chrono::Utc;
use diesel::dsl::now;
use diesel::prelude::*;
use empty_utils::errors::{ErrorConvert, Result};
use proto::pb;

use proto::schema;

pub fn query_by_token(
    conn: &mut PgConnection,
    access_token: String,
) -> Result<(pb::auth::auth::User, pb::auth::auth::Token)> {
    let resource: pb::auth::auth::Resource = schema::auth::resources::table
        .filter(schema::auth::resources::access_token.eq(access_token))
        .first(conn)?;
    let user = super::user::query_by_id(conn, resource.user_id)?;
    let until = resource.until.ok_or_loss()?;
    let token = pb::auth::auth::Token {
        access_token: resource.access_token,
        refresh_token: resource.refresh_token,
        scope: resource.scope,
        token_type: resource.token_type,
        expires_in: (until.seconds - Utc::now().timestamp()) as i32,
    };
    Ok((user, token))
}
pub fn query_all(conn: &mut PgConnection) -> Result<Vec<pb::auth::auth::Resource>> {
    let resource = schema::auth::resources::table.load(conn)?;
    Ok(resource)
}
pub fn insert(
    conn: &mut PgConnection,
    resource: pb::auth::auth::NewResource,
) -> Result<pb::auth::auth::Resource> {
    let resource = diesel::insert_into(schema::auth::resources::table)
        .values(resource)
        .get_result(conn)?;
    Ok(resource)
}

pub fn delete_invalid(conn: &mut PgConnection) -> Result<()> {
    diesel::delete(schema::auth::resources::table.filter(schema::auth::resources::until.le(now)))
        .execute(conn)?;
    Ok(())
}
