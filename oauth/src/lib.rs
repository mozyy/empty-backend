use axum::{Router, routing::{get, post}};
use state::State;

pub mod endpoint;
pub mod handler;
pub mod primitives;
pub mod state;


pub fn new() -> Router {
Router::new()
    .route("/authorize", get(handler::authorize_get))
    .route("/token", post(handler::token))
    .with_state(State::new())
}