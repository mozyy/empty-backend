use crate::schema::questions;

use empty_utils::add_orm_field;

#[add_orm_field]
pub struct Question {
    // pub id: i32,
    pub content: String,
    pub desc: Option<String>,
}
