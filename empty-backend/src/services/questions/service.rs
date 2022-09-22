/// Run query using Diesel to insert a new database row and return the result.
fn insert_new_question(
    conn: &mut PgConnection,
    question: &Question, // prevent collision with `name` column imported inside the function
) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::questions::dsl::*;
    Ok(conn.transaction::<_, diesel::result::Error, _>(|conn| {
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
    })?)
}
