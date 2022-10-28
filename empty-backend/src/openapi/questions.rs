use crate::api::questions;
use crate::model::questions::{
    Answer, NewAnswer, NewQuestion, Question, QuestionReq, QuestionResp,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(questions::index_get, questions::index_post),
    components(schemas(NewAnswer, NewQuestion, Question, Answer, QuestionReq, QuestionResp))
)]
pub struct ApiDoc {}
