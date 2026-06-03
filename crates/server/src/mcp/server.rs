use std::path::PathBuf;
use std::sync::Arc;

use crate::cli::{CheckArgs, Cli, OutputFormat};
use crate::mcp::schemas::Spec42CheckParams;
use crate::perform_check;
use rmcp::model::InitializeResult;
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, ServiceExt};
use serde_json::{Map, Value};
use tokio::io::{stdin, stdout};

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

fn schema_to_map(schema: Value) -> Arc<Map<String, Value>> {
    match schema {
        Value::Object(map) => {
            if map.is_empty() {
                let mut empty_map = Map::new();
                empty_map.insert("type".to_string(), Value::String("object".to_string()));
                empty_map.insert("properties".to_string(), Value::Object(Map::new()));
                Arc::new(empty_map)
            } else {
                Arc::new(map)
            }
        }
        _ => {
            let mut map = Map::new();
            map.insert("type".to_string(), Value::String("object".to_string()));
            map.insert("properties".to_string(), Value::Object(Map::new()));
            Arc::new(map)
        }
    }
}

fn handle_spec42_check(arguments: Value) -> Result<Value, String> {
    let params: Spec42CheckParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_check: {e}"))?;

    let cli = Cli {
        config_path: params.config_path.map(PathBuf::from),
        library_paths: params
            .library_paths
            .unwrap_or_default()
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        stdlib_path: params.stdlib_path.map(PathBuf::from),
        no_stdlib: params.no_stdlib,
        stdio: false,
        command: None,
    };

    let check_args = CheckArgs {
        path: PathBuf::from(&params.path),
        workspace_root: params.workspace_root.map(PathBuf::from),
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
    };

    let report = perform_check(&cli, &check_args)?;
    serde_json::to_value(&report).map_err(|e| format!("Failed to serialize validation report: {e}"))
}

impl ServerHandler for Spec42McpServer {
    fn get_info(&self) -> InitializeResult {
        use rmcp::model::ProtocolVersion;
        InitializeResult {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability {
                    list_changed: None,
                }),
                ..Default::default()
            },
            server_info: rmcp::model::Implementation {
                name: "spec42-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: Some("Spec42 MCP".into()),
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "Spec42 MCP validates SysML v2 and KerML models. Call spec42_check with path set to a .sysml or .kerml file or directory (same as the spec42 CLI check command). Optionally pass workspace_root, config_path, library_paths, stdlib_path, or no_stdlib when you need parity with CLI global flags.".into()),
        }
    }

    async fn list_tools(
        &self,
        _paginated: Option<rmcp::model::PaginatedRequestParam>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        use rmcp::model::Tool;

        let schema_value =
            serde_json::to_value(schemars::schema_for!(Spec42CheckParams)).unwrap_or_default();

        let tools = vec![Tool {
            name: "spec42_check".into(),
            title: Some("Validate SysML / KerML (spec42 check)".into()),
            description: Some(
                "Run the same validation pipeline as `spec42 check` on a file or directory. Returns a JSON ValidationReport (documents, diagnostics, summary, advice). Use summary.error_count to detect failed validation; the tool still succeeds at the MCP layer when the report is produced.".into(),
            ),
            input_schema: schema_to_map(schema_value),
            output_schema: None,
            annotations: None,
            icons: None,
            meta: None,
        }];

        Ok(rmcp::model::ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let name = &params.name;
        let arguments_map = params.arguments.unwrap_or_default();
        let arguments = Value::Object(arguments_map);

        let result = match name.as_ref() {
            "spec42_check" => handle_spec42_check(arguments),
            _ => Err(format!("Unknown tool: {name}")),
        };

        match result {
            Ok(content) => {
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
            Err(e) => {
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
                    "Call list_tools and use the spec42_check tool name."
                } else {
                    "Verify path exists, run spec42 doctor if library resolution fails, and retry with optional workspace_root or library_paths."
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
        }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let transport = (stdin(), stdout());
    let service = Spec42McpServer::new();
    let server = service.serve(transport).await?;
    server.waiting().await?;
    Ok(())
}
