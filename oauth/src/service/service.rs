use async_trait::async_trait;
use empty_utils::{
    errors::{Error, ErrorConvert},
    tonic::Resp,
};
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation, WebResponse},
    frontends::simple::endpoint::{FnSolicitor, Vacant},
};
use oxide_auth_async::endpoint::{
    access_token::AccessTokenFlow, authorization::AuthorizationFlow, resource::ResourceFlow,
};
use tonic::{Request, Response};

use crate::model::{
    diesel,
    endpoint::Endpoint,
    grpc::{request::OAuthRequest, response::OAuthResponse},
};
use proto::pb;

use super::state::State;

#[async_trait]
impl pb::oauth::oauth::o_auth_service_server::OAuthService for State {
    async fn authorize(
        &self,
        request: Request<pb::oauth::oauth::AuthorizeRequest>,
    ) -> Resp<pb::oauth::oauth::AuthorizeResponse> {
        let endpoint = self.endpoint_state.endpoint().await;
        let endpoint = endpoint.with_solicitor(FnSolicitor(
            |_: &mut OAuthRequest, solicitation: Solicitation| {
                let pre_g = &solicitation.pre_grant();
                let state = &solicitation.state();
                log::debug!("PreGrant: {:?}, {:?}", pre_g, state);

                let _client_id = &solicitation.pre_grant().client_id;

                let mut response = OAuthResponse::default();
                response
                    .redirect("http://www.com".parse().unwrap())
                    .unwrap();
                OwnerConsent::InProgress(response)
                // OwnerConsent::Authorized("abc".into())
            },
        ));

        let _p = AuthorizationFlow::prepare(endpoint)?
            .execute(request.into())
            .await?;
        Ok(Response::new(pb::oauth::oauth::AuthorizeResponse {
            code: "todo".into(),
        }))
    }
    async fn token(
        &self,
        request: Request<pb::oauth::oauth::TokenRequest>,
    ) -> Resp<pb::oauth::oauth::TokenResponse> {
        let _p = AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(
            self.endpoint_state.endpoint().await,
        )?
        .execute(OAuthRequest::from(request))
        .await?;
        todo!();
    }
    async fn resource(
        &self,
        request: Request<pb::oauth::oauth::ResourceRequest>,
    ) -> Resp<pb::oauth::oauth::ResourceResponse> {
        let pb::oauth::oauth::ResourceRequest { uri: _, auth } = request.into_inner();
        let res = ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(
            self.endpoint_state.endpoint().await,
        )?
        .execute(OAuthRequest::default().with_auth(auth))
        .await;
        let res = match res {
            Ok(r) => r,
            Err(e) => match e {
                Ok(r) => {
                    log::warn!("{:?}", r);
                    return Err(tonic::Status::unauthenticated("r.into()"));
                }
                Err(e) => return Err(e.into()),
            },
        };
        Ok(Response::new(res.into()))
    }
    async fn login(
        &self,
        request: Request<pb::oauth::oauth::LoginRequest>,
    ) -> Resp<pb::oauth::oauth::LoginResponse> {
        let mut conn = self.db.get_conn()?;
        let id = request.into_inner().user_id;
        let id = id.parse().map_err(Error::other)?;
        let user = diesel::user_query_by_id(&mut conn, id).await?;
        let client = diesel::client_query_by_name(&mut conn, "zuoyinyun".into()).await?;
        let req = OAuthRequest::default_with_client(&client);
        let req = self.endpoint_state.authorize_by_id(id, req, client).await?;

        let token = req
            .body
            .map(|body| serde_json::from_str(body.as_str()).unwrap());
        Ok(Response::new(pb::oauth::oauth::LoginResponse {
            user: Some(user),
            token,
        }))
    }
    async fn register(
        &self,
        _request: Request<pb::oauth::oauth::RegisterRequest>,
    ) -> Resp<pb::oauth::oauth::RegisterResponse> {
        let mut conn = self.db.get_conn()?;
        let user = diesel::user_insert(&mut conn).await?;
        let id = user.id.parse().map_err(Error::other)?;
        let client = diesel::client_query_by_name(&mut conn, "zuoyinyun".into()).await?;
        let req = OAuthRequest::default_with_client(&client);
        let req = self.endpoint_state.authorize_by_id(id, req, client).await?;

        let token = req
            .body
            .map(|body| serde_json::from_str(body.as_str()).unwrap());

        Ok(Response::new(pb::oauth::oauth::RegisterResponse {
            user: Some(user),
            token,
        }))
    }
    async fn client_list(
        &self,
        _request: Request<pb::oauth::oauth::ClientListRequest>,
    ) -> Resp<pb::oauth::oauth::ClientListResponse> {
        let mut conn = self.db.get_conn()?;
        let clients = diesel::client_query_all(&mut conn).await?;
        Ok(Response::new(pb::oauth::oauth::ClientListResponse {
            clients,
        }))
    }
    async fn client_create(
        &self,
        request: Request<pb::oauth::oauth::ClientCreateRequest>,
    ) -> Resp<pb::oauth::oauth::ClientCreateResponse> {
        let client = request.into_inner().client.ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let client = diesel::client_insert(&mut conn, client).await?;
        Ok(Response::new(pb::oauth::oauth::ClientCreateResponse {
            client: Some(client),
        }))
    }
    async fn config_list(
        &self,
        _request: Request<pb::oauth::oauth::ConfigListRequest>,
    ) -> Resp<pb::oauth::oauth::ConfigListResponse> {
        let mut conn = self.db.get_conn()?;
        let configs = diesel::config_query_all(&mut conn).await?;
        Ok(Response::new(pb::oauth::oauth::ConfigListResponse {
            configs,
        }))
    }
    async fn config_create(
        &self,
        request: Request<pb::oauth::oauth::ConfigCreateRequest>,
    ) -> Resp<pb::oauth::oauth::ConfigCreateResponse> {
        let config = request.into_inner().config.ok_or_invalid()?;
        let mut conn = self.db.get_conn()?;
        let config = diesel::config_insert(&mut conn, config).await?;
        Ok(Response::new(pb::oauth::oauth::ConfigCreateResponse {
            config: Some(config),
        }))
    }
}
