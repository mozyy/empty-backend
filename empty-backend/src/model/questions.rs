use crate::database::DbConnection;
use crate::schema::answers;
use crate::schema::questions;
use crate::utils::timestamp;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use empty_utils::add_orm_field;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Identifiable, Serialize, ToSchema)]
pub struct Question {
    pub id: i32,
    pub content: String,
    pub desc: Option<String>,
    #[serde(with = "timestamp")]
    #[schema(value_type = i64)]
    pub created_at: NaiveDateTime,
    #[serde(with = "timestamp")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub content: String,
    pub desc: Option<String>,
}
// #[add_orm_field]
// pub struct Question {
//     pub content: String,
//     pub desc: Option<String>,
// }

#[add_orm_field]
#[derive(Associations)]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub question_id: i32,
    pub content: String,
    pub correct: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct NewQuestionAnswerNth {
    pub answer_nth: i32,
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct QuestionResp {
    question: Question,
    answers: Vec<Answer>,
}

#[derive(Deserialize, ToSchema)]
pub struct QuestionReq {
    new_question: NewQuestion,
    new_answers: Vec<NewAnswer>,
}

impl Question {
    pub fn insert(req: &QuestionReq, conn: &PgConnection) -> Result<String, String> {
        todo!()
    }
    pub fn select_all(conn: &mut DbConnection) -> Result<Vec<QuestionResp>, Error> {
        let question = questions::table.load::<Question>(conn)?;
        let answer: Vec<Vec<Answer>> = Answer::belonging_to(&question)
            .load(conn)?
            .grouped_by(&question);
        let resp = question
            .into_iter()
            .zip(answer)
            .map(|(question, answers)| QuestionResp { question, answers })
            .collect::<Vec<_>>();
        Ok(resp)
    }
}
