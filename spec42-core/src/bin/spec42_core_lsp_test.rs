use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(spec42_core::default_server_config());
    spec42_core::run_lsp(config, "spec42-core-test").await;
}
