use async_trait::async_trait;
use empty_utils::{errors::Error, tonic::Resp};
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation, WebResponse},
    frontends::simple::endpoint::{FnSolicitor, Vacant},
};
use oxide_auth_async::endpoint::{
    access_token::AccessTokenFlow, authorization::AuthorizationFlow, resource::ResourceFlow,
};
use tonic::{Request, Response};

use crate::{
    model::oauth::{
        diesel,
        endpoint::Endpoint,
        grpc::{request::OAuthRequest, response::OAuthResponse},
    },
    pb::oauth as pb,
};

use super::state::State;

#[async_trait]
impl pb::o_auth_service_server::OAuthService for State {
    async fn authorize(
        &self,
        request: Request<pb::AuthorizeRequest>,
    ) -> Resp<pb::AuthorizeResponse> {
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

        let _p = AuthorizationFlow::prepare(endpoint)
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
            .execute(request.into())
            .await
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        Ok(Response::new(pb::AuthorizeResponse {
            code: "todo".into(),
        }))
    }
    async fn token(&self, request: Request<pb::TokenRequest>) -> Resp<pb::TokenResponse> {
        let _p = AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(
            self.endpoint_state.endpoint().await,
        )
        .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
        .execute(OAuthRequest::from(request))
        .await
        .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        todo!();
    }
    async fn resource(&self, request: Request<pb::ResourceRequest>) -> Resp<pb::ResourceResponse> {
        let pb::ResourceRequest { uri: _, auth } = request.into_inner();
        let res = ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(
            self.endpoint_state.endpoint().await,
        )
        .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
        .execute(OAuthRequest::default().with_auth(auth))
        .await;
        let res = match res {
            Ok(r) => r,
            Err(e) => match e {
                Ok(r) => {
                    log::warn!("{:?}", r);
                    return Err(tonic::Status::unauthenticated("r.into()"));
                }
                Err(e) => return Err(tonic::Status::unauthenticated(e.0.to_string())),
            },
        };
        Ok(Response::new(res.into()))
    }
    async fn login(&self, request: Request<pb::LoginRequest>) -> Resp<pb::LoginResponse> {
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
        Ok(Response::new(pb::LoginResponse {
            user: Some(user),
            token,
        }))
    }
    async fn register(&self, _request: Request<pb::RegisterRequest>) -> Resp<pb::RegisterResponse> {
        let mut conn = self.db.get_conn()?;
        let user = diesel::user_insert(&mut conn).await?;
        let id = user.id.parse().map_err(Error::other)?;
        let client = diesel::client_query_by_name(&mut conn, "zuoyinyun".into()).await?;
        let req = OAuthRequest::default_with_client(&client);
        let req = self.endpoint_state.authorize_by_id(id, req, client).await?;

        let token = req
            .body
            .map(|body| serde_json::from_str(body.as_str()).unwrap());

        Ok(Response::new(pb::RegisterResponse {
            user: Some(user),
            token,
        }))
    }
    async fn client_list(
        &self,
        _request: Request<pb::ClientListRequest>,
    ) -> Resp<pb::ClientListResponse> {
        let mut conn = self.db.get_conn()?;
        let clients = diesel::client_query_all(&mut conn).await?;
        Ok(Response::new(pb::ClientListResponse { clients }))
    }
    async fn client_create(
        &self,
        request: Request<pb::ClientCreateRequest>,
    ) -> Resp<pb::ClientCreateResponse> {
        let client = request.into_inner().client.ok_or_else(Error::invalid)?;
        let mut conn = self.db.get_conn()?;
        let client = diesel::client_insert(&mut conn, client).await?;
        Ok(Response::new(pb::ClientCreateResponse {
            client: Some(client),
        }))
    }
    async fn config_list(
        &self,
        _request: Request<pb::ConfigListRequest>,
    ) -> Resp<pb::ConfigListResponse> {
        let mut conn = self.db.get_conn()?;
        let configs = diesel::config_query_all(&mut conn).await?;
        Ok(Response::new(pb::ConfigListResponse { configs }))
    }
    async fn config_create(
        &self,
        request: Request<pb::ConfigCreateRequest>,
    ) -> Resp<pb::ConfigCreateResponse> {
        let config = request.into_inner().config.ok_or_else(Error::invalid)?;
        let mut conn = self.db.get_conn()?;
        let config = diesel::config_insert(&mut conn, config).await?;
        Ok(Response::new(pb::ConfigCreateResponse {
            config: Some(config),
        }))
    }
}
