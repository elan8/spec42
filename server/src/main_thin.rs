//! Thin `spec42` binary entrypoint: delegates to `spec42-core` LSP runner.

mod default_diagram_providers;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(default_diagram_providers::default_config());
    spec42_core::run_lsp(config, "spec42").await;
}

