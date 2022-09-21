use actix_web::{get, post, put, web, HttpResponse, Responder, Scope};
use diesel::Queryable;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(Default, OpenApi)]
#[openapi(
    paths(get, post, id_get, id_post, id_put,),
    components(schemas(Question, Answer))
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
    }
}

#[derive(Default, Queryable, Serialize, ToSchema)]
pub struct Question {
    id: i32,
    content: String,
    answers: Vec<Answer>,
}

#[derive(Serialize, ToSchema, Default)]
pub struct Answer {
    id: i32,
    content: String,
}

#[utoipa::path(context_path = "/questions"
,responses(
    (status=200,description="ok",body=[Question])
))]
#[get("")]
async fn get() -> HttpResponse {
    let questions = vec![Question::default()];
    HttpResponse::Ok().json(questions)
}

#[utoipa::path(context_path = "/questions",responses(
    (status=200,description="ok",body=[Question])
))]
#[post("")]
async fn post() -> HttpResponse {
    todo!()
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
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
}

#[utoipa::path(context_path = "/questions")]
#[put("/{id}")]
async fn id_put() -> HttpResponse {
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
}
