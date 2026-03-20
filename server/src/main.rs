//! Thin `spec42` binary entrypoint: delegates to `spec42-core` LSP runner.

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(spec42_core::default_server_config());
    spec42_core::run_lsp(config, "spec42").await;
}
