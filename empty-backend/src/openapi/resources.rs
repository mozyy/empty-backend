// use crate::api::questions;
use crate::model::resources::{NewResource, Resource};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    // paths(questions::index_get, questions::index_post),
    components(schemas(NewResource, Resource))
)]
pub struct ApiDoc {}
