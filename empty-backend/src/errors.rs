// DEFINE ERROR HERE
use crate::model::response::ResponseBody;
use axum::{http::StatusCode, response::IntoResponse};
use diesel::r2d2::{self, PoolError};

#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl std::error::Error for ServiceError {}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Service Error: {}:{}",
            self.http_status, self.body.message
        )
    }
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }
}

impl From<PoolError> for ServiceError {
    fn from(e: PoolError) -> Self {
        ServiceError {
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ResponseBody {
                message: format!("数据库错误: {e}"),
                data: e.to_string(),
            },
        }
    }
}

impl From<String> for ServiceError {
    fn from(e: String) -> Self {
        ServiceError {
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ResponseBody {
                message: format!("内部错误: {e}"),
                data: e,
            },
        }
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
        ServiceError {
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ResponseBody {
                message: format!("内部diesel错误: {e}"),
                data: e.to_string(),
            },
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum MyError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader { expected: String, found: String },
//     #[error("unknown data store error")]
//     Unknown,
// }
