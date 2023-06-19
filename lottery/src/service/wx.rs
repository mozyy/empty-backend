use async_trait::async_trait;
use empty_utils::tonic::Resp;
use tonic::{Request, Response};

use crate::pb::wx as pb;

#[derive(Default)]
pub struct Service {}

#[async_trait]
impl pb::wx_service_server::WxService for Service {
    async fn sns_jscode2session(
        &self,
        request: Request<pb::SnsJscode2sessionRequest>,
    ) -> Resp<pb::SnsJscode2sessionResponse> {
        let request = request.into_inner();
        let url = serde_qs::to_string(&request).unwrap();
        let res: pb::SnsJscode2sessionResponse = reqwest::get(url)
            .await
            .map_err(|e| tonic::Status::resource_exhausted(e.to_string()))?
            .json()
            .await
            .map_err(|e| tonic::Status::resource_exhausted(e.to_string()))?;
        log::info!("{:?}", res);
        Ok(Response::new(res))
    }
}
