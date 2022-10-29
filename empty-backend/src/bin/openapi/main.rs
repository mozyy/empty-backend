use empty_backend::openapi::questions;
use std::fs;
use utoipa::OpenApi;

fn main() {
    fs::write(
        "question.yaml",
        questions::ApiDoc::openapi().to_yaml().unwrap(),
    )
    .unwrap();
}
