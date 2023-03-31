use crate::schema::answers;
use crate::schema::questions;
use diesel::prelude::*;
use diesel::result::Error;
use empty_macro::add_orm_field;
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
    pub fn insert(req: &Vec<QuestionReq>, conn: &mut PgConnection) -> Result<Vec<i32>, Error> {
        conn.transaction::<_, diesel::result::Error, _>(move |conn| {
            let (question, answer): (Vec<_>, Vec<_>) = req
                .into_iter()
                .map(
                    |QuestionReq {
                         new_question,
                         new_answers,
                     }| (new_question, new_answers),
                )
                .unzip();
            let new_question_ids = diesel::insert_into(questions::dsl::questions)
                .values(question)
                .returning(questions::dsl::id)
                .get_results::<i32>(conn)?;
            diesel::insert_into(answers::dsl::answers)
                .values(
                    new_question_ids
                        .clone()
                        .into_iter()
                        .zip(answer)
                        .flat_map(|(i, answer)| {
                            answer.iter().map(move |a| {
                                (
                                    answers::columns::question_id.eq(i),
                                    answers::columns::content.eq(&a.content),
                                )
                            })
                        })
                        .collect::<Vec<_>>(),
                )
                // .returning((answers::dsl::question_id, answers::dsl::id))
                // .get_results::<(i32, i32)>(conn)?;
                .execute(conn)?;

            Ok(new_question_ids)
        })
    }
    pub fn select_all(conn: &mut PgConnection) -> Result<Vec<QuestionResp>, Error> {
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
