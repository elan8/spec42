//! Optional regression against the public MBSE vacuum-cleaner example.
//! Set `MBSE_VACUUM_EXAMPLE_DIR` to the repository root to enable.

use kernel::{default_server_config, validate_paths, ValidationRequest};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types::NumberOrString;

fn diagnostic_code_counts(report: &kernel::ValidationReport) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            if let Some(NumberOrString::String(code)) = &diagnostic.code {
                *counts.entry(code.clone()).or_default() += 1;
            }
        }
    }
    counts
}

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
            strict_diagnostics: false,
        },
    )
    .expect("vacuum validation report");

    let code_counts = diagnostic_code_counts(&report);

    assert_eq!(
        code_counts.get("visibility_violation").copied().unwrap_or(0),
        0,
        "private wildcard imports must not emit visibility_violation"
    );
    assert_eq!(
        code_counts
            .get("duplicate_namespace_member")
            .copied()
            .unwrap_or(0),
        0,
        "action def siblings must not collide as name 'def'"
    );
    assert_eq!(
        code_counts
            .get("recovered_part_usage_body_element")
            .copied()
            .unwrap_or(0),
        0,
        "valid v2 redefines keyword forms must parse without part-usage-body recovery"
    );
    assert!(
        report.summary.error_count <= 30,
        "expected error_count <= 30, got {} (warnings={})",
        report.summary.error_count,
        report.summary.warning_count
    );
}
