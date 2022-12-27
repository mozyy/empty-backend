use axum::extract::{FromRequest, State};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

use crate::{database::DbPool, model::oauth::OAuthState};

pub async fn get_authorize(req: OAuthRequest) -> Result<OAuthResponse, WebError> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    println!("get auth: ");
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
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
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
