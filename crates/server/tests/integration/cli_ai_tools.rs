//! CLI coverage for agent surfaces (`explain-diagnostic`, `model-export`).

use std::path::PathBuf;
use std::process::Command;

use crate::common::with_isolated_data_dir;
use serde_json::Value;

fn kitchen_timer_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml")
}

fn spec42_bin() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_spec42")
        .map(PathBuf::from)
        .expect("CARGO_BIN_EXE_spec42 (run via cargo test)")
}

fn run_spec42_json(args: &[&str]) -> Value {
    let output = Command::new(spec42_bin())
        .args(args)
        .output()
        .expect("spec42 subprocess");
    assert!(
        output.status.success(),
        "spec42 failed: status={} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("stdout JSON")
}

#[test]
fn cli_explain_diagnostic_returns_catalog_entry() {
    with_isolated_data_dir(|| {
        let cli = run_spec42_json(&[
            "explain-diagnostic",
            "--code",
            "unresolved_type_reference",
            "--format",
            "json",
        ]);
        assert_eq!(
            cli.get("code").and_then(|v| v.as_str()),
            Some("unresolved_type_reference")
        );
        assert_eq!(
            cli.get("catalog")
                .and_then(|c| c.get("code"))
                .and_then(|v| v.as_str()),
            Some("unresolved_type_reference")
        );
    });
}

#[test]
fn cli_model_export_emits_the_typed_projection() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let path_str = path.display().to_string();

        // No `--max-nodes`: the default is unbounded, so this is the whole workspace.
        let full = run_spec42_json(&["model-export", &path_str, "--format", "json"]);

        assert_eq!(
            full.get("summary")
                .and_then(|s| s.get("error_count"))
                .and_then(|v| v.as_u64()),
            Some(0),
            "expected no errors in KitchenTimer example"
        );

        let projection = full.get("projection").expect("projection object");
        assert_eq!(
            projection.get("schema_version").and_then(|v| v.as_u64()),
            Some(1)
        );
        let envelope = projection.get("envelope").expect("envelope object");
        assert_eq!(
            envelope.get("phase").and_then(|v| v.as_str()),
            Some("resolved")
        );
        assert_eq!(
            envelope.get("complete").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert!(
            envelope
                .get("admitted")
                .and_then(|a| a.get("standard_library"))
                .and_then(|v| v.as_u64())
                .is_some_and(|count| count > 0),
            "the KitchenTimer example resolves against the bundled standard library"
        );

        let elements = projection
            .get("elements")
            .and_then(|v| v.as_array())
            .expect("elements array");
        assert!(!elements.is_empty(), "KitchenTimer has workspace elements");
        assert!(
            elements
                .iter()
                .all(|element| element.get("token").and_then(|v| v.as_str()).is_some()),
            "every projected element carries a stable token"
        );
        let truncation = projection.get("truncation").expect("truncation object");
        let elements_total = truncation
            .get("elements_total")
            .and_then(|v| v.as_u64())
            .expect("elements_total");
        assert_eq!(elements_total, elements.len() as u64);
        assert_eq!(
            truncation
                .get("elements_incomplete")
                .and_then(|v| v.as_u64()),
            Some(0)
        );

        // `--max-nodes` bounds the element list and records the truncation.
        let bounded = run_spec42_json(&[
            "model-export",
            &path_str,
            "--max-nodes",
            "1",
            "--format",
            "json",
        ]);
        let bounded_truncation = bounded
            .get("projection")
            .and_then(|p| p.get("truncation"))
            .expect("truncation object");
        assert_eq!(
            bounded_truncation
                .get("elements_returned")
                .and_then(|v| v.as_u64()),
            Some(1)
        );
        assert_eq!(
            bounded_truncation
                .get("elements_total")
                .and_then(|v| v.as_u64()),
            Some(elements_total),
            "the total is unbounded even when the returned list is truncated"
        );
        assert_eq!(
            bounded
                .get("projection")
                .and_then(|p| p.get("elements"))
                .and_then(|v| v.as_array())
                .map(Vec::len),
            Some(1)
        );
    });
}
