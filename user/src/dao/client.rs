use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::{pb, schema};
use uuid::Uuid;

pub fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::user::auth::Client>> {
    let users = schema::user::clients::table.load::<pb::user::auth::Client>(conn)?;
    Ok(users)
}
pub fn query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::user::auth::Client> {
    let user = schema::user::clients::table
        .find(id)
        .first::<pb::user::auth::Client>(conn)?;
    Ok(user)
}

pub fn insert(
    conn: &mut PgConnection,
    client: pb::user::auth::NewClient,
) -> Result<pb::user::auth::Client> {
    let user = diesel::insert_into(schema::user::clients::table)
        .values(client)
        .get_result::<pb::user::auth::Client>(conn)?;
    Ok(user)
}

pub fn update_by_id(
    conn: &mut PgConnection,
    id: Uuid,
    client: pb::user::auth::NewClient,
) -> Result<pb::user::auth::Client> {
    let user = diesel::update(schema::user::clients::table.find(id))
        .set(client)
        .get_result::<pb::user::auth::Client>(conn)?;
    Ok(user)
}

pub fn delete_by_id(conn: &mut PgConnection, id: Uuid) -> Result {
    let value = diesel::delete(schema::user::clients::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
