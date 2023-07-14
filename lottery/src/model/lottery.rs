use crate::{pb, utils::diesel::Paginate};
use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use uuid::Uuid;

use crate::schema::lotterys;

pub async fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::ListRequest,
) -> Result<(Vec<pb::lottery::Lottery>, Option<pb::paginate::Paginated>)> {
    let lotterys = lotterys::table
        .filter(lotterys::id.is_not_null())
        .paginate(request.paginate)
        .load_and_paginated(conn)?;
    Ok(lotterys)
}
pub async fn query_list_by_user_id(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<pb::lottery::Lottery>> {
    let lotterys = lotterys::table
        .filter(lotterys::user_id.eq(user_id))
        .get_results::<pb::lottery::Lottery>(conn)?;
    Ok(lotterys)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::lottery::Lottery> {
    let lottery = lotterys::table
        .find(id)
        .first::<pb::lottery::Lottery>(conn)?;
    Ok(lottery)
}
pub async fn insert(
    conn: &mut PgConnection,
    lottery: pb::lottery::NewLottery,
) -> Result<pb::lottery::Lottery> {
    let lottery = diesel::insert_into(lotterys::table)
        .values(lottery)
        .get_result::<pb::lottery::Lottery>(conn)?;
    Ok(lottery)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    lottery: pb::lottery::NewLottery,
) -> Result<pb::lottery::Lottery> {
    let lottery = diesel::update(lotterys::table)
        .filter(lotterys::dsl::id.eq(id))
        .set(lottery)
        .get_result::<pb::lottery::Lottery>(conn)?;
    Ok(lottery)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(lotterys::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
