use diesel::{prelude::*, PgConnection};
use empty_utils::errors::ServiceResult;
use uuid::Uuid;

use crate::{pb, schema::oauth_users};

pub async fn query_list_by_id(conn: &mut PgConnection, id: Uuid) -> ServiceResult<pb::oauth::User> {
    let user = oauth_users::table.find(id).first::<pb::oauth::User>(conn)?;
    Ok(user)
}

pub async fn insert(conn: &mut PgConnection) -> ServiceResult<pb::oauth::User> {
    let user = diesel::insert_into(oauth_users::table)
        .default_values()
        .get_result(conn)?;
    Ok(user)
}
