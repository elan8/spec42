//! Optional regression against the public MBSE vacuum-cleaner example.
//! Set `MBSE_VACUUM_EXAMPLE_DIR` to the repository root to enable.

use lsp_server::{default_server_config, validate_paths, ValidationRequest};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types::NumberOrString;

fn diagnostic_code_counts(report: &lsp_server::ValidationReport) -> HashMap<String, usize> {
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

    let cache = tempfile::tempdir().expect("cache dir");
    let engine = super::harness::test_engine(&cache, Vec::new());
    let config = Arc::new(default_server_config());
    let report = validate_paths(
        &engine,
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
        code_counts
            .get("visibility_violation")
            .copied()
            .unwrap_or(0),
        0,
        "private wildcard imports must not emit visibility_violation"
    );
    let def_name_collisions = report
        .documents
        .iter()
        .flat_map(|document| document.diagnostics.iter())
        .filter(|diagnostic| {
            matches!(
                &diagnostic.code,
                Some(NumberOrString::String(code)) if code == "duplicate_namespace_member"
            ) && diagnostic.message.contains("'def'")
        })
        .count();
    assert_eq!(
        def_name_collisions, 0,
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
    assert_eq!(
        code_counts
            .get("unresolved_pending_relationship")
            .copied()
            .unwrap_or(0),
        0,
        "use-case first/then succession must resolve without pending flow edges"
    );
    let subject_action_false_positive_dupes = report
        .documents
        .iter()
        .flat_map(|document| document.diagnostics.iter())
        .filter(|diagnostic| {
            matches!(
                &diagnostic.code,
                Some(NumberOrString::String(code)) if code == "duplicate_namespace_member"
            ) && diagnostic.message.contains("roboticVacuumCleaner")
                && !diagnostic.message.contains("(action)")
        })
        .count();
    assert_eq!(
        subject_action_false_positive_dupes, 0,
        "subject and then action with the same name must not collide as duplicate_namespace_member"
    );
    assert_eq!(
        code_counts
            .get("missing_closing_brace")
            .copied()
            .unwrap_or(0),
        0,
        "port def inout item bodies must parse without brace cascade errors"
    );
    assert_eq!(
        code_counts.get("incompatible_type_kind").copied().unwrap_or(0),
        0,
        "actor typed by part/item def must not emit incompatible_type_kind (SysML §7.11.2 / §7.22.2)"
    );
    assert!(
        report.summary.error_count <= 12,
        "expected error_count <= 12, got {} (warnings={})",
        report.summary.error_count,
        report.summary.warning_count
    );
}
