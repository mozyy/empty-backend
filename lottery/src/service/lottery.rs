use empty_utils::{
    diesel::db,
    errors::ErrorConvert,
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};

use crate::dao;
use proto::pb;

pub struct Service {
    db: db::DbPool,
}

impl Service {
    pub fn new_by_db(db: db::DbPool) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl pb::lottery::lottery::lottery_service_server::LotteryService for Service {
    async fn list(
        &self,
        request: Request<pb::lottery::lottery::ListRequest>,
    ) -> Resp<pb::lottery::lottery::ListResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let (lotterys, paginated) = dao::lottery::query_list(&mut conn, request)?;
        pb::lottery::lottery::ListResponse {
            lotterys,
            paginated,
        }
        .to_resp()
    }

    async fn get(
        &self,
        request: Request<pb::lottery::lottery::GetRequest>,
    ) -> Resp<pb::lottery::lottery::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let lottery = dao::lottery::query_by_id(&mut conn, request.into_inner().id)?;
        pb::lottery::lottery::GetResponse {
            lottery: Some(lottery),
        }
        .to_resp()
    }

    async fn create(
        &self,
        request: Request<pb::lottery::lottery::CreateRequest>,
    ) -> Resp<pb::lottery::lottery::CreateResponse> {
        let lottery = request.into_inner().lottery.ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let lottery = dao::lottery::insert(&mut conn, lottery)?;
        pb::lottery::lottery::CreateResponse {
            lottery: Some(lottery),
        }
        .to_resp()
    }

    async fn update(
        &self,
        request: Request<pb::lottery::lottery::UpdateRequest>,
    ) -> Resp<pb::lottery::lottery::UpdateResponse> {
        let pb::lottery::lottery::UpdateRequest { id, lottery } = request.into_inner();
        let lottery = lottery.ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let lottery = dao::lottery::update_by_id(&mut conn, id, lottery)?;
        pb::lottery::lottery::UpdateResponse {
            lottery: Some(lottery),
        }
        .to_resp()
    }

    async fn delete(
        &self,
        request: Request<pb::lottery::lottery::DeleteRequest>,
    ) -> Resp<pb::lottery::lottery::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        dao::lottery::delete_by_id(&mut conn, id)?;
        pb::lottery::lottery::DeleteResponse {}.to_resp()
    }
}
