use async_trait::async_trait;
use config::ADDR_CLIENT;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::dao::user as model;
use proto::pb;

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
            db: db::DbPool::new("user_v2"),
        }
    }
}

#[async_trait]
impl pb::user::auth::auth_service_server::AuthService for Service {
    async fn authorize(
        &self,
        request: Request<pb::user::auth::AuthorizeRequest>,
    ) -> Resp<pb::user::auth::AuthorizeResponse> {
        let request = request.into_inner();
        if request.response_type != "code" {
            Err(Error::String("response_type must be code".into()))?;
        }
        todo!()
    }
    async fn get_token(
        &self,
        request: Request<pb::user::auth::TokenRequest>,
    ) -> Resp<pb::user::auth::Token> {
        todo!()
    }
    async fn refresh(
        &self,
        request: Request<pb::user::auth::RefreshRequest>,
    ) -> Resp<pb::user::auth::Token> {
        todo!()
    }
    async fn client_list(
        &self,
        request: Request<pb::user::auth::ClientListRequest>,
    ) -> Resp<pb::user::auth::ClientListResponse> {
        todo!()
    }
    async fn client_create(
        &self,
        request: Request<pb::user::auth::ClientCreateRequest>,
    ) -> Resp<pb::user::auth::ClientCreateResponse> {
        todo!()
    }
    async fn config_list(
        &self,
        request: Request<pb::user::auth::ConfigListRequest>,
    ) -> Resp<pb::user::auth::ConfigListResponse> {
        todo!()
    }
    async fn config_create(
        &self,
        request: Request<pb::user::auth::ConfigCreateRequest>,
    ) -> Resp<pb::user::auth::ConfigCreateResponse> {
        todo!()
    }
}
