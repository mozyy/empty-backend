use std::ops::Add;

use chrono::{Duration, Utc};
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use tonic::{Request, Response};

use crate::{
    model::{
        self,
        access_token::{self, delete_by_access_token},
    },
    pb::{self, user_server},
};

#[derive(Clone, Default)]
pub struct Service {
    db: db::DbPool,
}

impl Service {}

#[tonic::async_trait]
impl user_server::User for Service {
    async fn register(&self, request: Request<pb::RegisterRequest>) -> Resp<pb::RegisterResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::RegisterRequest { mobile, password } = request.into_inner();
        let info = model::info::NewInfo::new(mobile, password);
        let info_id = model::info::insert(&mut conn, info)?;
        let token = model::token::NewToken::new(info_id, String::from("public"));
        let token = model::token::insert(&mut conn, token)?;
        let token = pb::Token::from(token);
        Ok(Response::new(pb::RegisterResponse { token: Some(token) }))
    }
    async fn login(&self, request: Request<pb::LoginRequest>) -> Resp<pb::LoginResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::LoginRequest { mobile, password } = request.into_inner();
        let info = model::info::query_by_mobile(&mut conn, mobile)?;
        // TODO: check password
        if password != info.password {
            return Err(ServiceError::StatusError(tonic::Status::permission_denied(
                "password error",
            ))
            .into());
        }
        let token = model::token::NewToken::new(info.id, String::from("public"));
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
    async fn verify(&self, request: Request<pb::VerifyRequest>) -> Resp<pb::VerifyResponse> {
        let pb::VerifyRequest {
            access_token,
            resource,
        } = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let access_token = model::access_token::query_by_access_token(&mut conn, access_token)?;
        let now = Utc::now().naive_utc();
        let duration = now.signed_duration_since(access_token.created_at);
        let expires_in = Duration::seconds(access_token.expires_in as i64);
        if duration >= expires_in {
            delete_by_access_token(&mut conn, access_token.access_token)?;
            return Err(ServiceError::StatusError(tonic::Status::permission_denied(
                "no permission",
            ))
            .into());
        }
        // TODO: check scope
        log::debug!("verify resource:{resource}");
        if access_token.scope != String::from("public") {
            return Err(ServiceError::StatusError(tonic::Status::permission_denied(
                "no permission",
            ))
            .into());
        }
        Ok(Response::new(pb::VerifyResponse {
            id: access_token.info_id.to_string(),
        }))
    }
}
