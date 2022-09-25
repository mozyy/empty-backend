// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        content -> Text,
    }
}

diesel::table! {
    question_answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer_id -> Int4,
        content -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        content -> Text,
        desc -> Nullable<Text>,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(question_answers -> answers (answer_id));
diesel::joinable!(question_answers -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    question_answers,
    questions,
);
