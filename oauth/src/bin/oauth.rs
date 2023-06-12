use std::net::{Ipv4Addr, SocketAddr};

use oauth::{handler, state::State};
use oxide_auth::{
    endpoint::Scope,
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
#[tokio::main]
async fn main() {
    empty_utils::init();
    // build our application with a route
    let app = Router::new()
        .route("/authorize", get(handler::authorize_get))
        .route("/token", post(handler::token))
        .with_state(State::new());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
