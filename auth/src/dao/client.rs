use diesel::prelude::*;
use empty_utils::errors::{ErrorConvert, Result};
use proto::pb;

use uuid::Uuid;

use proto::schema;

pub fn query_by_id(conn: &mut PgConnection, id: String) -> Result<pb::auth::auth::Client> {
    let id = id.parse::<Uuid>().ok_or_invalid()?;
    let client: pb::auth::auth::Client = schema::auth::clients::table.find(id).first(conn)?;
    Ok(client)
}
pub fn query_all(conn: &mut PgConnection) -> Result<Vec<pb::auth::auth::Client>> {
    let clients = schema::auth::clients::table.load(conn)?;
    Ok(clients)
}
pub fn insert(conn: &mut PgConnection) -> Result<pb::auth::auth::Client> {
    let client = diesel::insert_into(schema::auth::clients::table)
        .default_values()
        .get_result(conn)?;
    Ok(client)
}
