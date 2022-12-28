use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseBody<T: Serialize> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T>
where
    T: Serialize,
{
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
    pub fn success(data: T) -> ResponseBody<T> {
        ResponseBody {
            message: "success".to_string(),
            data,
        }
    }
}

impl<T> IntoResponse for ResponseBody<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        serde_json::to_string(&self)
    }
}
