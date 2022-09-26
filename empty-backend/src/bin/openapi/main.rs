use empty_backend::services::questions;
use std::fs;
use utoipa::OpenApi;

fn main() {
    fs::write(
        "question.json",
        questions::Server::openapi().to_pretty_json().unwrap(),
    )
    .unwrap();
}
