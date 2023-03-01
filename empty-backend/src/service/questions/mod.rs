

use empty_utils::diesel::db;

use crate::{
    errors::ServiceError,
    model::questions::{Question, QuestionReq, QuestionResp},
};

// mod model;
// mod service;

// #[derive(Default, OpenApi)]
// #[openapi(
//     paths(get, post, id_get, id_post, id_put,),
//     components(schemas(
//         NewAnswer,
//         NewQuestion,
//         Answer,
//         Question,
//         PostParams,
//         NewQuestionAnswerNth,
//         QuestionAnswer,
//         GetResp,
//         Resource,
//         // Type,
//         // Route,
//     ))
// )]
// pub struct Server {}

// impl Server {
//     pub fn new() -> Self {
//         Server {}
//     }
// }

// #[derive(ToSchema, Serialize)]
// struct GetResp {
//     question: Question,
//     answers: Vec<Answer>,
//     answer: QuestionAnswer,
// }

pub fn get(pool: db::DbPool) -> Result<Vec<QuestionResp>, ServiceError> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let mut conn = pool.get()?;
    let resp = Question::select_all(&mut conn)?;
    Ok(resp)

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
pub fn post(req: &Vec<QuestionReq>, pool: db::DbPool) -> Result<Vec<i32>, ServiceError> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let mut conn = pool.get()?;
    let resp = Question::insert(req, &mut conn)?;
    Ok(resp)
}

// #[derive(ToSchema, Deserialize)]
// struct PostParams {
//     question: NewQuestion,
//     answer: Vec<NewAnswer>,
//     answer_nth: NewQuestionAnswerNth,
// }

// #[utoipa::path(context_path = "/questions",request_body = PostParams, responses(
//     (status=200,description="ok",body=i32)
// ))]
// #[post("")]
// async fn index_post(
//     pool: web::Data<Pool>,
//     question: web::Json<PostParams>,
// ) -> Result<HttpResponse, ActixError> {
//     let question = question.into_inner();

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let new_question_id = web::block(move || {
//         let mut conn = pool.get()?;
//         service::insert_question(
//             &mut conn,
//             (&question.question, &question.answer, &question.answer_nth),
//         )
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(new_question_id))
// }

// #[utoipa::path(
//     context_path = "/questions",
//     params(
//          ("id" = i32, description = "Pet id"),
//     ),
//     responses(
//     (status = 200, description = "Pet found from database",body = Question)
//     ))]
// #[get("/{id}")]
// async fn id_get() -> HttpResponse {
//     todo!()
// }

// #[utoipa::path(context_path = "/questions",params(
//     ("id" = i32, description = "Pet id"),
// ))]
// #[post("/{id}")]
// async fn id_post() -> HttpResponse {
//     todo!()
// }

// #[utoipa::path(context_path = "/questions",params(
//     ("id" = i32, description = "Pet id"),
// ))]
// #[put("/{id}")]
// async fn id_put() -> HttpResponse {
//     todo!()
// }
