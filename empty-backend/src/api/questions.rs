use axum::{extract::State, response::IntoResponse, Json};
use empty_utils::diesel::db;

use crate::{
    model::{questions::QuestionReq, response::ResponseBody},
    service::questions,
};

#[utoipa::path(
  context_path = "/questions",
  get,
  path="/",
  responses(
      (status=200,description="ok",body=[QuestionResp])
  )
)]
pub async fn index_get(State(pool): State<db::DbPool>) -> impl IntoResponse {
    match questions::get(pool) {
        Ok(res) => Json(res),
        // Ok(res) => Json(ResponseBody::new("success", res)),
        Err(err) => todo!(),
    }
}

#[utoipa::path(
  context_path = "/questions",
  post,
  path="/",
  request_body = [QuestionReq],
  responses(
    (status=200,description="ok",body=[i32])
  )
)]
pub async fn index_post(
    State(pool): State<db::DbPool>,
    Json(input): Json<Vec<QuestionReq>>,
) -> impl IntoResponse {
    match questions::post(&input, pool) {
        Ok(res) => Json(ResponseBody::new("success", res)),
        Err(err) => todo!(),
    }
}
