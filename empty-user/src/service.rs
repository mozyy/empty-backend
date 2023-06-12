use chrono::{Duration, Utc};
use diesel::Connection;
use empty_utils::{diesel::db, errors::ServiceError, tonic::Resp};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::{
    model,
    pb::{self, user_server},
};

#[derive(Clone)]
pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("empty_user"),
        }
    }
}

#[tonic::async_trait]
impl user_server::User for Service {
    async fn register(&self, request: Request<pb::RegisterRequest>) -> Resp<pb::RegisterResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::RegisterRequest { mobile, password } = request.into_inner();
        let info = model::info::NewInfo::new(mobile, password);
        let info_id = model::info::insert(&mut conn, info)?;
        let access_token =
            model::access_token::NewAccessToken::new(info_id, String::from("public"));
        let access_token = model::access_token::insert(&mut conn, access_token)?;
        let access_token = pb::AccessToken::from(access_token);
        Ok(Response::new(pb::RegisterResponse {
            access_token: Some(access_token),
        }))
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
        let access_token =
            model::access_token::NewAccessToken::new(info.id, String::from("public"));
        let access_token = model::access_token::insert(&mut conn, access_token)?;
        let access_token = pb::AccessToken::from(access_token);
        Ok(Response::new(pb::LoginResponse {
            access_token: Some(access_token),
        }))
    }
    async fn refresh(&self, request: Request<pb::RefreshRequest>) -> Resp<pb::RefreshResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::RefreshRequest { refresh_token } = request.into_inner();
        let access_token = conn.transaction::<_, ServiceError, _>(|conn| {
            let access_token =
                model::access_token::query_by_refresh_token(conn, refresh_token.clone())?;
            model::access_token::delete_by_refresh_token(conn, refresh_token)?;
            let access_token = access_token.refresh();
            let access_token = model::access_token::insert(conn, access_token)?;
            Ok(access_token)
        })?;
        let access_token = pb::AccessToken::from(access_token);
        Ok(Response::new(pb::RefreshResponse {
            access_token: Some(access_token),
        }))
    }
    async fn logout(&self, request: Request<pb::LogoutRequest>) -> Resp<pb::LogoutResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::LogoutRequest { access_token } = request.into_inner();
        model::access_token::delete_by_access_token(&mut conn, access_token)?;
        Ok(Response::new(pb::LogoutResponse {}))
    }
    async fn info(&self, request: Request<pb::InfoRequest>) -> Resp<pb::InfoResponse> {
        let mut conn = self.db.get_conn()?;
        let user_id = request.metadata().get("user_id").ok_or_else(|| {
            ServiceError::StatusError(tonic::Status::unauthenticated("no user_id"))
        })?;
        let user_id = user_id
            .to_str()
            .map_err(|e| ServiceError::String(format!("user_id:{e}")))?;
        let user_id = Uuid::parse_str(user_id)
            .map_err(|e| ServiceError::String(String::from(format!("user_id psrse:{e}"))))?;
        let info = model::info::query_by_id(&mut conn, user_id)?;
        let info = pb::Info::from(info);
        Ok(Response::new(pb::InfoResponse { info: Some(info) }))
    }
    async fn verify(&self, request: Request<pb::VerifyRequest>) -> Resp<pb::VerifyResponse> {
        let pb::VerifyRequest {
            access_token,
            resource,
        } = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let access_token = model::access_token::query_by_access_token(&mut conn, access_token)?;
        let now = Utc::now().naive_utc();
        let duration = now.signed_duration_since(access_token.updated_at);
        let expires_in = Duration::seconds(access_token.expires_in as i64);
        if duration >= expires_in {
            model::access_token::delete_by_access_token(&mut conn, access_token.access_token)?;
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
