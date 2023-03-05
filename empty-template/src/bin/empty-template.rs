#[tokio::main]
async fn main() {
    empty_utils::init();
    empty_template::registry::register().await;
}
