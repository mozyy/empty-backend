mod demo {
    use crate::schema::questions;
    use crate::add_orm_field;
    #[add_orm_field]
    pub struct Question {
        pub content: String,
        pub desc: Option<String>,
    }
}
