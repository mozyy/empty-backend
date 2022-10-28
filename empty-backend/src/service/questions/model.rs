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
pub struct Question {
    pub content: String,
    pub desc: Option<String>,
}

#[add_orm_field]
#[derive(Associations)]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub question_id: i32,
    pub content: String,
}

#[add_orm_field]
#[derive(Associations, Clone)]
#[diesel(belongs_to(Question))]
#[diesel(belongs_to(Answer))]
pub struct QuestionAnswer {
    pub question_id: i32,
    pub answer_id: i32,
    pub content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct NewQuestionAnswerNth {
    pub answer_nth: i32,
    pub content: String,
}
