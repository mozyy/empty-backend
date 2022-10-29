use actix_web::http::Uri;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    api::questions as api,
    database::{self, DbPool},
};

pub fn get_router() -> Router<DbPool> {
    let pool = database::get_db_pool();

    Router::with_state(pool).route("/", get(api::index_get).post(api::index_post))
}
