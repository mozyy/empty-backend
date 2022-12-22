use empty_backend::openapi::{questions, resources};
use std::fs;
use utoipa::OpenApi;

fn main() {
    fs::write(
        "openapi/questions.yaml",
        questions::ApiDoc::openapi().to_yaml().unwrap(),
    )
    .unwrap();
    fs::write(
        "openapi/resources.yaml",
        resources::ApiDoc::openapi().to_yaml().unwrap(),
    )
    .unwrap();
}
