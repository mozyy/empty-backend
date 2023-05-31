use crate::pb;
use diesel::prelude::*;
use empty_utils::{
    errors::{ServiceError, ServiceResult},
};

use crate::schema::{items, lotterys, remarks};

pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<pb::Lottery>> {
    let lotterys = lotterys::table.load::<pb::LotteryInfo>(conn)?;

    let items: Vec<Vec<pb::Item>> = pb::Item::belonging_to(&lotterys)
        .load::<pb::Item>(conn)?
        .grouped_by(&lotterys);
    let remarks: Vec<Vec<pb::Remark>> = pb::Remark::belonging_to(&lotterys)
        .load(conn)?
        .grouped_by(&lotterys);

    let lotterys = lotterys.into_iter().zip(items).zip(remarks).map(|((lottery, items), remarks)|pb::Lottery{
            lottery:Some(lottery),
            items,
            remarks,
    }).collect();
    Ok(lotterys)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult<pb::Lottery> {
    let lottery = lotterys::table.find(id).first::<pb::LotteryInfo>(conn)?;
    let items = pb::Item::belonging_to(&lottery).load::<pb::Item>(conn)?;
    let remarks = pb::Remark::belonging_to(&lottery).load::<pb::Remark>(conn)?;
    let lottery = pb::Lottery{lottery:Some(lottery),items, remarks};
    Ok(lottery)
}
pub async fn insert(
    conn: &mut PgConnection,
    value: pb::NewLottery,
) -> ServiceResult<pb::Lottery> {
    let lottery = diesel::insert_into(lotterys::table)
        .values(value.lottery)
        .get_result::<pb::LotteryInfo>(conn)?;
    let items = diesel::insert_into(items::table)
    .values(value.items.into_iter().map(|i|(
        items::name.eq(i.name),
        items::value.eq(i.value),
        items::lottery_id.eq(lottery.id)
    )).collect::<Vec<_>>()).get_results::<pb::Item>(conn)?;
    let remarks = diesel::insert_into(remarks::table)
    .values(value.remarks.into_iter().map(|i|(
        remarks::name.eq(i.name),
        remarks::require.eq(i.require),
        remarks::lottery_id.eq(lottery.id)
    )).collect::<Vec<_>>()).get_results::<pb::Remark>(conn)?;
    let lottery = pb::Lottery{lottery:Some(lottery), items,remarks};
    Ok(lottery)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    value: pb::NewLottery,
) -> ServiceResult<pb::Lottery> {
    let lottery = diesel::update(lotterys::table)
        .filter(lotterys::dsl::id.eq(id))
        .set(value.lottery)
        .get_result::<pb::LotteryInfo>(conn)?;
    diesel::delete(items::table.filter(items::lottery_id.eq(id))).execute(conn)?;
    let items = diesel::insert_into(items::table)
    .values(value.items.into_iter().map(|i|(
        items::name.eq(i.name),
        items::value.eq(i.value),
        items::lottery_id.eq(id)
    )).collect::<Vec<_>>()).get_results::<pb::Item>(conn)?;
    diesel::delete(remarks::table.filter(remarks::lottery_id.eq(id))).execute(conn)?;
    let remarks = diesel::insert_into(remarks::table)
    .values(value.remarks.into_iter().map(|i|(
        remarks::name.eq(i.name),
        remarks::require.eq(i.require),
        remarks::lottery_id.eq(id)
    )).collect::<Vec<_>>()).get_results::<pb::Remark>(conn)?;
    let lottery = pb::Lottery{lottery:Some(lottery), items,remarks};
    Ok(lottery)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult {
    diesel::delete(items::table.filter(items::lottery_id.eq(id))).execute(conn)?;
    diesel::delete(remarks::table.filter(remarks::lottery_id.eq(id))).execute(conn)?;
    let value = diesel::delete(lotterys::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(ServiceError::String(String::from("delete error")));
    }
    Ok(())
}
