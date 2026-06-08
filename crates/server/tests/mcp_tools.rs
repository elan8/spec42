//! MCP tool handlers (same logic as spec42-mcp stdio server).

mod common;

use std::path::PathBuf;

use common::with_isolated_data_dir;
use serde_json::json;
use spec42::mcp::handlers::{
    handle_spec42_check, handle_spec42_doctor, handle_spec42_explain_diagnostic,
    handle_spec42_model_summary,
};
use spec42::mcp::server::{execute_tool, MCP_TOOL_NAMES};

fn kitchen_timer_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml")
}

fn invalid_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mcp_invalid.sysml")
}

#[test]
fn mcp_tool_names_match_server_registration() {
    assert_eq!(MCP_TOOL_NAMES.len(), 4);
    assert!(MCP_TOOL_NAMES.contains(&"spec42_check"));
    assert!(MCP_TOOL_NAMES.contains(&"spec42_doctor"));
}

#[test]
fn mcp_spec42_check_kitchen_timer() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let value = handle_spec42_check(json!({
            "path": path.display().to_string(),
        }))
        .expect("spec42_check");
        let summary = value.get("summary").expect("summary");
        assert_eq!(summary.get("error_count").and_then(|v| v.as_u64()), Some(0));
    });
}

#[test]
fn mcp_spec42_check_rejects_missing_path() {
    with_isolated_data_dir(|| {
        let err = handle_spec42_check(json!({})).expect_err("missing path");
        assert!(
            err.contains("Invalid arguments") || err.contains("path"),
            "unexpected error: {err}"
        );
    });
}

#[test]
fn mcp_spec42_check_unknown_path_returns_error() {
    with_isolated_data_dir(|| {
        let err = handle_spec42_check(json!({
            "path": "C:/nonexistent/spec42-mcp-test-path/model.sysml",
        }))
        .expect_err("unknown path");
        assert!(
            err.contains("No .sysml")
                || err.contains("not found")
                || err.contains("Failed to resolve")
                || err.contains("cannot find"),
            "unexpected error: {err}"
        );
    });
}

#[test]
fn mcp_execute_tool_unknown_tool_returns_mcp_error() {
    with_isolated_data_dir(|| {
        let result = execute_tool("nope", json!({})).expect("tool result");
        assert_eq!(result.is_error, Some(true));
        let structured = result.structured_content.expect("structured content");
        assert!(structured.get("error").is_some());
        let suggestion = structured["error"]["suggestion"]
            .as_str()
            .unwrap_or_default();
        assert!(suggestion.contains("list_tools"));
    });
}

#[test]
fn mcp_spec42_doctor_returns_version() {
    with_isolated_data_dir(|| {
        let value = handle_spec42_doctor(json!({})).expect("spec42_doctor");
        assert!(value.get("version").and_then(|v| v.as_str()).is_some());
        assert!(value.get("stdlib_source_kind").is_some());
    });
}

#[test]
fn mcp_spec42_model_summary_returns_nodes() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let value = handle_spec42_model_summary(json!({
            "path": path.display().to_string(),
            "max_nodes": 50,
        }))
        .expect("spec42_model_summary");
        let nodes = value
            .get("nodes")
            .and_then(|v| v.as_array())
            .expect("nodes");
        assert!(!nodes.is_empty());
        let rels = value
            .get("relationships")
            .and_then(|v| v.as_array())
            .expect("relationships");
        for rel in rels {
            let kind = rel.get("kind").and_then(|v| v.as_str()).unwrap_or("");
            assert!(
                matches!(kind, "typing" | "connection" | "reference"),
                "unexpected relationship kind: {kind}"
            );
        }
    });
}

#[test]
fn mcp_model_summary_respects_max_nodes() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let value = handle_spec42_model_summary(json!({
            "path": path.display().to_string(),
            "max_nodes": 1,
        }))
        .expect("spec42_model_summary");
        let truncation = value.get("truncation").expect("truncation");
        assert_eq!(
            truncation.get("nodes_returned").and_then(|v| v.as_u64()),
            Some(1)
        );
        let nodes = value
            .get("nodes")
            .and_then(|v| v.as_array())
            .expect("nodes");
        assert_eq!(nodes.len(), 1);
    });
}

#[test]
fn mcp_spec42_explain_diagnostic_catalog() {
    with_isolated_data_dir(|| {
        let value = handle_spec42_explain_diagnostic(json!({
            "code": "unresolved_type_reference",
        }))
        .expect("spec42_explain_diagnostic");
        let catalog = value.get("catalog").expect("catalog");
        assert_eq!(
            catalog.get("code").and_then(|v| v.as_str()),
            Some("unresolved_type_reference")
        );
        assert!(catalog.get("typical_fix").is_some());
        assert_eq!(
            catalog.get("alignment").and_then(|v| v.as_str()),
            Some("spec_constraint")
        );
    });
}

#[test]
fn mcp_explain_unknown_code_has_no_catalog() {
    with_isolated_data_dir(|| {
        let value = handle_spec42_explain_diagnostic(json!({
            "code": "not_a_real_diagnostic_code",
        }))
        .expect("spec42_explain_diagnostic");
        assert!(value
            .get("catalog")
            .unwrap_or(&serde_json::Value::Null)
            .is_null());
        let sample = value
            .get("known_codes_sample")
            .and_then(|v| v.as_array())
            .expect("known_codes_sample");
        assert!(!sample.is_empty());
    });
}

#[test]
fn mcp_explain_with_invalid_model_lists_instances_when_diagnostics_exist() {
    with_isolated_data_dir(|| {
        let path = invalid_fixture_path();
        let path = path.canonicalize().unwrap_or(path);
        let check = handle_spec42_check(json!({
            "path": path.display().to_string(),
        }))
        .expect("check invalid fixture");
        let has_errors = check
            .get("summary")
            .and_then(|s| s.get("error_count"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            > 0;
        if !has_errors {
            return;
        }
        let first_code = check
            .get("documents")
            .and_then(|d| d.as_array())
            .and_then(|docs| docs.first())
            .and_then(|doc| doc.get("diagnostics"))
            .and_then(|d| d.as_array())
            .and_then(|diags| diags.first())
            .and_then(|diag| diag.get("code"))
            .and_then(|c| c.as_str())
            .expect("diagnostic code on invalid fixture");
        let explain = handle_spec42_explain_diagnostic(json!({
            "code": first_code,
            "path": path.display().to_string(),
        }))
        .expect("explain with path");
        let instances = explain
            .get("instances")
            .and_then(|v| v.as_array())
            .expect("instances");
        assert!(
            !instances.is_empty(),
            "expected instances for code {first_code}"
        );
    });
}

#[test]
fn mcp_spec42_check_include_semantic_model() {
    with_isolated_data_dir(|| {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);
        let value = handle_spec42_check(json!({
            "path": path.display().to_string(),
            "include_semantic_model": true,
        }))
        .expect("spec42_check with semantics");
        assert!(value.get("semantic_model").is_some());
        assert!(value.get("validation").is_some());
    });
}
