use std::net::{Ipv4Addr, SocketAddr};

use empty_utils::errors::{Error, Result};
use oauth::{handler, state::State};

use axum::{
    routing::{get, post},
    Router,
};
#[tokio::main]
async fn main() -> Result {
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
        .map_err(Error::other)?;
    Ok(())
}
