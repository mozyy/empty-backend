use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::scopes;

#[derive(Queryable, Identifiable)]
pub struct Scope {
    pub id: i32,
    pub scope: String,
    pub pattern: String,
    pub desc: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name =scopes)]
pub struct NewScope {
    pub scope: String,
    pub pattern: String,
    pub desc: Option<String>,
}

impl NewScope {
    pub fn new(scope: String, pattern: String, desc: Option<String>) -> Self {
        Self {
            scope,
            pattern,
            desc,
        }
    }
}
