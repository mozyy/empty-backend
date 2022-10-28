use utoipa::openapi;

#[openapi(
  paths(get, post, id_get, id_post, id_put,),
  components(schemas(
      NewAnswer,
      NewQuestion,
      Answer,
      Question,
      PostParams,
      NewQuestionAnswerNth,
      QuestionAnswer,
      GetResp,
      Resource,
      // Type,
      // Route,
  ))
)]
pub struct Server {}
