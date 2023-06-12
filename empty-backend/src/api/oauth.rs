use axum::extract::{FromRequest};
use empty_utils::errors::{ServiceResult, ServiceError};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

use crate::{model::oauth::OAuthState};

pub async fn get_authorize(req: OAuthRequest) -> ServiceResult<OAuthResponse> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    log::info!("get auth");
    let r = flow.execute(req).map_err(|_|ServiceError::String(String::from("oauth")))?;
    Ok(r)
}

pub async fn get_clients(req: OAuthRequest) -> ServiceResult<OAuthResponse> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    log::info!("infoooooooo!3");
    let r = flow.execute(req).map_err(|_|ServiceError::String(String::from("oauth")))?;
    Ok(r)
}

pub struct Req {}

impl<S, B> FromRequest<S, B> for Req {
    type Rejection = WebError;

    fn from_request<'life0, 'async_trait>(
        _req: axum::http::Request<B>,
        _state: &'life0 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = std::result::Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
