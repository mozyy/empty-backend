use std::{borrow::Cow, collections::HashMap};

use oxide_auth::endpoint::WebRequest;

use proto::pb;

use super::{error::OAuthError, response::OAuthResponse};

#[derive(Default)]
pub struct Auth(pub Option<String>);

impl<T> From<&tonic::Request<T>> for Auth {
    fn from(value: &tonic::Request<T>) -> Self {
        let auth = match value.metadata().get("authorization") {
            Some(a) => a.to_str().ok().map(|a| a.to_owned()),
            None => None,
        };
        Self(auth)
    }
}

#[derive(Default)]
pub struct OAuthRequest {
    pub auth: Auth,
    pub query: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

impl WebRequest for OAuthRequest {
    type Error = OAuthError;

    type Response = OAuthResponse;

    fn query(
        &mut self,
    ) -> Result<std::borrow::Cow<dyn oxide_auth::endpoint::QueryParameter + 'static>, Self::Error>
    {
        Ok(std::borrow::Cow::Borrowed(&self.query))
    }
    fn urlbody(
        &mut self,
    ) -> Result<std::borrow::Cow<dyn oxide_auth::endpoint::QueryParameter + 'static>, Self::Error>
    {
        Ok(std::borrow::Cow::Borrowed(&self.body))
    }

    fn authheader(&mut self) -> Result<Option<std::borrow::Cow<str>>, Self::Error> {
        Ok(self.auth.0.as_deref().map(Cow::Borrowed))
    }
}

impl From<tonic::Request<pb::oauth::oauth::AuthorizeRequest>> for OAuthRequest {
    fn from(value: tonic::Request<pb::oauth::oauth::AuthorizeRequest>) -> Self {
        let auth = (&value).into();
        let req = value.into_inner();
        let mut query = HashMap::new();
        query.insert(String::from("client_id"), req.client_id);
        query.insert(String::from("response_type"), req.response_type);
        if let Some(redirect_uri) = req.redirect_uri {
            query.insert(String::from("redirect_uri"), redirect_uri);
        }

        Self {
            auth,
            query,
            body: Default::default(),
        }
    }
}
impl From<tonic::Request<pb::oauth::oauth::TokenRequest>> for OAuthRequest {
    fn from(value: tonic::Request<pb::oauth::oauth::TokenRequest>) -> Self {
        let auth = (&value).into();
        let req = value.into_inner();
        let mut query = HashMap::new();
        query.insert(String::from("code"), req.code);

        Self {
            auth,
            query,
            body: Default::default(),
        }
    }
}
impl OAuthRequest {
    pub fn with_auth(&self, auth: String) -> Self {
        Self {
            auth: Auth(Some(auth)),
            query: self.query.clone(),
            body: self.body.clone(),
        }
    }
    pub fn default_authorize() -> Self {
        let mut state = Self::default();
        let mut query = HashMap::new();
        query.insert(String::from("client_id"), "zuoyin".into());
        query.insert(String::from("response_type"), "code".into());
        // query.insert(String::from("redirect_uri"), "http://localhost:8021/endpoint".into());
        state.query = query;
        state
    }
    pub fn default_with_client(client: &pb::oauth::oauth::Client) -> Self {
        let mut state = Self::default();
        let mut query = HashMap::new();
        query.insert(String::from("client_id"), client.id.to_owned());
        query.insert(String::from("response_type"), "code".into());
        // query.insert(String::from("redirect_uri"), "http://localhost:8021/endpoint".into());
        state.query = query;
        state
    }
}
