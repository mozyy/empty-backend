use std::collections::HashMap;

use crate::{model, pb, service::record, utils::diesel::Paginate};
use diesel::prelude::*;
use empty_utils::errors::{Error, Result, ErrorConvert};
use uuid::Uuid;

use crate::schema;

fn query_records(
    conn: &mut PgConnection,
    records: Vec<pb::record::RecordInfo>,
) -> Result<Vec<pb::record::Record>> {
    let record_remarks = pb::record::RecordRemark::belonging_to(&records)
        .load::<pb::record::RecordRemark>(conn)?
        .grouped_by(&records);
    let lottery_ids = records.iter().map(|f| f.lottery_id).collect::<Vec<_>>();
    let lotterys = model::lottery::query_list_by_id(conn, lottery_ids)?.into_iter().map(|l| -> Result<_> {
        let lottery = l.lottery.clone().ok_or_loss()?;
        Ok((lottery.id, l))
    })
    .collect::<Result<HashMap<_,_>>>()?;

    let records = records
        .into_iter()
        .zip(record_remarks)
        .map(|(record, record_remarks)| {
            let lottery = lotterys.get(&record.lottery_id).map(|l|l.to_owned());
            pb::record::Record {
            record: Some(record),
            lottery,
            record_remarks,
        }
    })
        .collect();
    Ok(records)
}

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::record::ListRequest,
) -> Result<(Vec<pb::record::Record>, Option<pb::paginate::Paginated>)> {
    let mut filter = schema::records::table.into_boxed();
    if let Some(record) = request.record {
        if let Some(id) = record.id {
            filter = filter.filter(schema::records::id.eq(id));
        }
        if let Some(lottery_id) = record.lottery_id {
            filter = filter.filter(schema::records::lottery_id.eq(lottery_id));
        }
        if let Some(user_id) = record.user_id {
            filter = filter.filter(schema::records::user_id.eq(user_id.parse::<Uuid>().unwrap()));
        }
    }

    let (records, paginated) = filter
        .paginate(request.paginate)
        .load_and_paginated::<pb::record::RecordInfo>(conn)?;
    let records = query_records(conn, records)?;
    Ok((records, paginated))
}
pub fn query_list_by_record(
    conn: &mut PgConnection,
    record: Option<pb::record::RecordQuery>,
) -> Result<Vec<pb::record::Record>> {
    let mut filter = schema::records::table.into_boxed();
    if let Some(record) = record {
        if let Some(id) = record.id {
            filter = filter.filter(schema::records::id.eq(id));
        }
        if let Some(lottery_id) = record.lottery_id {
            filter = filter.filter(schema::records::lottery_id.eq(lottery_id));
        }
        if let Some(user_id) = record.user_id {
            filter = filter.filter(schema::records::user_id.eq(user_id.parse::<Uuid>().unwrap()));
        }
    }

    let records = filter.get_results::<pb::record::RecordInfo>(conn)?;
    let records = query_records(conn, records)?;
    Ok(records)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::record::Record> {
    let record = schema::records::table
        .find(id)
        .first::<pb::record::RecordInfo>(conn)?;
    let record_remarks =
        pb::record::RecordRemark::belonging_to(&record).load::<pb::record::RecordRemark>(conn)?;
        let lottery = model::lottery::query_by_id(conn, record.lottery_id)?;
    let records = pb::record::Record {
        record: Some(record),
        lottery: Some(lottery),
        record_remarks,
    };
    Ok(records)
}

fn insert_record_remarks(
    conn: &mut PgConnection,
    record: pb::record::RecordInfo,
    record_remarks: Vec<pb::record::NewRecordRemark>,
) -> Result<pb::record::Record> {
    let record_id = record.id;
    let record_remarks = record_remarks
        .into_iter()
        .map(|mut item| {
            item.record_id = record_id;
            item
        })
        .collect::<Vec<_>>();
    let record_remarks = diesel::insert_into(schema::record_remarks::table)
        .values(record_remarks)
        .get_results::<pb::record::RecordRemark>(conn)?;
    let lottery = model::lottery::query_by_id(conn, record.lottery_id)?;

    Ok(pb::record::Record {
        record: Some(record),
        lottery: Some(lottery),
        record_remarks,
    })
}

pub fn insert(
    conn: &mut PgConnection,
    record: pb::record::NewRecord,
) -> Result<pb::record::Record> {
    let pb::record::NewRecord {
        record,
        record_remarks,
    } = record;
    let record = diesel::insert_into(schema::records::table)
        .values(record)
        .get_result::<pb::record::RecordInfo>(conn)?;
    let record = insert_record_remarks(conn, record, record_remarks)?;
    Ok(record)
}
// TODO: patch
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    record: pb::record::NewRecord,
) -> Result<pb::record::Record> {
    let pb::record::NewRecord {
        record,
        record_remarks,
    } = record;
    diesel::delete(schema::record_remarks::table.filter(schema::record_remarks::record_id.eq(id)))
        .execute(conn)?;
    let record = diesel::update(schema::records::table)
        .filter(schema::records::dsl::id.eq(id))
        .set(record)
        .get_result::<pb::record::RecordInfo>(conn)?;
    let record = insert_record_remarks(conn, record, record_remarks)?;
    Ok(record)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    diesel::delete(schema::record_remarks::table.filter(schema::record_remarks::record_id.eq(id)))
        .execute(conn)?;
    let value = diesel::delete(schema::records::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
