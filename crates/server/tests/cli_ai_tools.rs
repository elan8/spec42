//! CLI parity for agent surfaces (`explain-diagnostic`, `model-summary`) vs MCP handlers.

mod common;

use std::path::PathBuf;
use std::process::Command;

use common::with_isolated_data_dir;
use serde_json::Value;
use spec42::mcp::handlers::{handle_spec42_explain_diagnostic, handle_spec42_model_summary};

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
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
fn cli_explain_diagnostic_matches_mcp_catalog() {
    with_isolated_data_dir(|| {
        let mcp = handle_spec42_explain_diagnostic(serde_json::json!({
            "code": "unresolved_type_reference",
        }))
        .expect("mcp explain");
        let cli = run_spec42_json(&[
            "explain-diagnostic",
            "--code",
            "unresolved_type_reference",
            "--format",
            "json",
        ]);
        assert_eq!(
            cli.get("code").and_then(|v| v.as_str()),
            mcp.get("code").and_then(|v| v.as_str())
        );
        assert_eq!(
            cli.get("catalog")
                .and_then(|c| c.get("code"))
                .and_then(|v| v.as_str()),
            mcp.get("catalog")
                .and_then(|c| c.get("code"))
                .and_then(|v| v.as_str())
        );
    });
}

#[test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
fn cli_model_summary_matches_mcp_with_max_nodes_one() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let path_str = path.display().to_string();

        let mcp = handle_spec42_model_summary(serde_json::json!({
            "path": path_str,
            "max_nodes": 1,
        }))
        .expect("mcp model summary");

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
            mcp.get("truncation")
                .and_then(|t| t.get("nodes_returned"))
                .and_then(|v| v.as_u64())
        );
        assert_eq!(
            cli.get("summary")
                .and_then(|s| s.get("error_count"))
                .and_then(|v| v.as_u64()),
            mcp.get("summary")
                .and_then(|s| s.get("error_count"))
                .and_then(|v| v.as_u64())
        );
    });
}
