use crate::{pb, utils::diesel::Paginate};
use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use uuid::Uuid;

use crate::schema;

fn query_lottery(
    conn: &mut PgConnection,
    lotterys: Vec<pb::lottery::LotteryInfo>,
) -> Result<Vec<pb::lottery::Lottery>> {
    let items = pb::lottery::Item::belonging_to(&lotterys)
        .load::<pb::lottery::Item>(conn)?
        .grouped_by(&lotterys);
    let remarks = pb::lottery::Remark::belonging_to(&lotterys)
        .load::<pb::lottery::Remark>(conn)?
        .grouped_by(&lotterys);
    let lotterys = lotterys
        .into_iter()
        .zip(items)
        .zip(remarks)
        .map(|((lottery, items), remarks)| pb::lottery::Lottery {
            lottery: Some(lottery),
            items,
            remarks,
        })
        .collect();
    Ok(lotterys)
}

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::ListRequest,
) -> Result<(Vec<pb::lottery::Lottery>, Option<pb::paginate::Paginated>)> {
    let (lotterys, paginated) = schema::lotterys::table
        .filter(schema::lotterys::id.is_not_null())
        .paginate(request.paginate)
        .load_and_paginated::<pb::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok((lotterys, paginated))
}
pub fn query_list_by_user_id(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<pb::lottery::Lottery>> {
    let lotterys = schema::lotterys::table
        .filter(schema::lotterys::user_id.eq(user_id))
        .get_results::<pb::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok(lotterys)
}

pub fn query_list_by_id(
    conn: &mut PgConnection,
    ids: Vec<i32>,
) -> Result<Vec<pb::lottery::Lottery>> {
    let lotterys = schema::lotterys::table
        .filter(schema::lotterys::id.eq_any(ids))
        .get_results::<pb::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok(lotterys)
}

pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::lottery::Lottery> {
    let lottery = schema::lotterys::table
        .find(id)
        .first::<pb::lottery::LotteryInfo>(conn)?;
    let items = pb::lottery::Item::belonging_to(&lottery).load::<pb::lottery::Item>(conn)?;
    let remarks = pb::lottery::Remark::belonging_to(&lottery).load::<pb::lottery::Remark>(conn)?;
    let lotterys = pb::lottery::Lottery {
        lottery: Some(lottery),
        items,
        remarks,
    };
    Ok(lotterys)
}

fn insert_items_remarks(
    conn: &mut PgConnection,
    lottery: pb::lottery::LotteryInfo,
    items: Vec<pb::lottery::NewItem>,
    remarks: Vec<pb::lottery::NewRemark>,
) -> Result<pb::lottery::Lottery> {
    let lottery_id = lottery.id;
    let items = items
        .into_iter()
        .map(|mut item| {
            item.lottery_id = lottery_id;
            item
        })
        .collect::<Vec<_>>();
    let items = diesel::insert_into(schema::items::table)
        .values(items)
        .get_results::<pb::lottery::Item>(conn)?;
    let remarks = remarks
        .into_iter()
        .map(|mut remark| {
            remark.lottery_id = lottery_id;
            remark
        })
        .collect::<Vec<_>>();
    let remarks = diesel::insert_into(schema::remarks::table)
        .values(remarks)
        .get_results::<pb::lottery::Remark>(conn)?;
    Ok(pb::lottery::Lottery {
        lottery: Some(lottery),
        items,
        remarks,
    })
}
pub fn insert(
    conn: &mut PgConnection,
    lottery: pb::lottery::NewLottery,
) -> Result<pb::lottery::Lottery> {
    let pb::lottery::NewLottery {
        lottery,
        items,
        remarks,
    } = lottery;
    let lottery = diesel::insert_into(schema::lotterys::table)
        .values(lottery)
        .get_result::<pb::lottery::LotteryInfo>(conn)?;
    let lottery = insert_items_remarks(conn, lottery, items, remarks)?;
    Ok(lottery)
}
// TODO: patch
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    lottery: pb::lottery::NewLottery,
) -> Result<pb::lottery::Lottery> {
    diesel::delete(schema::items::table.filter(schema::items::lottery_id.eq(id))).execute(conn)?;
    diesel::delete(schema::remarks::table.filter(schema::remarks::lottery_id.eq(id)))
        .execute(conn)?;
    let pb::lottery::NewLottery {
        lottery,
        items,
        remarks,
    } = lottery;
    let lottery = diesel::update(schema::lotterys::table)
        .filter(schema::lotterys::dsl::id.eq(id))
        .set(lottery)
        .get_result::<pb::lottery::LotteryInfo>(conn)?;
    let lottery = insert_items_remarks(conn, lottery, items, remarks)?;
    Ok(lottery)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    diesel::delete(schema::items::table.filter(schema::items::lottery_id.eq(id))).execute(conn)?;
    diesel::delete(schema::remarks::table.filter(schema::remarks::lottery_id.eq(id)))
        .execute(conn)?;
    let value = diesel::delete(schema::lotterys::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
