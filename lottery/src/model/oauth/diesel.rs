use diesel::{prelude::*, PgConnection};
use empty_utils::errors::Result;
use uuid::Uuid;

use crate::{
    pb,
    schema::{oauth_clients, oauth_configs, users},
};

pub async fn user_query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::oauth::User> {
    let user = users::table.find(id).first::<pb::oauth::User>(conn)?;
    Ok(user)
}

pub async fn user_insert(conn: &mut PgConnection) -> Result<pb::oauth::User> {
    let user = diesel::insert_into(users::table)
        .default_values()
        .get_result(conn)?;
    Ok(user)
}

pub async fn client_query_all(conn: &mut PgConnection) -> Result<Vec<pb::oauth::Client>> {
    let clients = oauth_clients::table.load::<pb::oauth::Client>(conn)?;
    Ok(clients)
}
pub async fn client_query_by_name(
    conn: &mut PgConnection,
    name: String,
) -> Result<pb::oauth::Client> {
    let client = oauth_clients::table
        .filter(oauth_clients::name.eq(name))
        .first::<pb::oauth::Client>(conn)?;
    Ok(client)
}
pub async fn client_insert(
    conn: &mut PgConnection,
    client: pb::oauth::NewClient,
) -> Result<pb::oauth::Client> {
    let client = diesel::insert_into(oauth_clients::table)
        .values(client)
        .get_result(conn)?;
    Ok(client)
}

pub async fn config_query_all(conn: &mut PgConnection) -> Result<Vec<pb::oauth::Config>> {
    let configs = oauth_configs::table.load::<pb::oauth::Config>(conn)?;
    Ok(configs)
}
pub async fn config_insert(
    conn: &mut PgConnection,
    config: pb::oauth::NewConfig,
) -> Result<pb::oauth::Config> {
    let config = diesel::insert_into(oauth_configs::table)
        .values(config)
        .get_result(conn)?;
    Ok(config)
}
