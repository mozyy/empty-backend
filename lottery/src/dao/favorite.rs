use std::collections::HashMap;

use diesel::prelude::*;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb;

use proto::{schema, utils::diesel::Paginate};
use uuid::Uuid;

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::favorite::ListRequest,
) -> Result<(
    Vec<pb::lottery::favorite::FavoriteWithLottery>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let user_id = request.user_id.parse::<Uuid>().ok_or_invalid()?;
    let (favorites, paginated) = schema::lottery::favorites::table
        .filter(schema::lottery::favorites::user_id.eq(user_id))
        .paginate(request.paginate)
        .load_and_paginated::<pb::lottery::favorite::Favorite>(conn)?;
    let fovorite_ids = favorites.iter().map(|f| f.lottery_id).collect::<Vec<_>>();
    let lotterys: Vec<pb::lottery::lottery::Lottery> =
        super::lottery::query_list_by_id(conn, fovorite_ids)?;
    let lotterys = lotterys
        .into_iter()
        .map(|l| Ok((l.lottery.clone().ok_or_loss()?.id, l)))
        .collect::<Result<HashMap<i32, pb::lottery::lottery::Lottery>>>()?;
    let favorites = favorites
        .into_iter()
        .map(|f| {
            let lottery = lotterys.get(&f.id.clone()).cloned();
            pb::lottery::favorite::FavoriteWithLottery {
                favorite: Some(f),
                lottery,
            }
        })
        .collect();
    Ok((favorites, paginated))
}

pub fn query_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<pb::lottery::favorite::FavoriteWithLottery> {
    let favorite = schema::lottery::favorites::table
        .find(id)
        .first::<pb::lottery::favorite::Favorite>(conn)?;
    let lottery = super::lottery::query_by_id(conn, favorite.lottery_id)?;
    Ok(pb::lottery::favorite::FavoriteWithLottery {
        favorite: Some(favorite),
        lottery: Some(lottery),
    })
}

pub fn insert(
    conn: &mut PgConnection,
    favorite: pb::lottery::favorite::NewFavorite,
) -> Result<pb::lottery::favorite::FavoriteWithLottery> {
    let lottery_id = favorite.lottery_id;
    let favorite = diesel::insert_into(schema::lottery::favorites::table)
        .values(favorite)
        .get_result::<pb::lottery::favorite::Favorite>(conn)?;
    let lottery = super::lottery::query_by_id(conn, lottery_id)?;
    Ok(pb::lottery::favorite::FavoriteWithLottery {
        favorite: Some(favorite),
        lottery: Some(lottery),
    })
}

pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    favorite: pb::lottery::favorite::NewFavorite,
) -> Result<pb::lottery::favorite::FavoriteWithLottery> {
    let favorite = diesel::update(schema::lottery::favorites::table.find(id))
        .set(favorite)
        .get_result::<pb::lottery::favorite::Favorite>(conn)?;
    let lottery = super::lottery::query_by_id(conn, favorite.lottery_id)?;
    Ok(pb::lottery::favorite::FavoriteWithLottery {
        favorite: Some(favorite),
        lottery: Some(lottery),
    })
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::lottery::favorites::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
