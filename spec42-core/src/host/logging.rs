//! Shared tracing initialization for Spec42 binaries.
//!
//! Log policy:
//! - `debug`: verbose indexing internals
//! - `info`: lifecycle milestones and summary metrics
//! - `warn`: recoverable anomalies and degraded behavior
//! - `error`: request failures and unexpected faults

use std::sync::Once;

static INIT_TRACING: Once = Once::new();

/// Initialize global tracing subscriber once.
/// Uses `RUST_LOG` when set, otherwise defaults to `kernel=info`.
pub fn init_tracing() {
    INIT_TRACING.call_once(|| {
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("kernel=info"));
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .with_writer(std::io::stderr)
            .init();
    });
}
