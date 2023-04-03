use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use oxide_auth::endpoint::{
    AccessTokenFlow, AuthorizationFlow, ClientCredentialsFlow, QueryParameter, RefreshFlow,
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse};
use serde::Serialize;

use crate::service::Service;

pub async fn index() -> impl IntoResponse {
    "hello wrold"
}

pub async fn authorize(State(state): State<Service>, req: OAuthRequest) -> Result<OAuthResponse> {
    let endpoint = state.endpoint();
    let res = AuthorizationFlow::prepare(endpoint)
        .map_err(|_e| "error")?
        .execute(req)
        .map_err(|_e| "error")?;
    Ok(res)
}
#[derive(Serialize)]
struct GrantType {
    grant_type: String,
}
pub async fn token(State(state): State<Service>, req: OAuthRequest) -> Result<OAuthResponse> {
    let endpoint = state.endpoint();
    let grant_type = req.body().and_then(|body| body.unique_value("grant_type"));
    if grant_type.as_deref() == Some("client_credentials") {
        let res = ClientCredentialsFlow::prepare(endpoint)
            .map_err(|_e| "error")?
            .execute(req)
            .map_err(|_e| "error")?;
        return Ok(res);
    }
    let res = AccessTokenFlow::prepare(endpoint)
        .map_err(|_e| "error")?
        .execute(req)
        .map_err(|_e| "error")?;
    Ok(res)
}
pub async fn refresh(State(state): State<Service>, req: OAuthRequest) -> Result<OAuthResponse> {
    let endpoint = state.endpoint();
    let res = RefreshFlow::prepare(endpoint)
        .map_err(|_e| "error")?
        .execute(req)
        .map_err(|_e| "error")?;
    Ok(res)
}
