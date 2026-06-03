//! Optional regression against the public MBSE vacuum-cleaner example.
//! Set `MBSE_VACUUM_EXAMPLE_DIR` to the repository root to enable.

use kernel::{default_server_config, validate_paths, ValidationRequest};
use std::path::PathBuf;
use std::sync::Arc;

#[test]
#[ignore = "requires MBSE_VACUUM_EXAMPLE_DIR pointing at the public example checkout"]
fn mbse_vacuum_example_diagnostic_baseline() {
    let Some(root) = std::env::var_os("MBSE_VACUUM_EXAMPLE_DIR") else {
        return;
    };
    let root = PathBuf::from(root);
    if !root.is_dir() {
        panic!(
            "MBSE_VACUUM_EXAMPLE_DIR must be a directory: {}",
            root.display()
        );
    }

    let config = Arc::new(default_server_config());
    let report = validate_paths(
        &config,
        ValidationRequest {
            targets: vec![root.clone()],
            workspace_root: Some(root),
            library_paths: Vec::new(),
            parallel_enabled: true,
        },
    )
    .expect("vacuum validation report");

    assert!(
        report.summary.error_count < 120,
        "expected errors below legacy full-report baseline (132), got {} errors (warnings={})",
        report.summary.error_count,
        report.summary.warning_count
    );
}
