use empty_backend::openapi::questions;
use std::fs;
use utoipa::OpenApi;

fn main() {
    fs::write(
        "question.json",
        questions::ApiDoc::openapi().to_pretty_json().unwrap(),
    )
    .unwrap();
}
