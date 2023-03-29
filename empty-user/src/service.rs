
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use tonic::{Request, Response};

use crate::{
    model,
    pb::{self, user_server},
};

#[derive(Clone)]
pub struct Service {
    db: db::DbPool,
}
impl Service {
    pub fn new() -> Self {
        Self {
            db: db::DbPool::new(),
        }
    }
}

#[tonic::async_trait]
impl user_server::User for Service {
    async fn register(&self, request: Request<pb::RegisterRequest>) -> Resp<pb::RegisterResponse> {
        let mut conn = self.db.get_conn().map_err(ServiceError::from)?;
        let pb::RegisterRequest { mobile, password } = request.into_inner();
        let info = model::info::NewInfo::new(mobile, password);
        let info_id = model::info::insert(&mut conn, info)?;
        let token = model::token::NewToken::new(info_id);
        let token = model::token::insert(&mut conn, token)?;
        let token = pb::Token::from(token);
        Ok(Response::new(pb::RegisterResponse { token: Some(token) }))
    }
    async fn login(&self, request: Request<pb::LoginRequest>) -> Resp<pb::LoginResponse> {
        let mut conn = self.db.get_conn().map_err(ServiceError::from)?;
        let pb::LoginRequest { mobile, password } = request.into_inner();
        let info = model::info::query_by_mobile(&mut conn, mobile)?;
        // TODO: check password
        if password != info.password {
            return Err(ServiceError::StatusError(tonic::Status::permission_denied(
                "password error",
            ))
            .into());
        }
        let token = model::token::NewToken::new(info.id);
        let token = model::token::insert(&mut conn, token)?;
        let token = pb::Token::from(token);
        Ok(Response::new(pb::LoginResponse { token: Some(token) }))
    }
    async fn refresh(&self, _request: Request<pb::RefreshRequest>) -> Resp<pb::RefreshResponse> {
        todo!()
    }
    async fn logout(&self, _request: Request<pb::LogoutRequest>) -> Resp<pb::LogoutResponse> {
        todo!()
    }
    async fn info(&self, _request: Request<pb::InfoRequest>) -> Resp<pb::InfoResponse> {
        todo!()
    }
}
