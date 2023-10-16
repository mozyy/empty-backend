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
        let (favorites, paginated) = dao::favorite::query_list(&mut conn, request)?;
        pb::lottery::favorite::ListResponse {
            favorites,
            paginated,
        }
        .to_resp()
    }
    async fn get(
        &self,
        request: Request<pb::lottery::favorite::GetRequest>,
    ) -> Resp<pb::lottery::favorite::GetResponse> {
        let id = request.into_inner().id;
        let mut conn = self.db.get_conn()?;
        let favorite = dao::favorite::query_by_id(&mut conn, id)?;
        pb::lottery::favorite::GetResponse {
            favorite: Some(favorite),
        }
        .to_resp()
    }
    async fn create(
        &self,
        request: Request<pb::lottery::favorite::CreateRequest>,
    ) -> Resp<pb::lottery::favorite::CreateResponse> {
        let favorite = request.into_inner().favorite.ok_or_loss()?;
        let mut conn = self.db.get_conn()?;
        let favorite = dao::favorite::insert(&mut conn, favorite)?;
        pb::lottery::favorite::CreateResponse {
            favorite: Some(favorite),
        }
        .to_resp()
    }
    async fn update(
        &self,
        request: Request<pb::lottery::favorite::UpdateRequest>,
    ) -> Resp<pb::lottery::favorite::UpdateResponse> {
        let pb::lottery::favorite::UpdateRequest { id, favorite } = request.into_inner();
        let favorite = favorite.ok_or_loss()?;
        let mut conn = self.db.get_conn()?;
        let favorite = dao::favorite::update_by_id(&mut conn, id, favorite)?;
        pb::lottery::favorite::UpdateResponse {
            favorite: Some(favorite),
        }
        .to_resp()
    }
    async fn delete(
        &self,
        request: Request<pb::lottery::favorite::DeleteRequest>,
    ) -> Resp<pb::lottery::favorite::DeleteResponse> {
        let id = request.into_inner().id;
        let mut conn = self.db.get_conn()?;
        dao::favorite::delete_by_id(&mut conn, id)?;
        pb::lottery::favorite::DeleteResponse {}.to_resp()
    }
}
