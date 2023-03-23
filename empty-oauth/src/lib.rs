use std::{borrow::Cow, collections::HashMap};

use empty_utils::{errors::ServiceError, tonic::Resp};
use oxide_auth::endpoint::{QueryParameter, WebRequest, WebResponse};
use pb::{
    oauth_service_server::OauthService, oauth_service_server::OauthServiceServer,
    AccessTokenRequest, AccessTokenResponse, AuthorizationRequest, AuthorizationResponse,
    ClientCredentialsRequest, ClientCredentialsResponse, RefreshRequest, RefreshResponse,
    ResourceRequest, ResourceResponse,
};
use state::OAuthState;
use tokio::sync::Mutex;
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
    state: Mutex<OAuthState>,
}

impl WebRequest for AccessTokenRequest {
    type Error = empty_utils::errors::ServiceError;

    type Response = AccessTokenResponse;

    fn query(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(String::from("response_type"), self.response_type.clone());
        hash.insert(String::from("client_id"), self.client_id.clone());
        hash.insert(String::from("state"), self.state.clone());
        hash.insert(
            String::from("redirect_uri"),
            self.redirect_uri.clone().unwrap_or_default(),
        );
        hash.insert(
            String::from("scope"),
            self.scope.clone().unwrap_or_default(),
        );
        // Ok(hash.into())
        let hash = Cow::Borrowed(&hash as &dyn QueryParameter);
        Ok(hash)
    }

    fn urlbody(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        let hash: HashMap<String, String> = HashMap::new();
        Ok(Cow::Borrowed(&hash))
    }

    fn authheader(&mut self) -> Result<Option<Cow<str>>, Self::Error> {
        Ok(None)
    }
}
impl WebResponse for AccessTokenResponse {
    type Error = ServiceError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn redirect(&mut self, url: oxide_auth::frontends::dev::Url) -> Result<(), Self::Error> {
        todo!()
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn unauthorized(&mut self, header_value: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        todo!()
    }
}
// TODO: async_trait to async_fn_in_trait
// https://github.com/rust-lang/rust/issues/91611
#[tonic::async_trait]
impl OauthService for Service {
    async fn access_token(
        &self,
        request: Request<AccessTokenRequest>,
    ) -> Resp<AccessTokenResponse> {
        let mut state = self.state.lock().await;

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
