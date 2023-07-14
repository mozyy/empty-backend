use crate::{model::oauth::UserId, pb::lottery as pb};
use empty_utils::{diesel::db, errors::Error, tonic::Resp};
use tonic::{Request, Response};

use crate::model::lottery as model;

pub struct Service {
    db: db::DbPool,
}

impl Service {
    pub fn new_by_db(db: db::DbPool) -> Self {
        Self { db }
    }
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
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let (lotterys, paginated) = model::query_list(&mut conn, request).await?;
        Ok(Response::new(pb::ListResponse {
            lotterys,
            paginated,
        }))
    }

    async fn get(&self, request: Request<pb::GetRequest>) -> Resp<pb::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = model::query_by_id(&mut conn, request.into_inner().id).await?;
        Ok(Response::new(pb::GetResponse {
            lottery: Some(lottery),
        }))
    }

    async fn create(&self, request: Request<pb::CreateRequest>) -> Resp<pb::CreateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let mut lottery = request
            .into_inner()
            .lottery
            .ok_or_else(|| Error::StatusError(tonic::Status::data_loss("no blog")))?;
        lottery.user_id = user_id.clone();
        log::info!("user:{}, {:?}", user_id, lottery);
        let mut conn = self.db.get_conn()?;
        let lottery = model::insert(&mut conn, lottery).await?;
        Ok(Response::new(pb::CreateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn update(&self, request: Request<pb::UpdateRequest>) -> Resp<pb::UpdateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let pb::UpdateRequest { id, lottery } = request.into_inner();
        let mut lottery =
            lottery.ok_or_else(|| Error::StatusError(tonic::Status::data_loss("no blog")))?;
        lottery.user_id = user_id;
        let mut conn = self.db.get_conn()?;
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
