use pb::{template_service_server::TemplateService, TemplateRequest, TemplateResponse};

pub mod pb {
    tonic::include_proto!("empty.template.v1");
}

pub mod registry {
    use crate::{
        pb::template_service_server::{TemplateService, TemplateServiceServer},
        Service,
    };
    use empty_registry::{
        pb::{registry_service_client::RegistryServiceClient, RegisterRequest},
        REGISTRY_ADDR,
    };
    use std::net::TcpListener;
    use tonic::transport::{Endpoint, NamedService, Server};
    pub async fn register() {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        println!("RegistryServer listening on {}", local_addr);

        let service = Service::default();
        let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

        let mut client = RegistryServiceClient::connect(format!("http://{REGISTRY_ADDR}"))
            .await
            .unwrap();

        let request = tonic::Request::new(RegisterRequest {
            name: TemplateServiceServer::<Service>::NAME.into(),
            endpoint: local_addr.to_string(),
        });

        client.register(request).await.unwrap();
        Server::builder()
            // .trace_fn(|request| {
            //     log::info!("resive request: {:?}", request);
            //     tracing::info_span!("registry_server", "{:?}", request)
            // })
            // .layer(TraceLayer::new_for_http())
            // TODO: helth_service
            .add_service(TemplateServiceServer::new(service))
            // TODO: proxy_service
            // TODO: oauth_service
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    }
}

#[derive(Default)]
struct Template {}
impl Template {
    pub fn get_template(&self, name: &str) -> String {
        format!("response template service: {name}").to_string()
    }
}
#[derive(Default)]
struct Service {
    template: Template,
}

// TODO: async_trait to async_fn_in_trait
// https://github.com/rust-lang/rust/issues/91611
#[tonic::async_trait]
impl TemplateService for Service {
    async fn template(
        &self,
        request: tonic::Request<TemplateRequest>,
    ) -> Result<tonic::Response<TemplateResponse>, tonic::Status> {
        log::info!("template reservice: {:?}", request);
        let response = self
            .template
            .get_template(request.into_inner().name.as_str());
        let response = TemplateResponse { response };
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
