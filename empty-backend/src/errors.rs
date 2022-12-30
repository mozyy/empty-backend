use std::error::Error;

// DEFINE ERROR HERE
use crate::model::response::ResponseBody;
use axum::{http::StatusCode, response::IntoResponse};
use diesel::r2d2::PoolError;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{1}")]
    Custom(StatusCode, String),
    #[error("数据库连接错误")]
    PoolError(#[from] diesel::r2d2::PoolError),
    #[error("数据库错误")]
    DieselError(#[from] diesel::result::Error),
    #[error("oauth错误")]
    AuthError(#[from] oxide_auth_axum::WebError),
}

impl From<oxide_auth::frontends::simple::endpoint::Error<oxide_auth_axum::OAuthRequest>>
    for ServiceError
{
    fn from(
        e: oxide_auth::frontends::simple::endpoint::Error<oxide_auth_axum::OAuthRequest>,
    ) -> Self {
        e.into()
    }
}

impl From<oxide_auth::endpoint::OAuthError> for ServiceError {
    fn from(e: oxide_auth::endpoint::OAuthError) -> Self {
        e.into()
    }
}

impl From<axum::http::header::InvalidHeaderValue> for ServiceError {
    fn from(e: axum::http::header::InvalidHeaderValue) -> Self {
        e.into()
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            ServiceError::Custom(code, message) => return (code, message).into_response(),
            ServiceError::PoolError(e) => e.to_string(),
            ServiceError::DieselError(e) => e.to_string(),
            ServiceError::AuthError(e) => e.to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}
