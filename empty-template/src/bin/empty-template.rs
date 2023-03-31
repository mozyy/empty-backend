use empty_template::pb::template_server::TemplateServer;

#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_registry::client::register(TemplateServer::default())
        .await
        .unwrap();
}
