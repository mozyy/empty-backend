use crate::{configs::ADDR_CLIENT, pb};
use async_trait::async_trait;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert},
    tonic::Resp,
};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::model::user as model;

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
            db: db::DbPool::new("lottery"),
        }
    }
}

#[async_trait]
impl pb::user::user_service_server::UserService for Service {
    async fn list(&self, _request: Request<pb::user::ListRequest>) -> Resp<pb::user::ListResponse> {
        log::debug!("request list");
        let mut conn = self.db.get_conn()?;
        log::debug!("get conn");
        let wx_users = model::query_list(&mut conn).await?;
        log::debug!("get blogs");
        Ok(Response::new(pb::user::ListResponse { wx_users }))
    }

    async fn get(&self, request: Request<pb::user::GetRequest>) -> Resp<pb::user::GetResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        let wx_user = model::query_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::user::GetResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn create(
        &self,
        request: Request<pb::user::CreateRequest>,
    ) -> Resp<pb::user::CreateResponse> {
        let mut conn = self.db.get_conn()?;
        let wx_user = request.into_inner().wx_user.ok_or_invalid()?;
        let wx_user = model::insert(&mut conn, wx_user).await?;
        Ok(Response::new(pb::user::CreateResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn update(
        &self,
        request: Request<pb::user::UpdateRequest>,
    ) -> Resp<pb::user::UpdateResponse> {
        let mut conn = self.db.get_conn()?;
        let pb::user::UpdateRequest { id, wx_user } = request.into_inner();
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        let wx_user = wx_user.ok_or_invalid()?;
        let wx_user = model::update_by_id(&mut conn, id, wx_user).await?;
        Ok(Response::new(pb::user::UpdateResponse {
            wx_user: Some(wx_user),
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::user::DeleteRequest>,
    ) -> Resp<pb::user::DeleteResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().id;
        let id = Uuid::parse_str(&id).ok_or_invalid()?;
        model::delete_by_id(&mut conn, id).await?;
        Ok(Response::new(pb::user::DeleteResponse {}))
    }
    async fn login(
        &self,
        request: Request<pb::user::LoginRequest>,
    ) -> Resp<pb::user::LoginResponse> {
        let code = request.into_inner().code;
        let mut client = pb::wx::wx_service_client::WxServiceClient::connect(ADDR_CLIENT)
            .await
            .map_err(Error::other)?;
        log::warn!("code: {code}");
        let resp = client
            .sns_jscode2session(pb::wx::SnsJscode2sessionRequest::new(code))
            .await?;
        let resp = resp.into_inner();
        let mut conn = self.db.get_conn()?;
        let mut client = pb::oauth::o_auth_service_client::OAuthServiceClient::connect(ADDR_CLIENT)
            .await
            .map_err(Error::other)?;
        let token = match model::query_by_openid(&mut conn, resp.openid.clone()).await {
            Ok(user) => {
                let resp = client
                    .login(pb::oauth::LoginRequest {
                        user_id: user.user_id,
                    })
                    .await?;
                resp.into_inner().token
            }
            Err(e) => {
                log::info!("query_by_openid error: {}", e.to_string());
                let res = client.register(pb::oauth::RegisterRequest {}).await?;
                let res = res.into_inner();
                let user_id = res.user.ok_or_invalid()?.id;
                let user = pb::user::NewWxUser {
                    user_id,
                    openid: resp.openid,
                    unionid: resp.unionid,
                    session_key: resp.session_key,
                    name: String::from("user name"),
                    avatar: None,
                    mobile: None,
                };
                let _user = model::insert(&mut conn, user).await?;
                res.token
            }
        };
        dbg!(&token);
        Ok(Response::new(pb::user::LoginResponse { token }))
    }
    async fn info(&self, _request: Request<pb::user::InfoRequest>) -> Resp<pb::user::InfoResponse> {
        todo!();
    }
}
