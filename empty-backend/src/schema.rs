// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        content -> Text,
        question_id -> Nullable<Int4>,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        content -> Text,
        answer_id -> Nullable<Int4>,
        desc -> Nullable<Text>,
    }
}

diesel::joinable!(answers -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
