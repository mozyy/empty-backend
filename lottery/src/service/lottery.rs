use crate::{model::oauth::UserId, pb};
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::Resp,
};
use tonic::{Request, Response};

use crate::model;

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
impl pb::lottery::lottery_service_server::LotteryService for Service {
    async fn list(
        &self,
        request: Request<pb::lottery::ListRequest>,
    ) -> Resp<pb::lottery::ListResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let (lotterys, paginated) = model::lottery::query_list(&mut conn, request)?;
        Ok(Response::new(pb::lottery::ListResponse {
            lotterys,
            paginated,
        }))
    }

    async fn get(
        &self,
        request: Request<pb::lottery::GetRequest>,
    ) -> Resp<pb::lottery::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::query_by_id(&mut conn, request.into_inner().id)?;
        Ok(Response::new(pb::lottery::GetResponse {
            lottery: Some(lottery),
        }))
    }

    async fn create(
        &self,
        request: Request<pb::lottery::CreateRequest>,
    ) -> Resp<pb::lottery::CreateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let mut new_lottery = request.into_inner().lottery.ok_or_invalid()?;
        let mut lottery = new_lottery.lottery.as_mut().ok_or_invalid()?;
        lottery.user_id = user_id;
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::insert(&mut conn, new_lottery)?;
        Ok(Response::new(pb::lottery::CreateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn update(
        &self,
        request: Request<pb::lottery::UpdateRequest>,
    ) -> Resp<pb::lottery::UpdateResponse> {
        let user_id = UserId::try_from(&request)?.0;
        let pb::lottery::UpdateRequest { id, lottery } = request.into_inner();
        let mut new_lottery = lottery.ok_or_invalid()?;
        let mut lottery = new_lottery.lottery.as_mut().ok_or_invalid()?;
        lottery.user_id = user_id;
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::update_by_id(&mut conn, id, new_lottery)?;
        Ok(Response::new(pb::lottery::UpdateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::lottery::DeleteRequest>,
    ) -> Resp<pb::lottery::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::lottery::delete_by_id(&mut conn, id)?;
        Ok(Response::new(pb::lottery::DeleteResponse {}))
    }
}
