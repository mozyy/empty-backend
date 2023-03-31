use empty_utils::tonic::Resp;

use crate::pb::{heartbeat_server::Heartbeat, HeartbeatRequest, HeartbeatResponse};

pub struct Service;

#[tonic::async_trait]
impl Heartbeat for Service {
    async fn heartbeat(
        &self,
        _request: tonic::Request<HeartbeatRequest>,
    ) -> Resp<HeartbeatResponse> {
        Ok(tonic::Response::new(HeartbeatResponse {}))
    }
}
