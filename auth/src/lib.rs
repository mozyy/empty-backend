use empty_utils::errors::Result;
use proto::pb::auth::auth::auth_service_server::AuthServiceServer;

pub(crate) mod dao;
pub(crate) mod model;
pub(crate) mod service;

pub async fn get_service() -> Result<AuthServiceServer<service::Service>> {
    let auth = AuthServiceServer::new(service::Service::new().await?);
    Ok(auth)
}
