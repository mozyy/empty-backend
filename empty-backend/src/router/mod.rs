use actix_web::http::Uri;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub mod oauth;
pub mod questions;

pub fn get_router() -> Router {
    Router::new()
        .nest("/questions", questions::get_router())
        .with_state(())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
