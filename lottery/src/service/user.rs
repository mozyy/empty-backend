use crate::{configs::ADDR, pb::user as pb, service};
use async_trait::async_trait;
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::model::user as model;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("lottery"),
        }
    }
}

#[async_trait]
impl pb::user_service_server::UserService for Service {
    async fn list(&self, _request: Request<pb::ListRequest>) -> Resp<pb::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let users = model::query_list(&mut conn).await?;
        log::debug!("get blogs");
        Ok(Response::new(pb::ListResponse { users }))
    }

    async fn get(&self, request: Request<pb::GetRequest>) -> Resp<pb::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id =
            Uuid::parse_str(&id).map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let user = model::query_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::GetResponse { user: Some(user) }))
    }

    async fn create(&self, request: Request<pb::CreateRequest>) -> Resp<pb::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let user = request
            .into_inner()
            .user
            .ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no user")))?;
        let user = model::insert(&mut conn, user).await?;
        Ok(Response::new(pb::CreateResponse { user: Some(user) }))
    }

    async fn update(&self, request: Request<pb::UpdateRequest>) -> Resp<pb::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::UpdateRequest { id, user } = request.into_inner();
        let id =
            Uuid::parse_str(&id).map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let user =
            user.ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        let user = model::update_by_id(&mut conn, id, user).await?;
        Ok(Response::new(pb::UpdateResponse { user: Some(user) }))
    }

    async fn delete(&self, request: Request<pb::DeleteRequest>) -> Resp<pb::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id =
            Uuid::parse_str(&id).map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::DeleteResponse {}))
    }
    async fn login(&self, request: Request<pb::LoginRequest>) -> Resp<pb::LoginResponse> {
        let code = request.into_inner().code;
        let mut client = crate::pb::wx::wx_service_client::WxServiceClient::connect(ADDR)
            .await
            .unwrap();
        let resp = client
            .sns_jscode2session(crate::pb::wx::SnsJscode2sessionRequest::new(code))
            .await?;
        dbg!(resp);
        // let resp = service::wx::Wx::sns_jscode2session(
        //     crate::model::wx::SnsJscode2sessionRequest::new(code),
        // )
        // .await?;
        // let mut conn = self.db.get_conn()?;
        // let user = request
        //     .into_inner()
        //     .user
        //     .ok_or_else(|| ServiceError::StatusError(tonic::Status::data_loss("no blog")))?;
        // let user = model::insert(&mut conn, user).await?;
        // Ok(Response::new(pb::CreateResponse { user: Some(user) }));
        todo!();
    }
    async fn info(&self, request: Request<pb::InfoRequest>) -> Resp<pb::InfoResponse> {
        todo!();
    }
}
