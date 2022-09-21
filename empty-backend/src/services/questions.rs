use actix_web::{get, post, put, web, HttpResponse, Responder, Scope};
use diesel::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

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

#[derive(Serialize)]
pub struct Answer {
    id: i32,
    content: String,
}

#[utoipa::path(context_path = "/questions")]
#[get("")]
async fn get() -> String {
    // let questios = vec![Question::default()];
    "questios".to_string()
}

#[utoipa::path(context_path = "/questions")]
#[post("")]
async fn post() -> HttpResponse {
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
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
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
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
