// DEFINE ERROR HERE

use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

impl Error {
    pub fn invalid() -> Self {
        Self::StatusError(tonic::Status::invalid_argument("invalid_argument"))
    }
    pub fn other<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(anyhow::Error::from(e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            Error::AxumStatus(code, message) => return (code, message).into_response(),
            Error::PoolError(e) => e.to_string(),
            Error::DieselError(e) => e.to_string(),
            Error::AuthError(e) => e.to_string(),
            Error::StatusError(e) => e.to_string(),
            Error::String(e) => e,
            Error::TransportError(e) => e.to_string(),
            Error::Unknown => "unknown".to_string(),
            Error::Other(e) => e.to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        let message = match value {
            Error::AxumStatus(_, message) => message,
            Error::PoolError(e) => e.to_string(),
            Error::DieselError(e) => e.to_string(),
            Error::AuthError(e) => e.to_string(),
            Error::StatusError(e) => return e,
            Error::String(e) => e,
            Error::TransportError(e) => e.to_string(),
            Error::Unknown => "unknown".to_string(),
            Error::Other(e) => e.to_string(),
        };
        tonic::Status::unknown(message)
    }
}

pub type Result<T = (), E = Error> = core::result::Result<T, E>;
