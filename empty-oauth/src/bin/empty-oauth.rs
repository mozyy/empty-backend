use std::net::SocketAddr;

use empty_oauth::router;
#[tokio::main]
async fn main() {
    empty_utils::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("listening on http://{}", addr);
    let app = router::router();
    // `GET /` goes to `root`
    // `POST /users` goes to `create_user`
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
