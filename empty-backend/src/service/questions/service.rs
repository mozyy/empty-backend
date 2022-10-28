use crate::schema::answers;
use crate::schema::question_answers;
use crate::schema::questions;
use diesel::prelude::*;

use super::model::{
    Answer, NewAnswer, NewQuestion, NewQuestionAnswer, NewQuestionAnswerNth, Question,
    QuestionAnswer,
};

pub type DaoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_question(
    conn: &mut PgConnection,
    question: (&NewQuestion, &Vec<NewAnswer>, &NewQuestionAnswerNth), // prevent collision with `name` column imported inside the function
) -> DaoResult<i32> {
    let &i = insert_questions(conn, vec![question])?
        .first()
        .ok_or("insert error")?;
    Ok(i)
}

pub fn insert_questions(
    conn: &mut PgConnection,
    question: Vec<(&NewQuestion, &Vec<NewAnswer>, &NewQuestionAnswerNth)>, // prevent collision with `name` column imported inside the function
) -> DaoResult<Vec<i32>> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    Ok(
        conn.transaction::<_, diesel::result::Error, _>(move |conn| {
            let (question, answer): (Vec<_>, Vec<_>) =
                question.into_iter().map(|(q, a, n)| ((q, n), a)).unzip();
            let new_question_ids = diesel::insert_into(questions::dsl::questions)
                .values(
                    question
                        .clone()
                        .into_iter()
                        .map(|q| q.0)
                        .collect::<Vec<_>>(),
                )
                .returning(questions::dsl::id)
                .get_results(conn)?;
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
            let a = question
                .into_iter()
                .zip(new_question_ids.clone())
                .map(|((_, n), a)| {
                    answers::table
                        .filter(answers::dsl::question_id.eq(a))
                        .select(answers::dsl::id)
                        .offset(n.answer_nth as i64)
                        .first::<i32>(conn)
                        .map(|i| NewQuestionAnswer {
                            question_id: a,
                            answer_id: i,
                            content: n.content.to_owned(),
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            diesel::insert_into(question_answers::dsl::question_answers)
                .values(a)
                .execute(conn)?;
            Ok(new_question_ids)
        })?,
    )
}

pub fn select_questions(
    conn: &mut PgConnection,
) -> DaoResult<Vec<(Question, Vec<Answer>, QuestionAnswer)>> {
    let value = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let question = questions::table.load::<Question>(conn)?;
        let answer: Vec<Vec<Answer>> = Answer::belonging_to(&question)
            .load(conn)?
            .grouped_by(&question);
        let question_answer: Vec<Vec<QuestionAnswer>> = QuestionAnswer::belonging_to(&question)
            .load(conn)?
            .grouped_by(&question);
        let data = question
            .into_iter()
            .zip(answer)
            .zip(question_answer)
            .map(|((q, a), n)| match n.first() {
                Some(qn) => Ok((q, a, qn.to_owned())),
                None => Err("answer error"),
            })
            .collect::<Result<Vec<_>, _>>();
        Ok(data)
    })??;
    Ok(value)
}
