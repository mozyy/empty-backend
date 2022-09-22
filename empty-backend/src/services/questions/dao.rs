use crate::schema::answers::{self, dsl::*};
use crate::schema::questions::{self, dsl::*};
use crate::DbPool;
use actix_web::{get, post, put, web, Error as ActixError, HttpResponse, Responder, Scope};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Question {
    pub id: Option<i32>,
    pub content: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Queryable, Insertable, Associations, Clone)]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub id: Option<i32>,
    pub question_id: Option<i32>,
    pub content: String,
}
