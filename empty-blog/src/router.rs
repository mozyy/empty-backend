use crate::{controller, service::Service};
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(controller::get).post(controller::post_item))
        .route(
            "/:id",
            get(controller::get_item)
                .put(controller::put_item)
                .delete(controller::delete_item),
        )
        .with_state(Service::default())
    // .route("/edit", post(users::edit))
}
