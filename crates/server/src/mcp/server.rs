use rmcp::model::InitializeResult;
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, ServiceExt};
use serde_json::{Map, Value};
use tokio::io::{stdin, stdout};

use crate::mcp::handlers::{
    handle_spec42_check, handle_spec42_doctor, handle_spec42_explain_diagnostic,
    handle_spec42_model_summary,
};
use crate::mcp::schemas::{
    Spec42CheckParams, Spec42DoctorParams, Spec42ExplainDiagnosticParams, Spec42ModelSummaryParams,
};

/// Registered MCP tool names (order matches [`build_mcp_tools`]).
pub const MCP_TOOL_NAMES: &[&str] = &[
    "spec42_check",
    "spec42_doctor",
    "spec42_model_summary",
    "spec42_explain_diagnostic",
];

pub struct Spec42McpServer;

impl Default for Spec42McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Spec42McpServer {
    pub fn new() -> Self {
        Self
    }
}

fn schema_to_map(schema: Value) -> std::sync::Arc<Map<String, Value>> {
    match schema {
        Value::Object(map) => {
            if map.is_empty() {
                let mut empty_map = Map::new();
                empty_map.insert("type".to_string(), Value::String("object".to_string()));
                empty_map.insert("properties".to_string(), Value::Object(Map::new()));
                std::sync::Arc::new(empty_map)
            } else {
                std::sync::Arc::new(map)
            }
        }
        _ => {
            let mut map = Map::new();
            map.insert("type".to_string(), Value::String("object".to_string()));
            map.insert("properties".to_string(), Value::Object(Map::new()));
            std::sync::Arc::new(map)
        }
    }
}

const MCP_INSTRUCTIONS: &str = "Spec42 MCP supports SysML v2 and KerML workflows for AI assistants. \
Recommended order: (1) spec42_doctor for environment/stdlib/library paths, \
(2) spec42_check on a .sysml/.kerml file or directory (use summary.error_count and diagnostics[].code), \
(3) spec42_explain_diagnostic for a diagnostic code, optionally with path/line for concrete instances, \
(4) spec42_model_summary for a compact semantic graph (nodes + typing/connection/reference edges). \
Pass workspace_root when validating a single file in a multi-file project. \
Global flags: config_path, library_paths, stdlib_path, no_stdlib.";

fn build_mcp_tools() -> Vec<rmcp::model::Tool> {
    use rmcp::model::Tool;

    vec![
        Tool {
            name: MCP_TOOL_NAMES[0].into(),
            title: Some("Validate SysML / KerML (spec42 check)".into()),
            description: Some(
                "Run the same validation pipeline as `spec42 check`. Returns JSON with documents, \
                diagnostics (code, message, range), summary (error_count, warning_count), and advice. \
                Set include_semantic_model=true only when you need the full semantic projection; \
                prefer spec42_model_summary for large workspaces.".into(),
            ),
            input_schema: schema_to_map(
                serde_json::to_value(schemars::schema_for!(Spec42CheckParams)).unwrap_or_default(),
            ),
            output_schema: None,
            annotations: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: MCP_TOOL_NAMES[1].into(),
            title: Some("Spec42 environment doctor".into()),
            description: Some(
                "Report standard library installation, config/data dirs, library paths, and Sysand \
                detection (same as `spec42 doctor`). Run before blaming unresolved imports on model text.".into(),
            ),
            input_schema: schema_to_map(
                serde_json::to_value(schemars::schema_for!(Spec42DoctorParams)).unwrap_or_default(),
            ),
            output_schema: None,
            annotations: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: MCP_TOOL_NAMES[2].into(),
            title: Some("Compact semantic model summary".into()),
            description: Some(
                "Validate the path and return a capped semantic graph: nodes (qualified names, kinds) \
                and relationships filtered to typing, connection, and reference. Use max_nodes to limit payload size.".into(),
            ),
            input_schema: schema_to_map(
                serde_json::to_value(schemars::schema_for!(Spec42ModelSummaryParams))
                    .unwrap_or_default(),
            ),
            output_schema: None,
            annotations: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: MCP_TOOL_NAMES[3].into(),
            title: Some("Explain a diagnostic code".into()),
            description: Some(
                "Deterministic catalog entry for a diagnostic code (severity, meaning, typical fix, \
                editor quick-fix hints). Optionally pass path and line to list matching instances from spec42_check.".into(),
            ),
            input_schema: schema_to_map(
                serde_json::to_value(schemars::schema_for!(Spec42ExplainDiagnosticParams))
                    .unwrap_or_default(),
            ),
            output_schema: None,
            annotations: None,
            icons: None,
            meta: None,
        },
    ]
}

fn dispatch_tool_handler(name: &str, arguments: Value) -> Result<Value, String> {
    match name {
        "spec42_check" => handle_spec42_check(arguments),
        "spec42_doctor" => handle_spec42_doctor(arguments),
        "spec42_model_summary" => handle_spec42_model_summary(arguments),
        "spec42_explain_diagnostic" => handle_spec42_explain_diagnostic(arguments),
        _ => Err(format!("Unknown tool: {name}")),
    }
}

/// Execute an MCP tool by name (used by [`Spec42McpServer::call_tool`] and integration tests).
pub fn execute_tool(
    name: &str,
    arguments: Value,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    match dispatch_tool_handler(name, arguments) {
        Ok(content) => tool_success(content),
        Err(e) => tool_error(name, e),
    }
}

impl ServerHandler for Spec42McpServer {
    fn get_info(&self) -> InitializeResult {
        use rmcp::model::ProtocolVersion;
        InitializeResult {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: rmcp::model::Implementation {
                name: "spec42-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: Some("Spec42 MCP".into()),
                website_url: Some("https://github.com/elan8/spec42".into()),
                icons: None,
            },
            instructions: Some(MCP_INSTRUCTIONS.into()),
        }
    }

    async fn list_tools(
        &self,
        _paginated: Option<rmcp::model::PaginatedRequestParam>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        Ok(rmcp::model::ListToolsResult {
            tools: build_mcp_tools(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let arguments = Value::Object(params.arguments.unwrap_or_default());
        execute_tool(params.name.as_ref(), arguments)
    }
}

fn tool_success(content: Value) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    let content_str = serde_json::to_string(&content).unwrap_or_default();
    Ok(rmcp::model::CallToolResult {
        content: vec![rmcp::model::Annotated {
            raw: rmcp::model::RawContent::Text(rmcp::model::RawTextContent {
                text: content_str,
                meta: None,
            }),
            annotations: None,
        }],
        is_error: Some(false),
        meta: None,
        structured_content: Some(content),
    })
}

fn tool_error(name: &str, e: String) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    let is_unknown_tool = e.starts_with("Unknown tool:");
    let error_code = if is_unknown_tool { -32601 } else { -32603 };
    let example_retry = if is_unknown_tool {
        None
    } else {
        Some(serde_json::json!({
            "path": "examples/timer/KitchenTimer.sysml"
        }))
    };
    let suggestion = if is_unknown_tool {
        "Call list_tools. Available tools: spec42_doctor, spec42_check, spec42_explain_diagnostic, spec42_model_summary."
    } else {
        "Verify path exists, run spec42_doctor if library resolution fails, and retry with optional workspace_root or library_paths."
    };
    let error_content = serde_json::json!({
        "error": {
            "code": error_code,
            "message": if is_unknown_tool { e.clone() } else { format!("{name} failed") },
            "cause": e,
            "suggestion": suggestion,
            "example_retry": example_retry,
        }
    });
    let error_str = serde_json::to_string(&error_content).unwrap_or_else(|_| {
        format!("{{\"error\":{{\"code\":{error_code},\"message\":\"{name} failed\"}}}}")
    });
    Ok(rmcp::model::CallToolResult {
        content: vec![rmcp::model::Annotated {
            raw: rmcp::model::RawContent::Text(rmcp::model::RawTextContent {
                text: error_str,
                meta: None,
            }),
            annotations: None,
        }],
        is_error: Some(true),
        meta: None,
        structured_content: Some(error_content),
    })
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let transport = (stdin(), stdout());
    let service = Spec42McpServer::new();
    let server = service.serve(transport).await?;
    server.waiting().await?;
    Ok(())
}
