use empty_utils::{diesel::db, errors::Error, tonic::Resp};
use tonic::{Request, Response};

use crate::model;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("empty_blog"),
        }
    }
}
use proto::pb;

#[tonic::async_trait]
impl pb::blog::blog::blog_service_server::BlogService for Service {
    async fn list(
        &self,
        _request: Request<pb::blog::blog::ListRequest>,
    ) -> Resp<pb::blog::blog::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let blogs = model::query_list(&mut conn).await?;
        log::debug!("get blogs");
        Ok(Response::new(pb::blog::blog::ListResponse { blogs }))
    }

    async fn get(
        &self,
        request: Request<pb::blog::blog::GetRequest>,
    ) -> Resp<pb::blog::blog::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let blog = model::query_by_id(&mut conn, request.into_inner().id).await?;
        Ok(Response::new(pb::blog::blog::GetResponse {
            blog: Some(blog),
        }))
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
        let blog = model::insert(&mut conn, blog).await?;
        Ok(Response::new(pb::blog::blog::CreateResponse {
            blog: Some(blog),
        }))
    }

    async fn update(
        &self,
        request: Request<pb::blog::blog::UpdateRequest>,
    ) -> Resp<pb::blog::blog::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::blog::blog::UpdateRequest { id, blog } = request.into_inner();
        let blog = blog.ok_or_else(|| Error::StatusError(tonic::Status::data_loss("no blog")))?;
        let blog = model::update_by_id(&mut conn, id, blog).await?;
        Ok(Response::new(pb::blog::blog::UpdateResponse {
            blog: Some(blog),
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::blog::blog::DeleteRequest>,
    ) -> Resp<pb::blog::blog::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::blog::blog::DeleteResponse {}))
    }
}
