use empty_utils::{
    diesel::db,
    errors::ErrorConvert,
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};

use proto::pb;

use crate::dao;

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
            db: db::DbPool::new("oss_v2"),
        }
    }
}

#[tonic::async_trait]
impl pb::oss::oss::oss_service_server::OssService for Service {
    async fn list(
        &self,
        request: Request<pb::oss::oss::ListRequest>,
    ) -> Resp<pb::oss::oss::ListResponse> {
        let mut conn = self.db.get_conn()?;
        let (oss, paginated) = dao::query_list(&mut conn, request.into_inner())?;
        pb::oss::oss::ListResponse { oss, paginated }.to_resp()
    }
    async fn get(
        &self,
        request: Request<pb::oss::oss::GetRequest>,
    ) -> Resp<pb::oss::oss::GetResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let oss = dao::query_by_id(&mut conn, request.id)?;
        pb::oss::oss::GetResponse { oss: Some(oss) }.to_resp()
    }
    async fn create(
        &self,
        request: Request<pb::oss::oss::CreateRequest>,
    ) -> Resp<pb::oss::oss::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let oss = request.into_inner().oss.ok_or_invalid()?;
        let oss = dao::insert(&mut conn, oss)?;
        pb::oss::oss::CreateResponse { oss: Some(oss) }.to_resp()
    }
    async fn update(
        &self,
        request: Request<pb::oss::oss::UpdateRequest>,
    ) -> Resp<pb::oss::oss::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::oss::oss::UpdateRequest { id, oss } = request.into_inner();
        let oss = oss.ok_or_invalid()?;
        let oss = dao::update_by_id(&mut conn, id, oss)?;
        pb::oss::oss::UpdateResponse { oss: Some(oss) }.to_resp()
    }
    async fn delete(
        &self,
        request: Request<pb::oss::oss::DeleteRequest>,
    ) -> Resp<pb::oss::oss::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        dao::delete_by_id(&mut conn, request.into_inner().id)?;
        pb::oss::oss::DeleteResponse {}.to_resp()
    }
}
