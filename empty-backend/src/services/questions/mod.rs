use crate::database::DbPool;
use actix_web::{get, post, put, web, Error as ActixError, HttpResponse, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

mod model;
mod service;

pub use model::{Answer, NewAnswer, NewQuestion, NewQuestionAnswerNth, Question};

#[derive(Default, OpenApi)]
#[openapi(
    paths(get, post, id_get, id_post, id_put,),
    components(schemas(
        NewAnswer,
        NewQuestion,
        Answer,
        Question,
        PostParams,
        NewQuestionAnswerNth
    ))
)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

impl super::Server for Server {
    fn service(&self) -> Scope {
        web::scope("/questions")
            .service(get)
            .service(post)
            .service(id_get)
            .service(id_post)
            .service(id_put)
    }
}

#[utoipa::path(context_path = "/questions"
,responses(
    (status=200,description="ok",body=[Question])
))]
#[get("")]
async fn get(pool: web::Data<DbPool>) -> Result<HttpResponse, ActixError> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || {
        let mut conn = pool.get()?;
        service::select_questions(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(ToSchema, Deserialize)]
struct PostParams {
    question: NewQuestion,
    answer: Vec<NewAnswer>,
    answer_nth: NewQuestionAnswerNth,
}

#[utoipa::path(context_path = "/questions",request_body = PostParams, responses(
    (status=200,description="ok",body=i32)
))]
#[post("")]
async fn post(
    pool: web::Data<DbPool>,
    question: web::Json<PostParams>,
) -> Result<HttpResponse, ActixError> {
    let question = question.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let new_question_id = web::block(move || {
        let mut conn = pool.get()?;
        service::insert_question(
            &mut conn,
            (&question.question, &question.answer, &question.answer_nth),
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(new_question_id))
}

#[utoipa::path(
    context_path = "/questions",
    params(
         ("id", description = "Pet id"),
    ),
    responses(
    (status = 200, description = "Pet found from database",body = Question)
    ))]
#[get("/{id}")]
async fn id_get() -> HttpResponse {
    todo!()
}

#[utoipa::path(context_path = "/questions")]
#[post("/{id}")]
async fn id_post() -> HttpResponse {
    todo!()
}

#[utoipa::path(context_path = "/questions")]
#[put("/{id}")]
async fn id_put() -> HttpResponse {
    todo!()
}
