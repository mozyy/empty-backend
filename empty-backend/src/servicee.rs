use crate::protos::proto::user::oauth;
use empty_utils::errors::{Error, Result};
use oauth::o_auth_service_server::{OAuthService, OAuthServiceServer};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl OAuthService for MyGreeter {
    async fn token(
        &self,
        _request: tonic::Request<oauth::TokenRequest>,
    ) -> Result<tonic::Response<oauth::TokenResponse>, tonic::Status> {
        todo!();
    }
    async fn valid(
        &self,
        _request: tonic::Request<oauth::ValidRequest>,
    ) -> Result<tonic::Response<oauth::ValidResponse>, tonic::Status> {
        todo!();
    }
}

#[tokio::main]
async fn main() -> Result {
    let addr = "[::]:50091".parse().map_err(Error::other)?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(OAuthServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
