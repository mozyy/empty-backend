use super::questions;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

impl Server {
    pub fn service(&self) -> SwaggerUi {
        SwaggerUi::new("/docs/{_:.*}").urls(vec![(
            Url::with_primary("questions", "/docs/api-doc/questions.json", true),
            questions::Server::openapi(),
        )])
    }
}
