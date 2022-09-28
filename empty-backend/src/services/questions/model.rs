use crate::schema::answers;
use crate::schema::question_answers;
use crate::schema::questions;
use crate::utils::timestamp;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use empty_utils::add_orm_field;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// #[derive(Queryable, Identifiable, Serialize, ToSchema)]
// pub struct Question {
//     pub id: i32,
//     pub content: String,
//     pub desc: Option<String>,
//     #[serde(with = "timestamp")]
//     #[schema(value_type = i64)]
//     pub created_at: NaiveDateTime,
//     #[serde(with = "timestamp")]
//     pub updated_at: NaiveDateTime,
// }

// #[derive(Insertable, Deserialize, ToSchema)]
// #[diesel(table_name = questions)]
// pub struct NewQuestion {
//     pub content: String,
//     pub desc: Option<String>,
// }
#[add_orm_field]
#[diesel(table_name = questions)]
pub struct Question {
    pub content: String,
    pub desc: Option<String>,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, ToSchema)]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub content: String,
    #[serde(with = "timestamp")]
    #[schema(value_type = i64)]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub question_id: i32,
    pub content: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, ToSchema, Clone)]
#[diesel(belongs_to(Question))]
pub struct QuestionAnswer {
    pub id: i32,
    pub question_id: i32,
    pub answer_id: i32,
    pub content: String,
    #[serde(with = "timestamp")]
    #[schema(value_type = i64)]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable, Clone)]
#[diesel(table_name = question_answers)]
pub struct NewQuestionAnswer {
    pub question_id: i32,
    pub answer_id: i32,
    pub content: String,
}
#[derive(Deserialize, ToSchema)]
pub struct NewQuestionAnswerNth {
    pub answer_nth: i32,
    pub content: String,
}
