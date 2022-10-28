use actix_web::http::Uri;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

// mod docs;
pub mod questions;
mod system;

// pub fn get_router() {
//     Router::new()
//         .nest("/questions", questions::get_router())
//         .fallback(handler_404);
// }

// async fn handler_404() -> impl IntoResponse {
//     (StatusCode::NOT_FOUND, "nothing to see here")
// }
