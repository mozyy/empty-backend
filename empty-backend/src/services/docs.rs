use actix_web::{get, post, put, web, HttpResponse, Responder, Scope};
use diesel::Queryable;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use super::questions;

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