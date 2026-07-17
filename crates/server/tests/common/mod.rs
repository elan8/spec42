//! Shared helpers for integration tests that mutate process-global env vars.
//!
//! Included (via `#[path]`) into every sibling integration-test binary under `tests/`; each
//! binary only exercises a subset, so unused-in-this-binary is expected.
#![allow(dead_code)]

/// Reason string for `#[ignore]` on slow agent/API/MCP integration tests.
///
/// Default `cargo test` skips these; CI and focused runs use `--include-ignored`:
///
/// ```bash
/// cargo test -p spec42 --test api_http --test mcp_tools --test mcp_protocol \
///   --test mcp_binary --test cli_ai_tools -- --include-ignored
/// ```
pub const AGENT_SURFACE_IGNORE: &str =
    "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored";

use tempfile::TempDir;
use tokio::sync::Mutex;

static SPEC42_DATA_DIR_LOCK: Mutex<()> = Mutex::const_new(());

/// Run a test with an isolated `SPEC42_DATA_DIR`, serialized across threads in this binary.
#[allow(dead_code)]
pub fn with_isolated_data_dir(test: impl FnOnce()) {
    let _guard = SPEC42_DATA_DIR_LOCK.blocking_lock();
    let data_dir = TempDir::new().expect("temp data dir");
    let previous = std::env::var_os("SPEC42_DATA_DIR");
    std::env::set_var("SPEC42_DATA_DIR", data_dir.path());
    test();
    match previous {
        Some(value) => std::env::set_var("SPEC42_DATA_DIR", value),
        None => std::env::remove_var("SPEC42_DATA_DIR"),
    }
}

/// Async variant of [`with_isolated_data_dir`].
#[allow(dead_code)]
pub async fn with_isolated_data_dir_async<F, Fut, T>(f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let _guard = SPEC42_DATA_DIR_LOCK.lock().await;
    let data_dir = TempDir::new().expect("temp data dir");
    let previous = std::env::var_os("SPEC42_DATA_DIR");
    std::env::set_var("SPEC42_DATA_DIR", data_dir.path());
    let result = f().await;
    match previous {
        Some(value) => std::env::set_var("SPEC42_DATA_DIR", value),
        None => std::env::remove_var("SPEC42_DATA_DIR"),
    }
    result
}
