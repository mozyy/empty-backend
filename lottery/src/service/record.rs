use async_trait::async_trait;
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
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
        Self { db: db }
    }
}

#[async_trait]
impl pb::record_service_server::RecordService for Service {
    async fn list(&self, request: tonic::Request<pb::ListRequest>) -> Resp<pb::ListResponse> {
        let mut conn = self.db.get_conn()?;
        let (records, paginated) = model::query_list(&mut conn, request.into_inner()).await?;
        Ok(Response::new(pb::ListResponse { records, paginated }))
    }
    async fn get(&self, request: tonic::Request<pb::GetRequest>) -> Resp<pb::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let record = model::query_by_id(&mut conn, request.into_inner().id).await?;
        Ok(Response::new(pb::GetResponse {
            record: Some(record),
        }))
    }
    async fn create(&self, request: tonic::Request<pb::CreateRequest>) -> Resp<pb::CreateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let mut conn = self.db.get_conn()?;
        let record = request
            .into_inner()
            .record
            .ok_or_else(|| tonic::Status::data_loss("no new record"))?;
        let client =
            crate::pb::lottery::lottery_service_client::LotteryServiceClient::connect(ADDR_CLIENT)
                .await
                .unwrap();
        let record = model::insert(&mut conn, record).await?;
        Ok(Response::new(pb::CreateResponse {
            record: Some(record),
        }))
    }
    async fn update(&self, request: tonic::Request<pb::UpdateRequest>) -> Resp<pb::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::UpdateRequest { id, record } = request.into_inner();
        let record = record.ok_or_else(|| tonic::Status::data_loss("no new record"))?;
        let record = model::update_by_id(&mut conn, id, record).await?;
        Ok(Response::new(pb::UpdateResponse {
            record: Some(record),
        }))
    }
    async fn delete(&self, request: tonic::Request<pb::DeleteRequest>) -> Resp<pb::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::DeleteResponse {}))
    }
}
