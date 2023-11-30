use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::pb;

use proto::{schema, utils::diesel::Paginate};

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::oss::oss::ListRequest,
) -> Result<(
    Vec<pb::oss::oss::Oss>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let resp = schema::oss::oss::table
        .filter(schema::oss::oss::id.eq_any(request.ids))
        .paginate(request.paginate)
        .load_and_paginated::<pb::oss::oss::Oss>(conn)?;
    Ok(resp)
}

pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::oss::oss::Oss> {
    let oss = schema::oss::oss::table
        .find(id)
        .first::<pb::oss::oss::Oss>(conn)?;
    log::info!("oss: {:?}", oss);
    Ok(oss)
}

pub fn insert(conn: &mut PgConnection, oss: pb::oss::oss::NewOss) -> Result<pb::oss::oss::Oss> {
    let oss = diesel::insert_into(schema::oss::oss::table)
        .values(oss)
        .get_result::<pb::oss::oss::Oss>(conn)?;
    Ok(oss)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    oss: pb::oss::oss::NewOss,
) -> Result<pb::oss::oss::Oss> {
    let oss = diesel::update(schema::oss::oss::table.find(id))
        .set(oss)
        .get_result::<pb::oss::oss::Oss>(conn)?;
    Ok(oss)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::oss::oss::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
