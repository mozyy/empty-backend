use async_trait::async_trait;
use config::ADDR_CLIENT;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::{
    dao::{self, user as model},
    util::gen_resource_token,
};
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
        let request = request.into_inner();
        if request.grant_type != "refreshtoken" {
            Err(Error::String(String::from("grant_type invalid")))?;
        }
        let mut conn = self.db.get_conn()?;
        let pb::user::auth::RefreshResource {
            id,
            user_id,
            client_id,
            scope,
            ..
        } = dao::refresh_resource::query_by_refresh_token(&mut conn, request.refresh_token)?;
        let pb::user::auth::Client {
            name,
            default_expires_in,
            ..
        } = dao::client::query_by_id(&mut conn, client_id.parse().ok_or_invalid()?)?;
        let (resource, token) =
            gen_resource_token(user_id, client_id, name, default_expires_in, scope)?;
        dao::refresh_resource::insert(&mut conn, resource)?;
        dao::refresh_resource::delete_by_id(&mut conn, id)?;
        token.to_resp()
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
    async fn jwt(
        &self,
        request: Request<pb::user::auth::JwtPayload>,
    ) -> Resp<pb::user::auth::JwtPayload> {
        todo!()
    }
}
