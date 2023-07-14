use diesel::{prelude::*, PgConnection};
use empty_utils::errors::Result;
use uuid::Uuid;

use crate::{
    pb::oauth as pb,
    schema::{oauth_clients, oauth_configs, users},
};

pub async fn user_query_by_id(conn: &mut PgConnection, id: Uuid) -> Result<pb::User> {
    let user = users::table.find(id).first::<pb::User>(conn)?;
    Ok(user)
}

pub async fn user_insert(conn: &mut PgConnection) -> Result<pb::User> {
    let user = diesel::insert_into(users::table)
        .default_values()
        .get_result(conn)?;
    Ok(user)
}

pub async fn client_query_all(conn: &mut PgConnection) -> Result<Vec<pb::Client>> {
    let clients = oauth_clients::table.load::<pb::Client>(conn)?;
    Ok(clients)
}
pub async fn client_query_by_name(conn: &mut PgConnection, name: String) -> Result<pb::Client> {
    let client = oauth_clients::table
        .filter(oauth_clients::name.eq(name))
        .first::<pb::Client>(conn)?;
    Ok(client)
}
pub async fn client_insert(conn: &mut PgConnection, client: pb::NewClient) -> Result<pb::Client> {
    let client = diesel::insert_into(oauth_clients::table)
        .values(client)
        .get_result(conn)?;
    Ok(client)
}

pub async fn config_query_all(conn: &mut PgConnection) -> Result<Vec<pb::Config>> {
    let configs = oauth_configs::table.load::<pb::Config>(conn)?;
    Ok(configs)
}
pub async fn config_insert(conn: &mut PgConnection, config: pb::NewConfig) -> Result<pb::Config> {
    let config = diesel::insert_into(oauth_configs::table)
        .values(config)
        .get_result(conn)?;
    Ok(config)
}
