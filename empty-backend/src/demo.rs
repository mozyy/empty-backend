use crate::schema::questions;
extern crate empty_utils;

use empty_utils::add_orm_field;

#[add_orm_field]
#[diesel(table_name = questions)]
pub struct Question {
    // pub id: i32,
    pub content: String,
    pub desc: Option<String>,
}
