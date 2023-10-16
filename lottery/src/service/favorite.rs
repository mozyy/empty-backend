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
impl pb::lottery::favorite::favorite_service_server::FavoriteService for Service {
    async fn list(
        &self,
        request: Request<pb::lottery::favorite::ListRequest>,
    ) -> Resp<pb::lottery::favorite::ListResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        dao::favorite::query_list(&mut conn, request).to_resp()
    }
    async fn get(
        &self,
        request: Request<pb::lottery::favorite::GetRequest>,
    ) -> Resp<pb::lottery::favorite::GetResponse> {
        todo!()
    }
    async fn create(
        &self,
        request: Request<pb::lottery::favorite::CreateRequest>,
    ) -> Resp<pb::lottery::favorite::CreateResponse> {
        todo!()
    }
    async fn update(
        &self,
        request: Request<pb::lottery::favorite::UpdateRequest>,
    ) -> Resp<pb::lottery::favorite::UpdateResponse> {
        todo!()
    }
    async fn delete(
        &self,
        request: Request<pb::lottery::favorite::DeleteRequest>,
    ) -> Resp<pb::lottery::favorite::DeleteResponse> {
        todo!()
    }
}
