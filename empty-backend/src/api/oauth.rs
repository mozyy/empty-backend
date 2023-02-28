use axum::extract::{FromRequest, State};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

use super::Result;
use crate::model::oauth::OAuthState;

pub async fn get_authorize(req: OAuthRequest) -> Result<OAuthResponse> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    log::info!("get auth");
    let r = flow.execute(req)?;
    Ok(r)
}

pub async fn get_clients(req: OAuthRequest) -> Result<OAuthResponse> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    log::info!("infoooooooo!3");
    let r = flow.execute(req)?;
    Ok(r)
}

pub struct Req {}

impl<S, B> FromRequest<S, B> for Req {
    type Rejection = WebError;

    fn from_request<'life0, 'async_trait>(
        req: axum::http::Request<B>,
        state: &'life0 S,
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
