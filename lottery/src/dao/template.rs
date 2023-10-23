use std::collections::HashMap;

use diesel::prelude::*;
use empty_utils::errors::{Error, ErrorConvert, Result};
use proto::pb;

use uuid::Uuid;

use proto::{schema, utils::diesel::Paginate};

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::template::ListRequest,
) -> Result<(
    Vec<pb::lottery::template::TemplateWithLottery>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let (templates, paginated) = schema::lottery::templates::table
        .order(schema::lottery::templates::id.desc())
        .paginate(request.paginate)
        .load_and_paginated::<pb::lottery::template::Template>(conn)?;
    let fovorite_ids = templates.iter().map(|f| f.lottery_id).collect::<Vec<_>>();
    let lotterys: Vec<pb::lottery::lottery::Lottery> =
        super::lottery::query_list_by_id(conn, fovorite_ids)?;
    let lotterys = lotterys
        .into_iter()
        .map(|l| Ok((l.lottery.clone().ok_or_loss()?.id, l)))
        .collect::<Result<HashMap<i32, pb::lottery::lottery::Lottery>>>()?;
    let templates = templates
        .into_iter()
        .map(|f| {
            let lottery = lotterys.get(&f.lottery_id.clone()).cloned();
            pb::lottery::template::TemplateWithLottery {
                template: Some(f),
                lottery,
            }
        })
        .collect();
    Ok((templates, paginated))
}

pub fn query_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<pb::lottery::template::TemplateWithLottery> {
    let template = schema::lottery::templates::table
        .find(id)
        .first::<pb::lottery::template::Template>(conn)?;
    let lottery = super::lottery::query_by_id(conn, template.lottery_id)?;
    Ok(pb::lottery::template::TemplateWithLottery {
        template: Some(template),
        lottery: Some(lottery),
    })
}

pub fn query_by_lottery_id(
    conn: &mut PgConnection,
    lottery_id: i32,
) -> Result<pb::lottery::template::Template> {
    let template = schema::lottery::templates::table
        .filter(schema::lottery::templates::lottery_id.eq(lottery_id))
        .first::<pb::lottery::template::Template>(conn)?;
    Ok(template)
}

pub fn insert(
    conn: &mut PgConnection,
    template: pb::lottery::template::NewTemplate,
) -> Result<pb::lottery::template::TemplateWithLottery> {
    let lottery_id = template.lottery_id;
    let template = diesel::insert_into(schema::lottery::templates::table)
        .values(template)
        .get_result::<pb::lottery::template::Template>(conn)?;
    let lottery = super::lottery::query_by_id(conn, lottery_id)?;
    Ok(pb::lottery::template::TemplateWithLottery {
        template: Some(template),
        lottery: Some(lottery),
    })
}

pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    template: pb::lottery::template::NewTemplate,
) -> Result<pb::lottery::template::TemplateWithLottery> {
    let template = diesel::update(schema::lottery::templates::table.find(id))
        .set(template)
        .get_result::<pb::lottery::template::Template>(conn)?;
    let lottery = super::lottery::query_by_id(conn, template.lottery_id)?;
    Ok(pb::lottery::template::TemplateWithLottery {
        template: Some(template),
        lottery: Some(lottery),
    })
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::lottery::templates::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
