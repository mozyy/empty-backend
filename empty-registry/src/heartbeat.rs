use crate::pb::{heartbeat_service_server::HeartbeatService, HeartbeatRequest};

pub struct Service;

#[tonic::async_trait]
impl HeartbeatService for Service {
    async fn heartbeat(
        &self,
        _request: tonic::Request<HeartbeatRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
