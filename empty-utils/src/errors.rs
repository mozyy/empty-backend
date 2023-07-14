// DEFINE ERROR HERE

use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("[axum] error:{1}")]
    AxumStatus(StatusCode, String),
    #[error("[diesel pool] error")]
    PoolError(#[from] diesel::r2d2::PoolError),
    #[error("[diesel] error")]
    DieselError(#[from] diesel::result::Error),
    #[error("[oxide] error")]
    AuthError(#[from] oxide_auth_axum::WebError),
    #[error("[tonic] error")]
    StatusError(#[from] tonic::Status),
    #[error("[tonic] error")]
    TransportError(#[from] tonic::transport::Error),

    #[error("[custom] error:{0}")]
    String(String),
    #[error("[unknown] error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ServiceError {
    pub fn params_loss() -> Self {
        Self::StatusError(tonic::Status::invalid_argument("invalid_argument"))
    }
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
            ServiceError::TransportError(e) => e.to_string(),
            ServiceError::Unknown => "unknown".to_string(),
            ServiceError::Other(e) => e.to_string(),
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
            ServiceError::TransportError(e) => e.to_string(),
            ServiceError::Unknown => "unknown".to_string(),
            ServiceError::Other(e) => e.to_string(),
        };
        tonic::Status::unknown(message)
    }
}

pub type ServiceResult<T = ()> = Result<T, ServiceError>;
