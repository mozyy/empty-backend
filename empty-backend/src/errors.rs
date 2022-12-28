use std::error::Error;

// DEFINE ERROR HERE
use crate::model::response::ResponseBody;
use axum::{http::StatusCode, response::IntoResponse};
use diesel::r2d2::PoolError;
use serde::Serialize;

#[derive(Debug)]
pub struct ServiceError<T: IntoResponse + Serialize = String> {
    pub http_status: StatusCode,
    pub body: ResponseBody<T>,
}

impl<T> std::error::Error for ServiceError<T> where T: std::fmt::Debug + IntoResponse + Serialize {}

impl<T> std::fmt::Display for ServiceError<T>
where
    T: IntoResponse + Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Service Error: {}:{}",
            self.http_status, self.body.message
        )
    }
}

impl<T> ServiceError<T>
where
    T: IntoResponse + Serialize,
{
    pub fn new(http_status: StatusCode, message: String, data: T) -> ServiceError<T> {
        ServiceError {
            http_status,
            body: ResponseBody { message, data },
        }
    }
}

impl Default for ServiceError<()> {
    fn default() -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED, String::from("服务器错误"), ())
    }
}

// impl From<PoolError> for ServiceError<PoolError> {
//     fn from(e: PoolError) -> Self {
//         ServiceError {
//             http_status: StatusCode::INTERNAL_SERVER_ERROR,
//             body: ResponseBody {
//                 message: format!("数据库连接错误: {e}"),
//                 data: e,
//             },
//         }
//     }
// }

// impl From<diesel::result::Error> for ServiceError<diesel::result::Error> {
//     fn from(e: diesel::result::Error) -> Self {
//         ServiceError {
//             http_status: StatusCode::INTERNAL_SERVER_ERROR,
//             body: ResponseBody {
//                 message: format!("数据库错误: {e}"),
//                 data: e,
//             },
//         }
//     }
// }
impl From<PoolError> for ServiceError {
    fn from(e: PoolError) -> Self {
        ServiceError {
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ResponseBody {
                message: format!("数据库连接错误: {e}"),
                data: e.to_string(),
            },
        }
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
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

impl<T> IntoResponse for ServiceError<T>
where
    T: IntoResponse + Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.http_status, self.body).into_response()
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
