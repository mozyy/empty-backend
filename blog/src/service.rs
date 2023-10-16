use empty_utils::{
    diesel::db,
    errors::Error,
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};

use crate::dao;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("blog_v2"),
        }
    }
}
use proto::pb;

#[tonic::async_trait]
impl pb::blog::blog::blog_service_server::BlogService for Service {
    async fn list(
        &self,
        request: Request<pb::blog::blog::ListRequest>,
    ) -> Resp<pb::blog::blog::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let (blogs, paginated) = dao::query_list(&mut conn, request.into_inner()).await?;
        log::debug!("get blogs");
        pb::blog::blog::ListResponse { blogs, paginated }.to_resp()
    }

    async fn get(
        &self,
        request: Request<pb::blog::blog::GetRequest>,
    ) -> Resp<pb::blog::blog::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let blog = dao::query_by_id(&mut conn, request.into_inner().id).await?;
        pb::blog::blog::GetResponse { blog: Some(blog) }.to_resp()
    }

    async fn create(
        &self,
        request: Request<pb::blog::blog::CreateRequest>,
    ) -> Resp<pb::blog::blog::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let blog = request
            .into_inner()
            .blog
            .ok_or_else(|| Error::StatusError(tonic::Status::data_loss("no blog")))?;
        let blog = dao::insert(&mut conn, blog).await?;
        pb::blog::blog::CreateResponse { blog: Some(blog) }.to_resp()
    }

    async fn update(
        &self,
        request: Request<pb::blog::blog::UpdateRequest>,
    ) -> Resp<pb::blog::blog::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::blog::blog::UpdateRequest { id, blog } = request.into_inner();
        let blog = blog.ok_or_else(|| Error::StatusError(tonic::Status::data_loss("no blog")))?;
        let blog = dao::update_by_id(&mut conn, id, blog).await?;
        pb::blog::blog::UpdateResponse { blog: Some(blog) }.to_resp()
    }

    async fn delete(
        &self,
        request: Request<pb::blog::blog::DeleteRequest>,
    ) -> Resp<pb::blog::blog::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        dao::delete_by_id(&mut conn, id).await?;
        pb::blog::blog::DeleteResponse {}.to_resp()
    }
}
