use oauth::o_auth_service_server::{OAuthService, OAuthServiceServer};
use protos::proto::user::oauth;
use tonic::{transport::Server, Request, Response, Status};

mod protos;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl OAuthService for MyGreeter {
    async fn token(
        &self,
        request: tonic::Request<oauth::TokenRequest>,
    ) -> Result<tonic::Response<oauth::TokenResponse>, tonic::Status> {
        todo!();
    }
    async fn valid(
        &self,
        request: tonic::Request<oauth::ValidRequest>,
    ) -> Result<tonic::Response<oauth::ValidResponse>, tonic::Status> {
        todo!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50091".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(OAuthServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
