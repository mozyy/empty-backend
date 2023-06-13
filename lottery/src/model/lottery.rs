use crate::pb::lottery as pb;
use diesel::prelude::*;
use empty_utils::errors::{ServiceError, ServiceResult};
use uuid::Uuid;

use crate::schema::lotterys;

pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<pb::Lottery>> {
    let lotterys = lotterys::table.load::<pb::Lottery>(conn)?;
    Ok(lotterys)
}
pub async fn query_list_by_user_id(conn: &mut PgConnection, user_id:Uuid) -> ServiceResult<Vec<pb::Lottery>> {
    let lotterys = lotterys::table.filter(lotterys::user_id.eq(user_id)).get_results::<pb::Lottery>(conn)?;
    Ok(lotterys)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult<pb::Lottery> {
    let lottery = lotterys::table.find(id).first::<pb::Lottery>(conn)?;
    Ok(lottery)
}
pub async fn insert(
    conn: &mut PgConnection,
    lottery: pb::NewLottery,
) -> ServiceResult<pb::Lottery> {
    let lottery = diesel::insert_into(lotterys::table)
        .values(lottery)
        .get_result::<pb::Lottery>(conn)?;
    Ok(lottery)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    lottery: pb::NewLottery,
) -> ServiceResult<pb::Lottery> {
    let lottery = diesel::update(lotterys::table)
        .filter(lotterys::dsl::id.eq(id))
        .set(lottery)
        .get_result::<pb::Lottery>(conn)?;
    Ok(lottery)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult {
    let value = diesel::delete(lotterys::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(ServiceError::String(String::from("delete error")));
    }
    Ok(())
}
