use crate::{pb, utils::diesel::Paginate};
use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use uuid::Uuid;

use crate::schema::records;

pub async fn query_list(
    conn: &mut PgConnection,
    request: pb::record::ListRequest,
) -> Result<(Vec<pb::record::Record>, Option<pb::paginate::Paginated>)> {
    let records = records::table
        .filter(records::id.is_not_null())
        .paginate(request.paginate)
        .load_and_paginated(conn)?;
    Ok(records)
}
pub async fn query_list_by_user_id(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<pb::record::Record>> {
    let records = records::table
        .filter(records::user_id.eq(user_id))
        .get_results(conn)?;
    Ok(records)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::record::Record> {
    let record = records::table.find(id).first(conn)?;
    Ok(record)
}
pub async fn insert(
    conn: &mut PgConnection,
    record: pb::record::NewRecord,
) -> Result<pb::record::Record> {
    let record = diesel::insert_into(records::table)
        .values(record)
        .get_result::<pb::record::Record>(conn)?;
    Ok(record)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    record: pb::record::NewRecord,
) -> Result<pb::record::Record> {
    let record = diesel::update(records::table)
        .filter(records::dsl::id.eq(id))
        .set(record)
        .get_result(conn)?;
    Ok(record)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(records::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
