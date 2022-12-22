use axum::{
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use empty_backend::router;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .nest("/x", router::get_router())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
