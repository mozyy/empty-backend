use std::collections::HashMap;

use ::diesel::expression::is_aggregate::No;
use async_trait::async_trait;
use empty_utils::{errors::ServiceResult, tonic::Resp};
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation, WebResponse},
    frontends::simple::endpoint::{FnSolicitor, Vacant},
};
use oxide_auth_async::endpoint::{
    access_token::AccessTokenFlow, authorization::AuthorizationFlow, resource::ResourceFlow,
};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::{
    model::oauth::{
        diesel,
        endpoint::Endpoint,
        grpc::{
            error::OAuthError,
            request::{Auth, OAuthRequest},
            response::{OAuthResponse, ResponseStatus},
        },
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
        let endpoint = self.endpoint().await;
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
        let _p =
            AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
                .execute(OAuthRequest::from(request))
                .await
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        todo!();
    }
    async fn resource(&self, request: Request<pb::ResourceRequest>) -> Resp<pb::ResourceResponse> {
        let pb::ResourceRequest { uri: _, auth } = request.into_inner();
        let res =
            ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
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
        let id = id.parse().unwrap();
        let user = diesel::query_list_by_id(&mut conn, id).await?;
        let req = OAuthRequest::default_authorize();
        let req = self.authorize_by_id(id, req).await?;

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
        let user = diesel::insert(&mut conn).await?;
        let id = user.id.parse().unwrap();
        let req = OAuthRequest::default_authorize();
        let req = self.authorize_by_id(id, req).await?;

        let token = req
            .body
            .map(|body| serde_json::from_str(body.as_str()).unwrap());

        Ok(Response::new(pb::RegisterResponse {
            user: Some(user),
            token,
        }))
    }
}

impl State {
    async fn authorize_by_id(
        &self,
        user_id: Uuid,
        request: OAuthRequest,
    ) -> ServiceResult<OAuthResponse> {
        let endpoint = self.endpoint().await;
        let endpoint =
            endpoint.with_solicitor(FnSolicitor(|_: &mut OAuthRequest, _: Solicitation| {
                OwnerConsent::Authorized(user_id.to_string())
            }));

        let resp = AuthorizationFlow::prepare(endpoint)
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
            .execute(request)
            .await
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        let mut code = if let ResponseStatus::REDIRECT(url) = resp.status {
            url.query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new)
        } else {
            HashMap::new()
        };
        code.insert("grant_type".into(), "authorization_code".into());
        // TODO: from query
        code.insert("client_id".into(), "zuoyin".into());
        code.insert(
            "redirect_uri".into(),
            "http://localhost:8021/endpoint".into(),
        );
        let res =
            AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
                .execute(OAuthRequest {
                    auth: Auth(None),
                    query: code.clone(),
                    body: code,
                })
                .await
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        Ok(res)
    }
    pub async fn check_resource(
        &mut self,
        request: pb::ResourceRequest,
    ) -> Resp<pb::ResourceResponse> {
        let pb::ResourceRequest { uri: _, auth } = request;
        let res =
            ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
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
}
