//! HTTP API integration and parity tests.

mod common;

use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use clap::Parser;
use common::with_isolated_data_dir_async;
use http_body_util::BodyExt;
use serde_json::Value;
use spec42::api::{router, ApiServerState};
use spec42::cli::Cli;
use spec42::environment::resolve_environment;
use spec42::mcp::handlers::{handle_spec42_explain_diagnostic, handle_spec42_model_summary};
use tower::ServiceExt;

fn kitchen_timer_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml")
}

fn example_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer")
}

async fn build_test_state(workspace_root: PathBuf) -> Arc<ApiServerState> {
    let cli = Cli::parse_from(["spec42"]);
    let environment = resolve_environment(&cli).expect("environment");
    let workspace_root = workspace_root.canonicalize().unwrap_or(workspace_root);
    Arc::new(ApiServerState {
        config: Arc::new(
            lsp_server::default_server_config()
                .with_default_library_paths(environment.library_paths.clone()),
        ),
        cli,
        workspace_root,
        environment,
    })
}

async fn request_json(app: &axum::Router, uri: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).to_string()))
    };
    (status, value)
}

async fn post_json(app: &axum::Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let value = serde_json::from_slice(&bytes).expect("json body");
    (status, value)
}

async fn post_raw(app: &axum::Router, uri: &str, body: Value) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (status, String::from_utf8_lossy(&bytes).to_string())
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_health_returns_ok() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = request_json(&app, "/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("ok"));
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_ready_returns_workspace_root() {
    with_isolated_data_dir_async(|| async {
        let workspace = example_workspace_root();
        let state = build_test_state(workspace.clone()).await;
        let app = router(state);
        let (status, body) = request_json(&app, "/ready").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("ready"));
        assert!(body
            .get("workspace_root")
            .and_then(|v| v.as_str())
            .is_some());
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_doctor_returns_version() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = request_json(&app, "/v1/doctor").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.get("version").and_then(|v| v.as_str()).is_some());
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_validate_workspace() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = post_json(
            &app,
            "/v1/validate",
            serde_json::json!({ "path": "KitchenTimer.sysml" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.get("summary").is_some());
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_model_summary_matches_mcp() {
    with_isolated_data_dir_async(|| async {
        let timer = kitchen_timer_path();
        let timer = timer.canonicalize().unwrap_or(timer);
        let relative = "KitchenTimer.sysml".to_string();

        let mcp = handle_spec42_model_summary(serde_json::json!({
            "path": timer.display().to_string(),
            "max_nodes": 1,
        }))
        .expect("mcp model summary");

        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = post_json(
            &app,
            "/v1/model/summary",
            serde_json::json!({ "path": relative, "max_nodes": 1 }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body.get("truncation")
                .and_then(|t| t.get("nodes_returned"))
                .and_then(|v| v.as_u64()),
            mcp.get("truncation")
                .and_then(|t| t.get("nodes_returned"))
                .and_then(|v| v.as_u64())
        );
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_diagram_svg_uses_shared_renderer() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = post_raw(
            &app,
            "/v1/diagrams/export",
            serde_json::json!({
                "path": "KitchenTimer.sysml",
                "view": "general-view",
                "format": "svg"
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("viz-node--"));
        assert!(body.contains("general-d3-"));
        assert!(!body.contains("data-layout-engine=\"elkjs-quickjs\""));
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_explain_diagnostic_matches_mcp() {
    with_isolated_data_dir_async(|| async {
        let mcp = handle_spec42_explain_diagnostic(serde_json::json!({
            "code": "unresolved_type_reference",
        }))
        .expect("mcp explain");

        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) =
            request_json(&app, "/v1/diagnostics/explain/unresolved_type_reference").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body.get("code").and_then(|v| v.as_str()),
            mcp.get("code").and_then(|v| v.as_str())
        );
        assert_eq!(
            body.get("catalog")
                .and_then(|c| c.get("code"))
                .and_then(|v| v.as_str()),
            mcp.get("catalog")
                .and_then(|c| c.get("code"))
                .and_then(|v| v.as_str())
        );
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_openapi_json_loads() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = request_json(&app, "/openapi.json").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.get("openapi").and_then(|v| v.as_str()), Some("3.1.0"));
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_rejects_path_traversal() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = post_json(
            &app,
            "/v1/validate",
            serde_json::json!({ "path": "../outside" }),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body.get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("invalid_path")
        );
    })
    .await;
}

#[tokio::test]
#[ignore = "agent/API/MCP integration; run: cargo test -p spec42 -- --include-ignored"]
async fn api_diagnostic_codes_non_empty() {
    with_isolated_data_dir_async(|| async {
        let state = build_test_state(example_workspace_root()).await;
        let app = router(state);
        let (status, body) = request_json(&app, "/v1/diagnostics/codes").await;
        assert_eq!(status, StatusCode::OK);
        let codes = body.get("codes").and_then(|v| v.as_array()).expect("codes");
        assert!(!codes.is_empty());
    })
    .await;
}
