use pb::{
    oauth_service_server::OauthService, oauth_service_server::OauthServiceServer,
    OauthRequest, OauthResponse,
};

pub mod pb {
    tonic::include_proto!("empty.oauth.v1");
}

impl Default for OauthServiceServer<Service> {
    fn default() -> Self {
        Self::new(Service::default())
    }
}

#[derive(Default)]
struct Oauth {}
impl Oauth {
    pub fn get_oauth(&self, name: &str) -> String {
        format!("response oauth service: {name}")
    }
}
#[derive(Default)]
pub struct Service {
    oauth: Oauth,
}

// TODO: async_trait to async_fn_in_trait
// https://github.com/rust-lang/rust/issues/91611
#[tonic::async_trait]
impl OauthService for Service {
    async fn oauth(
        &self,
        request: tonic::Request<OauthRequest>,
    ) -> Result<tonic::Response<OauthResponse>, tonic::Status> {
        log::info!("oauth reservice: {:?}", request);
        let response = self
            .oauth
            .get_oauth(request.into_inner().name.as_str());
        let response = OauthResponse { response };
        Ok(tonic::Response::new(response))
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
