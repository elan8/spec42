//! MCP tool handlers (same logic as spec42-mcp stdio server).

use std::path::PathBuf;

use serde_json::json;
use spec42::mcp::handlers::{
    handle_spec42_check, handle_spec42_doctor, handle_spec42_explain_diagnostic,
    handle_spec42_model_summary,
};

fn with_isolated_data_dir(test: impl FnOnce()) {
    let data_dir = tempfile::TempDir::new().expect("temp data dir");
    let previous = std::env::var_os("SPEC42_DATA_DIR");
    std::env::set_var("SPEC42_DATA_DIR", data_dir.path());
    test();
    match previous {
        Some(value) => std::env::set_var("SPEC42_DATA_DIR", value),
        None => std::env::remove_var("SPEC42_DATA_DIR"),
    }
}

fn kitchen_timer_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml")
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
        let nodes = value.get("nodes").and_then(|v| v.as_array()).expect("nodes");
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
