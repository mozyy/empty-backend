use std::fs;

use crate::controller as blog;
use crate::model;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        paths(
            blog::get,
            blog::get_item,
            blog::post_item,
            blog::put_item,
            blog::delete_item,
        ),
        components(
            schemas(model::Blog, model::NewBlog)
        ),
        tags(
            (name = "blog", description = "blog api")
        ),
        servers(
            (url = "https://yyuck.com/i/api", description = "Local server")
        )
    )]
struct ApiDoc;

pub fn generate_file() {
    fs::write(
        "docs/blog.json",
        ApiDoc::openapi().to_pretty_json().unwrap(),
    )
    .unwrap();
}

pub fn swagger() -> SwaggerUi {
    SwaggerUi::new("/i/api/docs").url("/i/api/docs/blog.json", ApiDoc::openapi())
}
