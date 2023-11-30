use std::collections::HashMap;

use crate::dao;
use diesel::prelude::*;
use empty_utils::errors::{Error, ErrorConvert, Result};
use uuid::Uuid;

use proto::{pb, schema, utils::diesel::Paginate};

fn query_records(
    conn: &mut PgConnection,
    records: Vec<pb::lottery::record::RecordInfo>,
) -> Result<Vec<pb::lottery::record::Record>> {
    let record_remarks = pb::lottery::record::RecordRemark::belonging_to(&records)
        .load::<pb::lottery::record::RecordRemark>(conn)?
        .grouped_by(&records);
    let lottery_ids = records.iter().map(|f| f.lottery_id).collect::<Vec<_>>();
    let _lotterys = dao::lottery::query_list_by_id(conn, lottery_ids)?
        .into_iter()
        .map(|l| -> Result<_> {
            let lottery = l.lottery.clone().ok_or_loss()?;
            Ok((lottery.id, l))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let records = records
        .into_iter()
        .zip(record_remarks)
        .map(|(record, record_remarks)| pb::lottery::record::Record {
            record: Some(record),
            record_remarks,
        })
        .collect();
    Ok(records)
}

pub fn query_list(
    conn: &mut PgConnection,
    request: pb::lottery::record::ListRequest,
) -> Result<(
    Vec<pb::lottery::record::Record>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let mut filter = schema::lottery::records::table.into_boxed();
    if let Some(record) = request.record {
        if let Some(id) = record.id {
            filter = filter.filter(schema::lottery::records::id.eq(id));
        }
        if let Some(lottery_id) = record.lottery_id {
            filter = filter.filter(schema::lottery::records::lottery_id.eq(lottery_id));
        }
        if let Some(user_id) = record.user_id {
            filter = filter
                .filter(schema::lottery::records::user_id.eq(user_id.parse::<Uuid>().unwrap()));
        }
    }

    let (records, paginated) = filter
        .order(schema::lottery::records::id.desc())
        .paginate(request.paginate)
        .load_and_paginated::<pb::lottery::record::RecordInfo>(conn)?;
    let records = query_records(conn, records)?;
    Ok((records, paginated))
}
pub fn query_list_by_record(
    conn: &mut PgConnection,
    record: Option<pb::lottery::record::RecordQuery>,
) -> Result<Vec<pb::lottery::record::Record>> {
    let mut filter = schema::lottery::records::table.into_boxed();
    if let Some(record) = record {
        if let Some(id) = record.id {
            filter = filter.filter(schema::lottery::records::id.eq(id));
        }
        if let Some(lottery_id) = record.lottery_id {
            filter = filter.filter(schema::lottery::records::lottery_id.eq(lottery_id));
        }
        if let Some(user_id) = record.user_id {
            filter = filter
                .filter(schema::lottery::records::user_id.eq(user_id.parse::<Uuid>().unwrap()));
        }
    }

    let records = filter
        .order(schema::lottery::records::id.desc())
        .get_results::<pb::lottery::record::RecordInfo>(conn)?;
    let records = query_records(conn, records)?;
    Ok(records)
}
pub fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::lottery::record::Record> {
    let record = schema::lottery::records::table
        .find(id)
        .first::<pb::lottery::record::RecordInfo>(conn)?;
    let record_remarks = pb::lottery::record::RecordRemark::belonging_to(&record)
        .load::<pb::lottery::record::RecordRemark>(conn)?;
    let records = pb::lottery::record::Record {
        record: Some(record),
        record_remarks,
    };
    Ok(records)
}

fn insert_record_remarks(
    conn: &mut PgConnection,
    record: pb::lottery::record::RecordInfo,
    record_remarks: Vec<pb::lottery::record::NewRecordRemark>,
) -> Result<pb::lottery::record::Record> {
    let record_id = record.id;
    let record_remarks = record_remarks
        .into_iter()
        .map(|mut item| {
            item.record_id = record_id;
            item
        })
        .collect::<Vec<_>>();
    let record_remarks = diesel::insert_into(schema::lottery::record_remarks::table)
        .values(record_remarks)
        .get_results::<pb::lottery::record::RecordRemark>(conn)?;

    Ok(pb::lottery::record::Record {
        record: Some(record),
        record_remarks,
    })
}

pub fn insert(
    conn: &mut PgConnection,
    record: pb::lottery::record::NewRecord,
) -> Result<pb::lottery::record::Record> {
    let pb::lottery::record::NewRecord {
        record,
        record_remarks,
    } = record;
    let record = diesel::insert_into(schema::lottery::records::table)
        .values(record)
        .get_result::<pb::lottery::record::RecordInfo>(conn)?;
    let record = insert_record_remarks(conn, record, record_remarks)?;
    Ok(record)
}
pub fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    record: pb::lottery::record::NewRecord,
) -> Result<pb::lottery::record::Record> {
    let pb::lottery::record::NewRecord {
        record,
        record_remarks,
    } = record;
    diesel::delete(
        schema::lottery::record_remarks::table
            .filter(schema::lottery::record_remarks::record_id.eq(id)),
    )
    .execute(conn)?;
    let record = diesel::update(schema::lottery::records::table.find(id))
        .set(record)
        .get_result::<pb::lottery::record::RecordInfo>(conn)?;
    let record = insert_record_remarks(conn, record, record_remarks)?;
    Ok(record)
}
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    diesel::delete(
        schema::lottery::record_remarks::table
            .filter(schema::lottery::record_remarks::record_id.eq(id)),
    )
    .execute(conn)?;
    let value = diesel::delete(schema::lottery::records::table.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
