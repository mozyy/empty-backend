use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::{
    api::{oauth, questions},
};

pub fn get_router() -> Router {
    let pool = empty_utils::diesel::db::get();
    Router::new()
        .route(
            "/questions",
            get(questions::index_get).post(questions::index_post),
        )
        .route(
            "/oauth/authorize",
            get(oauth::get_authorize).post(questions::index_post),
        )
        .route(
            "/oauth/clients",
            get(oauth::get_clients).post(questions::index_post),
        )
        .with_state(pool)
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
