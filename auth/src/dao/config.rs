use diesel::{prelude::*, PgConnection};
use empty_utils::errors::Result;

use proto::{pb, schema};

pub fn query_all(conn: &mut PgConnection) -> Result<Vec<pb::auth::auth::Config>> {
    let configs = schema::auth::configs::table.load::<pb::auth::auth::Config>(conn)?;
    Ok(configs)
}
pub fn insert(
    conn: &mut PgConnection,
    config: pb::auth::auth::NewConfig,
) -> Result<pb::auth::auth::Config> {
    let config = diesel::insert_into(schema::auth::configs::table)
        .values(config)
        .get_result(conn)?;
    Ok(config)
}
