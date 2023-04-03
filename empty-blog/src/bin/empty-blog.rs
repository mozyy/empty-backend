use empty_blog::{
    openapi::{generate_file, swagger},
    router,
};
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    log::info!("listening on http://{}", addr);
    let swagger = swagger();
    let app = router::router().merge(swagger);
    // .layer(CompressionLayer::new());
    // generate_file();
    // `GET /` goes to `root`
    // `POST /users` goes to `create_user`
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
