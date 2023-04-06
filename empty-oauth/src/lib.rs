use std::str::FromStr;

use chrono::NaiveDateTime;
use empty_utils::{errors::ServiceError, tonic::Resp};

use pb::{
    oauth_server::Oauth as OauthService, oauth_server::OauthServer, PasswordRequest, TokenResponse,
};
use state::OAuthState;
use tokio::sync::Mutex;

pub mod model;
pub mod schema;
pub mod state;

pub mod pb {
    tonic::include_proto!("empty.oauth.v1");
}

impl Default for OauthServer<Service> {
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

// TODO: async_trait to async_fn_in_trait
// https://github.com/rust-lang/rust/issues/91611
#[tonic::async_trait]
impl OauthService for Service {
    async fn password(&self, request: tonic::Request<PasswordRequest>) -> Resp<TokenResponse> {
        let _client = Client::parse(&request);
        let _state = self.state.lock().await;
        todo!()
    }
}

struct Client {
    client_id: String,
    client_screct: Option<String>,
}
impl FromStr for Client {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut auth = s.split(':');
        let client_id = auth
            .next()
            .ok_or_else(|| tonic::Status::unauthenticated("no client"))?;
        let client_screct = auth
            .next()
            .ok_or_else(|| tonic::Status::unauthenticated("no screct"))?;
        let client_screct = if client_screct.is_empty() {
            None
        } else {
            Some(client_screct.to_owned())
        };
        let timestamp = auth
            .next()
            .ok_or_else(|| tonic::Status::unauthenticated("no timestamp"))?;
        let timestamp = timestamp
            .parse::<i64>()
            .map_err(|_e| tonic::Status::unauthenticated("no timestamp2"))?;

        let _time = NaiveDateTime::from_timestamp_opt(
            timestamp / 1000,
            ((timestamp % 1000) * 1_000_000) as u32,
        )
        .ok_or_else(|| tonic::Status::unauthenticated("no timestamp"))?;
        // TODO: check timestamp
        // TODO: check sha256
        Ok(Self {
            client_id: client_id.to_owned(),
            client_screct,
        })
    }
}
impl Client {
    pub fn parse<T>(request: &tonic::Request<T>) -> ServiceResult<Self> {
        let auth = request.metadata().get("Authorization");
        let auth = match auth {
            Some(auth) => auth
                .to_str()
                .map_err(|_e| tonic::Status::unauthenticated("auth err"))?,
            None => return Err(tonic::Status::unauthenticated("no auth").into()),
        };
        let client = auth.parse::<Client>()?;
        Ok(client)
    }
    pub async fn check(&self, _state: OAuthState) -> ServiceResult<()> {
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
