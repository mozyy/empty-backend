// DEFINE ERROR HERE

use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("axum error:{1}")]
    AxumStatus(StatusCode, String),
    #[error("diesel pool error")]
    PoolError(#[from] diesel::r2d2::PoolError),
    #[error("diesel error")]
    DieselError(#[from] diesel::result::Error),
    #[error("oauth error")]
    AuthError(#[from] oxide_auth_axum::WebError),
    #[error("tonic error")]
    StatusError(#[from] tonic::Status),
    #[error("string error:{0}")]
    String(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            ServiceError::AxumStatus(code, message) => return (code, message).into_response(),
            ServiceError::PoolError(e) => e.to_string(),
            ServiceError::DieselError(e) => e.to_string(),
            ServiceError::AuthError(e) => e.to_string(),
            ServiceError::StatusError(e) => e.to_string(),
            ServiceError::String(e) => e,
        };
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

impl From<ServiceError> for tonic::Status {
    fn from(value: ServiceError) -> Self {
        let message = match value {
            ServiceError::AxumStatus(_, message) => message,
            ServiceError::PoolError(e) => e.to_string(),
            ServiceError::DieselError(e) => e.to_string(),
            ServiceError::AuthError(e) => e.to_string(),
            ServiceError::StatusError(e) => return e,
            ServiceError::String(e) => e,
        };
        tonic::Status::unknown(message)
    }
}
