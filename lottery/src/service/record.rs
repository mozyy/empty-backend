use async_trait::async_trait;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert, Result},
    tonic::Resp,
};
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
        let new_record = request.into_inner().record.ok_or_invalid()?;
        let record = new_record.clone().record.ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let my_records = model::query_list_by_record(
            &mut conn,
            Some(pb::RecordQuery {
                id: None,
                user_id: Some(user_id.clone()),
                lottery_id: Some(record.lottery_id),
            }),
        )?;
        if my_records.len() > 0 {
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
        let crate::pb::lottery::Lottery {
            lottery,
            items,
            remarks,
        } = lottery;
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
        match r#type {
            crate::pb::lottery::Type::Number => {
                let items = items.into_iter();
                let total_count: i32 = items.map(|item| item.value).sum();
                if total_count <= all_records.len() as i32 {
                    return Err(Error::unknown("lottery has been drawn"))?;
                }
                // let items = items.filter_map(|item| {}).collect();
            }
            crate::pb::lottery::Type::Percent => todo!(),
        }
        // 生成一个随机数生成器
        // let mut rng = rand::thread_rng();

        // // 生成一个 0 到 (total_remaining - 1) 之间的随机数
        // let random_index = rng.gen_range(0..total_remaining);

        let my_records = model::query_list_by_record(
            &mut conn,
            Some(pb::RecordQuery {
                id: None,
                user_id: Some(user_id),
                lottery_id: Some(record.lottery_id),
            }),
        )?;
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
