use async_trait::async_trait;
use empty_utils::tonic::Resp;
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation},
    frontends::simple::endpoint::{FnSolicitor, Vacant},
};
use oxide_auth_async::endpoint::{
    access_token::AccessTokenFlow, authorization::AuthorizationFlow, resource::ResourceFlow,
};
use tonic::{Request, Response};

use crate::{
    model::oauth::{
        endpoint::Endpoint,
        grpc::{error::OAuthError, request::OAuthRequest, response::OAuthResponse},
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
                fn map_err<E: std::error::Error>(err: E) -> OwnerConsent<OAuthResponse> {
                    OwnerConsent::Error(OAuthError(tonic::Status::unknown(err.to_string())))
                }

                let pre_g = &solicitation.pre_grant();
                let state = &solicitation.state();
                log::debug!("PreGrant: {:?}, {:?}", pre_g, state);

                let _client_id = &solicitation.pre_grant().client_id;

                // // let mut response = OAuthResponse::default();
                // // response
                // //     .redirect("http://www.com".parse().unwrap())
                // //     .unwrap();
                // // OwnerConsent::InProgress(response)
                OwnerConsent::Authorized("abc".into())
            },
        ));

        let p = AuthorizationFlow::prepare(endpoint)
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
            .execute(request.into())
            .await
            .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        Ok(Response::new(pb::AuthorizeResponse {
            code: "todo".into(),
        }))
    }
    async fn token(&self, request: Request<pb::TokenRequest>) -> Resp<pb::TokenResponse> {
        let p =
            AccessTokenFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
                .execute(OAuthRequest::from(request))
                .await
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?;
        todo!();
    }
    async fn resource(&self, request: Request<pb::ResourceRequest>) -> Resp<pb::ResourceResponse> {
        let pb::ResourceRequest { uri, auth } = request.into_inner();
        let res =
            ResourceFlow::<Endpoint<'_, Vacant>, OAuthRequest>::prepare(self.endpoint().await)
                .map_err(|e| tonic::Status::unauthenticated(e.0.to_string()))?
                .execute(OAuthRequest::with_auth(auth))
                .await;                ;
        let res = match res{
            Ok(r) => r,
            Err(e) => match e {
                Ok(r) => {
                    log::warn!("{:?}",r); 
                    return Err(tonic::Status::unauthenticated("r.into()"))
                },
                Err(e) => return Err(tonic::Status::unauthenticated(e.0.to_string())),
            },
        };
        Ok(Response::new(res.into()))
    }
}
