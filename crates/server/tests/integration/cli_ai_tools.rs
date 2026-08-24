//! CLI coverage for agent surfaces (`explain-diagnostic`, `model-summary`).

mod common;

use std::path::PathBuf;
use std::process::Command;

use common::with_isolated_data_dir;
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
fn cli_model_summary_is_validation_only_until_typed_projection_lands() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let path_str = path.display().to_string();

        let cli = run_spec42_json(&[
            "model-summary",
            &path_str,
            "--max-nodes",
            "1",
            "--format",
            "json",
        ]);

        assert_eq!(
            cli.get("truncation")
                .and_then(|t| t.get("nodes_returned"))
                .and_then(|v| v.as_u64()),
            Some(0)
        );
        assert_eq!(
            cli.get("summary")
                .and_then(|s| s.get("error_count"))
                .and_then(|v| v.as_u64()),
            Some(0),
            "expected no errors in KitchenTimer example"
        );
    });
}
