use std::borrow::BorrowMut;

use async_trait::async_trait;
use diesel::GroupedBy;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert, Result},
    tonic::Resp,
};
use rand::Rng;
use tonic::Response;
use tracing::info;

use crate::{
    configs::ADDR_CLIENT,
    model::{self, oauth::UserId},
    pb,
};

pub struct Service {
    pub db: db::DbPool,
}
impl Service {
    pub fn new_by_db(db: db::DbPool) -> Self {
        Self { db }
    }
}


#[async_trait]
impl pb::record::record_service_server::RecordService for Service {
    async fn list(
        &self,
        request: tonic::Request<pb::record::ListRequest>,
    ) -> Resp<pb::record::ListResponse> {
        let user_id = UserId::try_from(&request)?.to_string();
        let mut conn = self.db.get_conn()?;
        let mut request = request.into_inner();
        match &mut request.record {
            Some(record) => {
                record.user_id = Some(user_id);
            },
            None => {
                request.record = Some(pb::record::RecordQuery{user_id:Some(user_id), ..Default::default()});
            },
        };
        let (records, paginated) = model::record::query_list(&mut conn, request)?;
        Ok(Response::new(pb::record::ListResponse {
            records,
            paginated,
        }))
    }
    async fn get(
        &self,
        request: tonic::Request<pb::record::GetRequest>,
    ) -> Resp<pb::record::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let record = model::record::query_by_id(&mut conn, request.into_inner().id)?;
        Ok(Response::new(pb::record::GetResponse {
            record: Some(record),
        }))
    }
    async fn create(
        &self,
        request: tonic::Request<pb::record::CreateRequest>,
    ) -> Resp<pb::record::CreateResponse> {
        let user_id = UserId::try_from(&request)?.to_string();
        let mut new_record = request.into_inner().record.ok_or_invalid()?;
        let mut record = new_record.record.as_mut().ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let my_records = model::record::query_list_by_record(
            &mut conn,
            Some(pb::record::RecordQuery {
                id: None,
                user_id: Some(user_id.clone()),
                lottery_id: Some(record.lottery_id),
            }),
        )?;
        if !my_records.is_empty() {
            return Err(Error::unknown("already records"))?;
        }
        let lottery = model::lottery::query_by_id(&mut conn, record.lottery_id)?;

        let pb::lottery::Lottery {
            lottery, mut items, ..
        } = lottery;
        let lottery = lottery.ok_or_loss()?;

        let r#type = pb::lottery::Type::from_i32(lottery.r#type).ok_or_invalid()?;
        let all_records = model::record::query_list_by_record(
            &mut conn,
            Some(pb::record::RecordQuery {
                id: None,
                user_id: None,
                lottery_id: Some(record.lottery_id),
            }),
        )?
        .into_iter()
        .map(|record| -> Result<_> {
            let record = record.record.ok_or_loss()?;
            Ok(record)
        })
        .collect::<Result<Vec<_>>>()?;
        let all_records_amount = all_records.len() as i32;
        let grouped_records = all_records.grouped_by(&items);
        let mut rng = rand::thread_rng();
        let mut total_amount: i32 = items.clone().into_iter().map(|item| item.value).sum();
        if let pb::lottery::Type::Number = r#type {
            let remaining_amount = total_amount - all_records_amount;
            if remaining_amount <= 0 {
                return Err(Error::unknown("lottery has been drawn"))?;
            }
            total_amount = remaining_amount;
            items = items
                .into_iter()
                .zip(grouped_records)
                .map(|(mut item, record)| {
                    item.value -= record.len() as i32;
                    item
                })
                .collect();
        }
        let random_index = rng.gen_range(0..total_amount);
        let mut index = 0;
        let item = items
            .into_iter()
            .find(|item| {
                let remaining = item.value;
                if random_index < index + remaining {
                    true
                } else {
                    index += remaining;
                    false
                }
            })
            .ok_or_loss()?;
        record.item_id = item.id;
        record.user_id = user_id;
        log::info!("new_record: {:?}", new_record);
        let record = model::record::insert(&mut conn, new_record)?;
        Ok(Response::new(pb::record::CreateResponse {
            record: Some(record),
        }))
    }
    async fn update(
        &self,
        request: tonic::Request<pb::record::UpdateRequest>,
    ) -> Resp<pb::record::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::record::UpdateRequest { id, record } = request.into_inner();
        let record = record.ok_or_invalid()?;
        let record = model::record::update_by_id(&mut conn, id, record)?;
        Ok(Response::new(pb::record::UpdateResponse {
            record: Some(record),
        }))
    }
    async fn delete(
        &self,
        request: tonic::Request<pb::record::DeleteRequest>,
    ) -> Resp<pb::record::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::record::delete_by_id(&mut conn, id)?;
        Ok(Response::new(pb::record::DeleteResponse {}))
    }
}
