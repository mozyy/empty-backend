use empty_utils::{errors::ServiceResult, tonic};

#[tokio::main]
async fn main() -> ServiceResult {
    empty_utils::init();

    let addr = "0.0.0.0:50052".parse().unwrap();
    let rest = axum::Server::bind(&addr).serve(oauth::new().into_make_service());

    let addr = "0.0.0.0:50051".parse().unwrap();

    let grpc = tonic::server()
        .add_service(blog::new())
        // .add_service(lottery::new())
        .serve(addr);
    let (_, grpc) = tokio::join!(rest, grpc);
    grpc?;
    Ok(())
}
