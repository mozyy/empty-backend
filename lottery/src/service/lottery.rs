use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::Resp,
};
use tonic::{Request, Response};

use crate::model;
use proto::{pb, UserId};

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
impl pb::lottery::lottery::lottery_service_server::LotteryService for Service {
    async fn list(
        &self,
        request: Request<pb::lottery::lottery::ListRequest>,
    ) -> Resp<pb::lottery::lottery::ListResponse> {
        let user_id = UserId::try_from(&request)?.to_string();
        let mut request = request.into_inner();
        match &mut request.lottery {
            Some(lottery) => {
                lottery.user_id = Some(user_id);
            }
            None => {
                request.lottery = Some(pb::lottery::lottery::LotteryQuery {
                    user_id: Some(user_id),
                    ..Default::default()
                });
            }
        };
        let mut conn = self.db.get_conn()?;
        let (lotterys, paginated) = model::lottery::query_list(&mut conn, request)?;
        Ok(Response::new(pb::lottery::lottery::ListResponse {
            lotterys,
            paginated,
        }))
    }

    async fn get(
        &self,
        request: Request<pb::lottery::lottery::GetRequest>,
    ) -> Resp<pb::lottery::lottery::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::query_by_id(&mut conn, request.into_inner().id)?;
        Ok(Response::new(pb::lottery::lottery::GetResponse {
            lottery: Some(lottery),
        }))
    }

    async fn create(
        &self,
        request: Request<pb::lottery::lottery::CreateRequest>,
    ) -> Resp<pb::lottery::lottery::CreateResponse> {
        let user_id = UserId::try_from(&request)?.to_string();
        let mut new_lottery = request.into_inner().lottery.ok_or_invalid()?;
        let mut lottery = new_lottery.lottery.as_mut().ok_or_invalid()?;
        lottery.user_id = user_id;
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::insert(&mut conn, new_lottery)?;
        Ok(Response::new(pb::lottery::lottery::CreateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn update(
        &self,
        request: Request<pb::lottery::lottery::UpdateRequest>,
    ) -> Resp<pb::lottery::lottery::UpdateResponse> {
        let user_id = UserId::try_from(&request)?.to_string();
        let pb::lottery::lottery::UpdateRequest { id, lottery } = request.into_inner();
        let mut new_lottery = lottery.ok_or_invalid()?;
        let mut lottery = new_lottery.lottery.as_mut().ok_or_invalid()?;
        lottery.user_id = user_id;
        let mut conn = self.db.get_conn()?;
        let lottery = model::lottery::update_by_id(&mut conn, id, new_lottery)?;
        Ok(Response::new(pb::lottery::lottery::UpdateResponse {
            lottery: Some(lottery),
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::lottery::lottery::DeleteRequest>,
    ) -> Resp<pb::lottery::lottery::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::lottery::delete_by_id(&mut conn, id)?;
        Ok(Response::new(pb::lottery::lottery::DeleteResponse {}))
    }
}
