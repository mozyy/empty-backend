use empty_utils::tonic::Resp;
use pb::{
    oauth_service_server::OauthService, oauth_service_server::OauthServiceServer,
    AccessTokenRequest, AccessTokenResponse, AuthorizationRequest, AuthorizationResponse,
    ClientCredentialsRequest, ClientCredentialsResponse, RefreshRequest, RefreshResponse,
    ResourceRequest, ResourceResponse,
};
use tonic::Request;

pub mod model;
pub mod schema;
pub mod state;

pub mod pb {
    tonic::include_proto!("empty.oauth.v1");
}

impl Default for OauthServiceServer<Service> {
    fn default() -> Self {
        Self::new(Service::default())
    }
}

#[derive(Default)]
struct Oauth {}
impl Oauth {
    pub fn get_oauth(&self, name: &str) -> String {
        format!("response oauth service: {name}")
    }
}
#[derive(Default)]
pub struct Service {
    oauth: Oauth,
}

// TODO: async_trait to async_fn_in_trait
// https://github.com/rust-lang/rust/issues/91611
#[tonic::async_trait]
impl OauthService for Service {
    async fn access_token(
        &self,
        request: Request<AccessTokenRequest>,
    ) -> Resp<AccessTokenResponse> {
        todo!()
    }
    async fn authorization(
        &self,
        request: Request<AuthorizationRequest>,
    ) -> Resp<AuthorizationResponse> {
        todo!()
    }
    async fn client_credentials(
        &self,
        request: Request<ClientCredentialsRequest>,
    ) -> Resp<ClientCredentialsResponse> {
        todo!()
    }
    async fn refresh(&self, request: Request<RefreshRequest>) -> Resp<RefreshResponse> {
        todo!()
    }
    async fn resource(&self, request: Request<ResourceRequest>) -> Resp<ResourceResponse> {
        todo!()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
