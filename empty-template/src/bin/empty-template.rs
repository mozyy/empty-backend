use empty_template::pb::template_service_server::TemplateServiceServer;

#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_registry::client::register(TemplateServiceServer::default())
        .await
        .unwrap();
}
