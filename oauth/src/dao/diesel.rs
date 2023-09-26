use diesel::{prelude::*, PgConnection};
use empty_utils::errors::Result;
use uuid::Uuid;

use proto::{pb, schema};

pub async fn user_query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::oauth::oauth::User> {
    let user = schema::oauth::users::table
        .find(id)
        .first::<pb::oauth::oauth::User>(conn)?;
    Ok(user)
}

pub async fn user_insert(conn: &mut PgConnection) -> Result<pb::oauth::oauth::User> {
    let user = diesel::insert_into(schema::oauth::users::table)
        .default_values()
        .get_result(conn)?;
    Ok(user)
}

pub async fn client_query_all(conn: &mut PgConnection) -> Result<Vec<pb::oauth::oauth::Client>> {
    let clients = schema::oauth::oauth_clients::table.load::<pb::oauth::oauth::Client>(conn)?;
    Ok(clients)
}
pub async fn client_query_by_name(
    conn: &mut PgConnection,
    name: String,
) -> Result<pb::oauth::oauth::Client> {
    let client = schema::oauth::oauth_clients::table
        .filter(schema::oauth::oauth_clients::name.eq(name))
        .first::<pb::oauth::oauth::Client>(conn)?;
    Ok(client)
}
pub async fn client_insert(
    conn: &mut PgConnection,
    client: pb::oauth::oauth::NewClient,
) -> Result<pb::oauth::oauth::Client> {
    let client = diesel::insert_into(schema::oauth::oauth_clients::table)
        .values(client)
        .get_result(conn)?;
    Ok(client)
}

pub async fn config_query_all(conn: &mut PgConnection) -> Result<Vec<pb::oauth::oauth::Config>> {
    let configs = schema::oauth::oauth_configs::table.load::<pb::oauth::oauth::Config>(conn)?;
    Ok(configs)
}
pub async fn config_insert(
    conn: &mut PgConnection,
    config: pb::oauth::oauth::NewConfig,
) -> Result<pb::oauth::oauth::Config> {
    let config = diesel::insert_into(schema::oauth::oauth_configs::table)
        .values(config)
        .get_result(conn)?;
    Ok(config)
}
