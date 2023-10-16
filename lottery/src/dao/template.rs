use diesel::prelude::*;
use empty_utils::errors::{Error, Result};
use proto::pb;

use uuid::Uuid;

use proto::{schema, utils::diesel::Paginate};

fn query_lottery(
    conn: &mut PgConnection,
    lotterys: Vec<pb::lottery::lottery::LotteryInfo>,
) -> Result<Vec<pb::lottery::lottery::Lottery>> {
    let items = pb::lottery::lottery::Item::belonging_to(&lotterys)
        .load::<pb::lottery::lottery::Item>(conn)?
        .grouped_by(&lotterys);
    let remarks = pb::lottery::lottery::Remark::belonging_to(&lotterys)
        .load::<pb::lottery::lottery::Remark>(conn)?
        .grouped_by(&lotterys);
    let lotterys = lotterys
        .into_iter()
        .zip(items)
        .zip(remarks)
        .map(
            |((lottery, items), remarks)| pb::lottery::lottery::Lottery {
                lottery: Some(lottery),
                items,
                remarks,
            },
        )
        .collect();
    Ok(lotterys)
}

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::lottery::ListRequest,
) -> Result<(
    Vec<pb::lottery::lottery::Lottery>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let mut filter = schema::lottery::lotterys::table.into_boxed();
    if let Some(lottery) = request.lottery {
        if !lottery.ids.is_empty() {
            filter = filter.filter(schema::lottery::lotterys::id.eq_any(lottery.ids));
        }
        if let Some(user_id) = lottery.user_id {
            filter = filter
                .filter(schema::lottery::lotterys::user_id.eq(user_id.parse::<Uuid>().unwrap()));
        }
    }

    let (lotterys, paginated) = filter
        .paginate(request.paginate)
        .load_and_paginated::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok((lotterys, paginated))
}
pub fn query_list_by_user_id(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<pb::lottery::lottery::Lottery>> {
    let lotterys = schema::lottery::lotterys::table
        .filter(schema::lottery::lotterys::user_id.eq(user_id))
        .get_results::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok(lotterys)
}

pub fn query_list_by_id(
    conn: &mut PgConnection,
    ids: Vec<i32>,
) -> Result<Vec<pb::lottery::lottery::Lottery>> {
    let lotterys = schema::lottery::lotterys::table
        .filter(schema::lottery::lotterys::id.eq_any(ids))
        .get_results::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let lotterys = query_lottery(conn, lotterys)?;
    Ok(lotterys)
}

pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::lottery::lottery::Lottery> {
    let lottery = schema::lottery::lotterys::table
        .find(id)
        .first::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let items = pb::lottery::lottery::Item::belonging_to(&lottery)
        .load::<pb::lottery::lottery::Item>(conn)?;
    let remarks = pb::lottery::lottery::Remark::belonging_to(&lottery)
        .load::<pb::lottery::lottery::Remark>(conn)?;
    let lotterys = pb::lottery::lottery::Lottery {
        lottery: Some(lottery),
        items,
        remarks,
    };
    Ok(lotterys)
}

fn insert_items_remarks(
    conn: &mut PgConnection,
    lottery: pb::lottery::lottery::LotteryInfo,
    items: Vec<pb::lottery::lottery::NewItem>,
    remarks: Vec<pb::lottery::lottery::NewRemark>,
) -> Result<pb::lottery::lottery::Lottery> {
    let lottery_id = lottery.id;
    let items = items
        .into_iter()
        .map(|mut item| {
            item.lottery_id = lottery_id;
            item
        })
        .collect::<Vec<_>>();
    let items = diesel::insert_into(schema::lottery::items::table)
        .values(items)
        .get_results::<pb::lottery::lottery::Item>(conn)?;
    let remarks = remarks
        .into_iter()
        .map(|mut remark| {
            remark.lottery_id = lottery_id;
            remark
        })
        .collect::<Vec<_>>();
    let remarks = diesel::insert_into(schema::lottery::remarks::table)
        .values(remarks)
        .get_results::<pb::lottery::lottery::Remark>(conn)?;
    Ok(pb::lottery::lottery::Lottery {
        lottery: Some(lottery),
        items,
        remarks,
    })
}
pub fn insert(
    conn: &mut PgConnection,
    lottery: pb::lottery::lottery::NewLottery,
) -> Result<pb::lottery::lottery::Lottery> {
    let pb::lottery::lottery::NewLottery {
        lottery,
        items,
        remarks,
    } = lottery;
    let lottery = diesel::insert_into(schema::lottery::lotterys::table)
        .values(lottery)
        .get_result::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let lottery = insert_items_remarks(conn, lottery, items, remarks)?;
    Ok(lottery)
}
// TODO: patch
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    lottery: pb::lottery::lottery::NewLottery,
) -> Result<pb::lottery::lottery::Lottery> {
    diesel::delete(schema::lottery::items::table.filter(schema::lottery::items::lottery_id.eq(id)))
        .execute(conn)?;
    diesel::delete(
        schema::lottery::remarks::table.filter(schema::lottery::remarks::lottery_id.eq(id)),
    )
    .execute(conn)?;
    let pb::lottery::lottery::NewLottery {
        lottery,
        items,
        remarks,
    } = lottery;
    let lottery = diesel::update(schema::lottery::lotterys::table)
        .filter(schema::lottery::lotterys::dsl::id.eq(id))
        .set(lottery)
        .get_result::<pb::lottery::lottery::LotteryInfo>(conn)?;
    let lottery = insert_items_remarks(conn, lottery, items, remarks)?;
    Ok(lottery)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    diesel::delete(schema::lottery::items::table.filter(schema::lottery::items::lottery_id.eq(id)))
        .execute(conn)?;
    diesel::delete(
        schema::lottery::remarks::table.filter(schema::lottery::remarks::lottery_id.eq(id)),
    )
    .execute(conn)?;
    let value = diesel::delete(schema::lottery::lotterys::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
