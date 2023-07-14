use empty_backend::openapi::{questions, resources};
use empty_utils::errors::{Error, Result};
use std::fs;
use utoipa::OpenApi;

fn main() -> Result {
    fs::write(
        "openapi/questions.yaml",
        questions::ApiDoc::openapi()
            .to_yaml()
            .map_err(Error::other)?,
    )
    .map_err(Error::other)?;
    fs::write(
        "openapi/resources.yaml",
        resources::ApiDoc::openapi()
            .to_yaml()
            .map_err(Error::other)?,
    )
    .map_err(Error::other)?;
    Ok(())
}
