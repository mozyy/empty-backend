use axum::extract::{Query, State};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

use crate::{database::DbPool, model::oauth::OAuthState};

pub async fn get_authorize(
    req: OAuthRequest,
    State(pool): State<DbPool>,
) -> Result<OAuthResponse, WebError> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    let mut flow = OAuthState::new().endpoint.authorization_flow();
    let r = flow.execute(req)?;
    Ok(r)
}
