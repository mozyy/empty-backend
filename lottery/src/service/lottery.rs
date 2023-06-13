use crate::pb::lottery as pb;
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::model::lottery as model;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("lottery"),
        }
    }
}

#[tonic::async_trait]
impl pb::lottery_service_server::LotteryService for Service {
    async fn list(&self, request: Request<pb::ListRequest>) -> Resp<pb::ListResponse> {
        let mut conn = self.db.get_conn()?;
        let user_id = request.into_inner().user_id;
        let lotterys = match user_id {
            Some(user_id) => {
                let user_id = Uuid::parse_str(&user_id)
                    .map_err(|e|tonic::Status::invalid_argument(e.to_string()))?;
                model::query_list_by_user_id(&mut conn, user_id).await?
            }
            None => model::query_list(&mut conn).await?
        };
        Ok(Response::new(pb::ListResponse { lotterys }))
    }

    async fn get(&self, request: Request<pb::GetRequest>) -> Resp<pb::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = model::query_by_id(&mut conn, request.into_inner().id).await?;
        Ok(Response::new(pb::GetResponse {
            lottery: Some(lottery),
        }))
    }

    async fn create(&self, request: Request<pb::CreateRequest>) -> Resp<pb::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = request
            .into_inner()
            .lottery
            .ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        let lottery = model::insert(&mut conn, lottery).await?;
        Ok(Response::new(pb::CreateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn update(&self, request: Request<pb::UpdateRequest>) -> Resp<pb::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::UpdateRequest { id, lottery } = request.into_inner();
        let lottery = lottery
            .ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        let lottery = model::update_by_id(&mut conn, id, lottery).await?;
        Ok(Response::new(pb::UpdateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn delete(&self, request: Request<pb::DeleteRequest>) -> Resp<pb::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::DeleteResponse {}))
    }
}
