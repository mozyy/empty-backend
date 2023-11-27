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
impl pb::user::user::user_service_server::UserService for Service {
    async fn login_mobile(
        &self,
        request: Request<pb::user::user::LoginMobileRequest>,
    ) -> Resp<pb::user::user::LoginResponse> {
        todo!()
    }
    async fn register_mobile(
        &self,
        request: Request<pb::user::user::RegisterMobileRequest>,
    ) -> Resp<pb::user::user::LoginResponse> {
        todo!()
    }
    async fn login_weixin(
        &self,
        request: Request<pb::user::user::LoginWeixinRequest>,
    ) -> Resp<pb::user::user::LoginResponse> {
        todo!()
    }
}
