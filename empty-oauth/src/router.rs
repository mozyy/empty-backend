use crate::{controller, service::Service};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(controller::index))
        .route(
            "/authorize",
            get(controller::authorize).post(controller::authorize),
        )
        .route("/token", post(controller::token))
        .route("/refresh", post(controller::refresh))
        .with_state(Service::default())
    // .route("/edit", post(users::edit))
}
