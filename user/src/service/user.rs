use async_trait::async_trait;
use config::ADDR_CLIENT;
use diesel::expression::is_aggregate::No;
use empty_utils::{
    diesel::db,
    errors::{Error, ErrorConvert, Result},
    tonic::{Resp, ToResp},
};
use password_auth::{generate_hash, verify_password};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{dao, util::gen_resource_token};
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
    ) -> Resp<pb::user::auth::Token> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let mobile = dao::mobile::query_by_mobile(&mut conn, request.mobile)?;
        verify_password(request.password, &mobile.password).ok_or_invalid()?;
        let user = dao::user::query_by_mobile_id(&mut conn, mobile.id)?;
        let pb::user::auth::Client {
            id,
            name,
            default_expires_in,
            default_scope,
            ..
        } = dao::client::query_by_id(&mut conn, request.client_id.parse().ok_or_invalid()?)?;
        let (resource, token) =
            gen_resource_token(user.id, id, name, default_expires_in, default_scope)?;
        dao::refresh_resource::insert(&mut conn, resource)?;
        token.to_resp()
    }
    async fn register_mobile(
        &self,
        request: Request<pb::user::user::RegisterMobileRequest>,
    ) -> Resp<pb::user::auth::Token> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        if dao::mobile::query_by_mobile(&mut conn, request.mobile.clone())
            .is_err_and(|e| !e.is_not_found())
        {
            Err(Error::StatusError(Status::already_exists(
                "mobile already exists",
            )))?;
        }
        let password = generate_hash(request.password);
        let info = pb::user::user::NewInfo {
            name: String::from("user name"),
            avatar: None,
        };
        let info = dao::info::insert(&mut conn, info)?;
        let mobile = pb::user::user::NewMobile {
            mobile: request.mobile,
            password,
        };
        let mobile = dao::mobile::insert(&mut conn, mobile)?;
        let user = pb::user::user::NewUser {
            info_id: info.id,
            mobile_id: Some(mobile.id),
            weixin_id: None,
        };
        let user = dao::user::insert(&mut conn, user)?;
        let pb::user::auth::Client {
            id,
            name,
            default_expires_in,
            default_scope,
            ..
        } = dao::client::query_by_id(&mut conn, request.client_id.parse().ok_or_invalid()?)?;
        let (resource, token) =
            gen_resource_token(user.id, id, name, default_expires_in, default_scope)?;

        dao::refresh_resource::insert(&mut conn, resource)?;
        token.to_resp()
    }
    async fn login_weixin(
        &self,
        request: Request<pb::user::user::LoginWeixinRequest>,
    ) -> Resp<pb::user::auth::Token> {
        let request = request.into_inner();
        let code = request.code;
        let mut client = pb::wx::wx::wx_service_client::WxServiceClient::connect(ADDR_CLIENT)
            .await
            .map_err(Error::other)?;
        let resp = client
            .sns_jscode2session(pb::wx::wx::SnsJscode2sessionRequest::new(code))
            .await?;
        let resp = resp.into_inner();
        log::info!("wx sns_jscode2session success: {:?}", resp);

        let new_weixin = pb::user::user::NewWeixin {
            openid: resp.openid.clone(),
            unionid: resp.unionid,
            session_key: resp.session_key,
        };

        let mut conn = self.db.get_conn()?;

        let user = match dao::weixin::query_by_open_id(&mut conn, resp.openid) {
            Ok(weixin) => {
                dao::weixin::update_by_id(&mut conn, weixin.id, new_weixin)?;
                let user = dao::user::query_by_weixin_id(&mut conn, weixin.id)?;
                user
            }
            Err(e) if e.is_not_found() => {
                log::info!("not found:{e}");
                let info = pb::user::user::NewInfo {
                    name: String::from("user name"),
                    avatar: None,
                };
                let info = dao::info::insert(&mut conn, info)?;
                
                let weixin = dao::weixin::insert(&mut conn, new_weixin)?;
                let user = pb::user::user::NewUser {
                    info_id: info.id,
                    mobile_id: None,
                    weixin_id: Some(weixin.id),
                };
                let user = dao::user::insert(&mut conn, user)?;
                user
            }
            Err(e) => Err(e)?,
        };
        let pb::user::auth::Client {
            id,
            name,
            default_expires_in,
            default_scope,
            ..
        } = dao::client::query_by_id(&mut conn, request.client_id.parse().ok_or_invalid()?)?;
        let (resource, token) =
            gen_resource_token(user.id, id, name, default_expires_in, default_scope)?;
        dao::refresh_resource::insert(&mut conn, resource)?;
        token.to_resp()
    }

    async fn get(
        &self,
        request: Request<pb::user::user::GetRequest>,
    ) -> Resp<pb::user::user::GetResponse> {
        let id = request.into_inner().id.parse().ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let user = dao::user::query_by_id(&mut conn, id)?;
        pb::user::user::GetResponse{user:Some(user)}.to_resp()
    }
    async fn get_info(
        &self,
        request: Request<pb::user::user::GetInfoRequest>,
    ) -> Resp<pb::user::user::GetInfoResponse> {
        let id = request.into_inner().id;
        let mut conn = self.db.get_conn()?;
        let info = dao::info::query_by_id(&mut conn, id)?;
        pb::user::user::GetInfoResponse{info:Some(info)}.to_resp()
    }
    async fn update_info(
        &self,
        request: Request<pb::user::user::UpdateInfoRequest>,
    ) -> Resp<pb::user::user::UpdateInfoResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let info = dao::info::update_by_id(&mut conn, request.id, request.info.ok_or_loss()?)?;
        pb::user::user::UpdateInfoResponse{info:Some(info)}.to_resp()
    }
}
