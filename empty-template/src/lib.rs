use pb::{
    template_service_server::TemplateService, template_service_server::TemplateServiceServer,
    TemplateRequest, TemplateResponse,
};

pub mod pb {
    tonic::include_proto!("empty.template.v1");
}

impl Default for TemplateServiceServer<Service> {
    fn default() -> Self {
        Self::new(Service::default())
    }
}

#[derive(Default)]
struct Template {}
impl Template {
    pub fn get_template(&self, name: &str) -> String {
        format!("response template service: {name}")
    }
}
#[derive(Default)]
pub struct Service {
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
