use empty_register::register::register_service_server::RegisterService;
use empty_register::{RegisterServiceServer, Services};
use tonic::{transport::Server, Request, Response, Status};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let greeter = Services::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(RegisterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
