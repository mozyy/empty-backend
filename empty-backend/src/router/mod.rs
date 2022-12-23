use actix_web::http::Uri;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    api::{oauth, questions},
    database::{self, DbPool},
};

pub fn get_router() -> Router {
    let pool = database::get_db_pool();
    Router::new()
        .route(
            "/questions",
            get(questions::index_get).post(questions::index_post),
        )
        .route(
            "/oauth",
            get(oauth::get_authorize).post(questions::index_post),
        )
        .with_state(pool)
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
