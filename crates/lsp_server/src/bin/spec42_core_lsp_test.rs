use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(lsp_server::default_server_config());
    lsp_server::run_lsp(config, "spec42-core-test").await;
}
