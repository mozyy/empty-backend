use async_trait::async_trait;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert, Result},
    tonic::Resp,
};
use rand::Rng;
use tonic::Response;

use crate::{
    configs::ADDR_CLIENT,
    model::{oauth::UserId, record as model},
    pb::record as pb,
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
impl pb::record_service_server::RecordService for Service {
    async fn list(&self, request: tonic::Request<pb::ListRequest>) -> Resp<pb::ListResponse> {
        let mut conn = self.db.get_conn()?;
        let (records, paginated) = model::query_list(&mut conn, request.into_inner())?;
        Ok(Response::new(pb::ListResponse { records, paginated }))
    }
    async fn get(&self, request: tonic::Request<pb::GetRequest>) -> Resp<pb::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let record = model::query_by_id(&mut conn, request.into_inner().id)?;
        Ok(Response::new(pb::GetResponse {
            record: Some(record),
        }))
    }
    async fn create(&self, request: tonic::Request<pb::CreateRequest>) -> Resp<pb::CreateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let mut new_record = request.into_inner().record.ok_or_invalid()?;
        let mut record = new_record.record.as_mut().ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let my_records = model::query_list_by_record(
            &mut conn,
            Some(pb::RecordQuery {
                id: None,
                user_id: Some(user_id.clone()),
                lottery_id: Some(record.lottery_id),
            }),
        )?;
        if !my_records.is_empty() {
            return Err(Error::unknown("already records"))?;
        }

        let mut client =
            crate::pb::lottery::lottery_service_client::LotteryServiceClient::connect(ADDR_CLIENT)
                .await
                .map_err(Error::other)?;
        let lottery = client
            .get(tonic::Request::new(crate::pb::lottery::GetRequest {
                id: record.lottery_id,
            }))
            .await?
            .into_inner()
            .lottery
            .ok_or_loss()?;
        let crate::pb::lottery::Lottery { lottery, items, .. } = lottery;
        let lottery = lottery.ok_or_loss()?;

        let r#type = crate::pb::lottery::Type::from_i32(lottery.r#type).ok_or_invalid()?;
        let all_records = model::query_list_by_record(
            &mut conn,
            Some(pb::RecordQuery {
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
        let mut rng = rand::thread_rng();
        let mut items = items.into_iter();
        let total_amount: i32 = items.clone().map(|item| item.value).sum();
        let item = match r#type {
            crate::pb::lottery::Type::Number => {
                let remaining_amount = total_amount - all_records.len() as i32;
                if remaining_amount <= 0 {
                    return Err(Error::unknown("lottery has been drawn"))?;
                }
                let random_index = rng.gen_range(0..remaining_amount);
                let mut index = 0;

                items
                    .find(|item| {
                        let mut already: i32 = 0;
                        all_records.iter().for_each(|record| {
                            if record.item_id == item.id {
                                already += 1
                            }
                        });
                        let remaining = item.value - already;
                        if random_index < index + remaining {
                            true
                        } else {
                            index += remaining;
                            false
                        }
                    })
                    .ok_or_loss()?
            }
            crate::pb::lottery::Type::Percent => {
                let random_index = rng.gen_range(0..total_amount);
                let mut index = 0;
                items
                    .find(|item| {
                        let mut already: i32 = 0;
                        all_records.iter().for_each(|record| {
                            if record.item_id == item.id {
                                already += 1
                            }
                        });
                        let remaining = item.value - already;
                        if random_index < index + remaining {
                            true
                        } else {
                            index += remaining;
                            false
                        }
                    })
                    .ok_or_loss()?
            }
        };
        record.item_id = item.id;
        let record = model::insert(&mut conn, new_record)?;
        Ok(Response::new(pb::CreateResponse {
            record: Some(record),
        }))
    }
    async fn update(&self, request: tonic::Request<pb::UpdateRequest>) -> Resp<pb::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::UpdateRequest { id, record } = request.into_inner();
        let record = record.ok_or_invalid()?;
        let record = model::update_by_id(&mut conn, id, record)?;
        Ok(Response::new(pb::UpdateResponse {
            record: Some(record),
        }))
    }
    async fn delete(&self, request: tonic::Request<pb::DeleteRequest>) -> Resp<pb::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::delete_by_id(&mut conn, id)?;
        Ok(Response::new(pb::DeleteResponse {}))
    }
}
