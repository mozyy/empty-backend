use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use proto::pb::blog::{self, blog_service_server::BlogService};
use tonic::{Request, Response};

use crate::model;

#[derive(Default)]
pub struct Service {
    db: db::DbPool,
}

#[tonic::async_trait]
impl BlogService for Service {
    async fn list(&self, _request: Request<blog::ListRequest>) -> Resp<blog::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let blogs = model::query_list(&mut conn).await?;
        log::debug!("get blogs");
        Ok(Response::new(blog::ListResponse {
            blogs: blogs.into_iter().map(blog::Blog::from).collect(),
        }))
    }

    async fn get(&self, request: Request<blog::GetRequest>) -> Resp<blog::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let blog = model::query_by_id(&mut conn, request.into_inner().id).await?;
        Ok(Response::new(blog::GetResponse {
            blog: Some(blog.into()),
        }))
    }

    async fn create(&self, request: Request<blog::CreateRequest>) -> Resp<blog::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let blog = request
            .into_inner()
            .blog
            .ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        let blog = model::insert(&mut conn, blog.into()).await?;
        Ok(Response::new(blog::CreateResponse {
            blog: Some(blog.into()),
        }))
    }

    async fn update(&self, request: Request<blog::UpdateRequest>) -> Resp<blog::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let blog::UpdateRequest { id, blog } = request.into_inner();
        let blog =
            blog.ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        let blog = model::update_by_id(&mut conn, id, blog.into()).await?;
        Ok(Response::new(blog::UpdateResponse {
            blog: Some(blog.into()),
        }))
    }

    async fn delete(&self, request: Request<blog::DeleteRequest>) -> Resp<blog::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(blog::DeleteResponse {}))
    }
}
