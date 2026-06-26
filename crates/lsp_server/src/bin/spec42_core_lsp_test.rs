use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(kernel::default_server_config());
    kernel::run_lsp(config, "spec42-core-test").await;
}
