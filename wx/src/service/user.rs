use async_trait::async_trait;
use config::ADDR_CLIENT;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::Resp,
};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::{dao::user as model, CLIENT_ID};
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
            db: db::DbPool::new("wx_v2"),
        }
    }
}

#[async_trait]
impl pb::wx::user::user_service_server::UserService for Service {
    async fn list(
        &self,
        _request: Request<pb::wx::user::ListRequest>,
    ) -> Resp<pb::wx::user::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let wx_users = model::query_list(&mut conn)?;
        log::debug!("get blogs");
        Ok(Response::new(pb::wx::user::ListResponse { wx_users }))
    }

    async fn get(
        &self,
        request: Request<pb::wx::user::GetRequest>,
    ) -> Resp<pb::wx::user::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        let wx_user = model::query_by_id(&mut conn, id)?;
        Ok(Response::new(pb::wx::user::GetResponse {
            wx_user: Some(wx_user),
        }))
    }
    async fn get_by_user_id(
        &self,
        request: Request<pb::wx::user::GetByUserIdRequest>,
    ) -> Resp<pb::wx::user::GetByUserIdResponse> {
        let mut conn = self.db.get_conn()?;
        let user_id = request.into_inner().user_id;
        let user_id = Uuid::parse_str(&user_id).ok_or_invalid()?;
        let wx_user = model::query_by_user_id(&mut conn, user_id)?;
        Ok(Response::new(pb::wx::user::GetByUserIdResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn create(
        &self,
        request: Request<pb::wx::user::CreateRequest>,
    ) -> Resp<pb::wx::user::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let wx_user = request.into_inner().wx_user.ok_or_invalid()?;
        let wx_user = model::insert(&mut conn, wx_user)?;
        Ok(Response::new(pb::wx::user::CreateResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn update(
        &self,
        request: Request<pb::wx::user::UpdateRequest>,
    ) -> Resp<pb::wx::user::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::wx::user::UpdateRequest { id, wx_user } = request.into_inner();
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        let wx_user = wx_user.ok_or_invalid()?;
        let wx_user = model::update_by_id(&mut conn, id, wx_user)?;
        Ok(Response::new(pb::wx::user::UpdateResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::wx::user::DeleteRequest>,
    ) -> Resp<pb::wx::user::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        model::delete_by_id(&mut conn, id)?;
        Ok(Response::new(pb::wx::user::DeleteResponse {}))
    }
    async fn login(
        &self,
        request: Request<pb::wx::user::LoginRequest>,
    ) -> Resp<pb::wx::user::LoginResponse> {
        let code = request.into_inner().code;
        let mut client = pb::wx::wx::wx_service_client::WxServiceClient::connect(ADDR_CLIENT)
            .await
            .map_err(Error::other)?;
        log::warn!("code: {code}");
        let resp = client
            .sns_jscode2session(pb::wx::wx::SnsJscode2sessionRequest::new(code))
            .await?;
        let resp = resp.into_inner();
        log::info!("wx sns_jscode2session success: {:?}", resp);
        let mut client =
            pb::auth::auth::auth_service_client::AuthServiceClient::connect(ADDR_CLIENT)
                .await
                .map_err(Error::other)?;
        log::info!("client success");
        let mut conn = self.db.get_conn()?;
        let (token, user) = match model::query_by_openid(&mut conn, resp.openid.clone()) {
            Ok(user) => {
                log::info!("query_by_openid success");
                let resp = client
                    .login(pb::auth::auth::LoginRequest {
                        user_id: user.user_id,
                        client_id: CLIENT_ID.to_string(),
                    })
                    .await?;
                log::info!("client login success");
                let resp = resp.into_inner();
                (resp.token, resp.user)
            }
            Err(e) => {
                log::info!("query_by_openid error: {}", e.to_string());
                let res = client
                    .register(pb::auth::auth::RegisterRequest {
                        client_id: CLIENT_ID.to_string(),
                    })
                    .await?;
                let res = res.into_inner();
                let user_id = res.user.clone().ok_or_invalid()?.id;
                let user = pb::wx::user::NewUser {
                    user_id,
                    openid: resp.openid,
                    unionid: resp.unionid,
                    session_key: resp.session_key,
                    name: String::from("user name"),
                    avatar: None,
                    mobile: None,
                };
                let _user = model::insert(&mut conn, user)?;
                (res.token, res.user)
            }
        };
        Ok(Response::new(pb::wx::user::LoginResponse { token, user }))
    }
    async fn info(
        &self,
        _request: Request<pb::wx::user::InfoRequest>,
    ) -> Resp<pb::wx::user::InfoResponse> {
        todo!();
    }
}
