use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};

use crate::{database::DbPool, model::response::ResponseBody, service::questions};

#[utoipa::path(
  context_path = "/questions",
  get,path="/",
  responses(
      (status=200,description="ok",body=[GetResp])
  )
)]
pub async fn index_get(State(pool): State<DbPool>) -> impl IntoResponse {
    match questions::get(pool) {
        Ok(res) => Json(ResponseBody::new("success", res)),
        Err(_) => todo!(),
    }
    // service::select_questions(&mut conn)
    //     .await?
    //     .map_err(actix_web::error::ErrorInternalServerError)?;
    // let res: Vec<GetResp> = res
    //     .into_iter()
    //     .map(|(question, answers, answer)| GetResp {
    //         question,
    //         answers,
    //         answer,
    //     })
    //     .collect();
    // Ok(HttpResponse::Ok().json(res))
}
