use std::str;

use async_trait::async_trait;
use empty_utils::{errors::Error, tonic::Resp};
use tonic::{Request, Response};

use crate::pb;

#[derive(Default)]
pub struct Service {}

#[async_trait]
impl pb::wx::wx_service_server::WxService for Service {
    async fn sns_jscode2session(
        &self,
        request: Request<pb::wx::SnsJscode2sessionRequest>,
    ) -> Resp<pb::wx::SnsJscode2sessionResponse> {
        let request = request.into_inner();
        let query = serde_qs::to_string(&request).map_err(Error::other)?;
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?{query}");
        let res = reqwest::get(url)
            .await
            .map_err(|e| tonic::Status::resource_exhausted(e.to_string()))?;
        let res = res
            .bytes()
            .await
            .map_err(|e| tonic::Status::resource_exhausted(e.to_string()))?;
        let res = match serde_json::from_slice::<pb::wx::SnsJscode2sessionResponse>(&res) {
            Ok(res) => res,
            Err(_e) => match serde_json::from_slice::<pb::wx::Error>(&res) {
                Ok(res) => {
                    return Err(tonic::Status::resource_exhausted(format!("code:{:?}", res)))
                }
                Err(_e) => {
                    return Err(tonic::Status::resource_exhausted(
                        str::from_utf8(&res).map_err(Error::other)?,
                    ))
                }
            },
        };
        log::info!("{:?}", res);
        Ok(Response::new(res))
    }
}
