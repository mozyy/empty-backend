use std::collections::HashMap;

use diesel::prelude::*;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb::{
    self,
    lottery::{favorite::Favorite, lottery::LotteryInfo},
};

use uuid::Uuid;

use proto::{schema, utils::diesel::Paginate};

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::favorite::ListRequest,
) -> Result<pb::lottery::favorite::ListResponse> {
    let (fovorites, paginated) = schema::lottery::favorites::table
        .into_boxed()
        .paginate(request.paginate)
        .load_and_paginated::<Favorite>(conn)?;
    let fovorite_ids = fovorites.iter().map(|f| f.id).collect::<Vec<_>>();
    let lotterys: Vec<pb::lottery::lottery::Lottery> =
        super::lottery::query_list_by_id(conn, fovorite_ids)?;
    let lotterys = lotterys
        .into_iter()
        .map(|l| Ok((l.lottery.clone().ok_or_loss()?.id, l)))
        .collect::<Result<HashMap<i32, pb::lottery::lottery::Lottery>>>()?;
    let favorites = fovorites
        .into_iter()
        .map(|f| {
            let lottery = lotterys.get(&f.id.clone()).cloned();
            pb::lottery::favorite::FavoriteWithLottery {
                favorite: Some(f),
                lottery,
            }
        })
        .collect();
    Ok(pb::lottery::favorite::ListResponse {
        favorites,
        paginated,
    })
}
