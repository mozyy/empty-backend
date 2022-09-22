use std::error::Error;

use crate::schema::answers::{self, dsl::*};
use crate::schema::questions::{self, dsl::*};
use crate::DbPool;
use actix_web::{get, post, put, web, Error as ActixError, HttpResponse, Responder, Scope};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
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
            .service(id_post)
            .service(id_put)
    }
}

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Question {
    pub id: Option<i32>,
    pub content: String,
    pub desc: String,
    pub answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Queryable, Insertable, Associations, Clone)]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub id: Option<i32>,
    pub question_id: Option<i32>,
    pub content: String,
}

#[utoipa::path(context_path = "/questions"
,responses(
    (status=200,description="ok",body=[Question])
))]
#[get("")]
async fn get() -> HttpResponse {
    let values = vec![Question::default()];
    HttpResponse::Ok().json(values)
}

#[utoipa::path(context_path = "/questions",request_body = Question, responses(
    (status=200,description="ok",body=i32)
))]
#[post("")]
async fn post(
    pool: web::Data<DbPool>,
    question: web::Json<Question>,
) -> Result<HttpResponse, ActixError> {
    let question = question.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let new_question_id = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_question(&mut conn, &question)
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
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
}

#[utoipa::path(context_path = "/questions")]
#[put("/{id}")]
async fn id_put() -> HttpResponse {
    let questios = vec![Question::default()];
    HttpResponse::Ok().json(questios)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_question(
    conn: &mut PgConnection,
    question: &Question, // prevent collision with `name` column imported inside the function
) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::questions::dsl::*;
    Ok(
        conn.transaction::<_, diesel::result::Error, _>(move |conn| {
            let new_question_id = diesel::insert_into(questions)
                .values((content.eq(&question.content), desc.eq(&question.desc)))
                .returning(id)
                .get_result::<i32>(conn)?;
            diesel::insert_into(answers)
                .values(
                    &question
                        .answers
                        .clone()
                        .into_iter()
                        .map(|answer| {
                            (
                                crate::schema::answers::columns::question_id.eq(new_question_id),
                                crate::schema::answers::columns::content.eq(answer.content),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .execute(conn)?;
            Ok(new_question_id)
        })?,
    )
}
