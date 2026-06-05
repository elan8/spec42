//! In-process MCP protocol smoke (duplex transport, same stack as stdio hosts).

mod common;

use std::path::PathBuf;

use common::with_isolated_data_dir_async;
use rmcp::model::{CallToolRequestParam, ClientInfo};
use rmcp::{ClientHandler, ServiceExt};
use serde_json::json;
use spec42::mcp::server::{Spec42McpServer, MCP_TOOL_NAMES};

#[derive(Debug, Clone, Default)]
struct TestMcpClient;

impl ClientHandler for TestMcpClient {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }
}

fn kitchen_timer_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml")
}

#[tokio::test]
async fn mcp_protocol_lists_four_tools() -> anyhow::Result<()> {
    with_isolated_data_dir_async(|| async {
        let (server_transport, client_transport) = tokio::io::duplex(4096);
        let server_handle = tokio::spawn(async move {
            let server = Spec42McpServer::new().serve(server_transport).await?;
            server.waiting().await?;
            anyhow::Ok(())
        });

        let client = TestMcpClient.serve(client_transport).await?;
        let tools = client.list_all_tools().await?;
        let names: Vec<String> = tools.into_iter().map(|t| t.name.to_string()).collect();
        for expected in MCP_TOOL_NAMES {
            assert!(
                names.iter().any(|n| n == expected),
                "missing tool {expected} in {names:?}"
            );
        }

        client.cancel().await?;
        server_handle.abort();
        Ok(())
    })
    .await
}

#[tokio::test]
async fn mcp_protocol_call_spec42_check() -> anyhow::Result<()> {
    with_isolated_data_dir_async(|| async {
        let path = kitchen_timer_path();
        let path = path.canonicalize().unwrap_or(path);

        let (server_transport, client_transport) = tokio::io::duplex(4096);
        let server_handle = tokio::spawn(async move {
            let server = Spec42McpServer::new().serve(server_transport).await?;
            server.waiting().await?;
            anyhow::Ok(())
        });

        let client = TestMcpClient.serve(client_transport).await?;
        let result = client
            .call_tool(CallToolRequestParam {
                name: "spec42_check".into(),
                arguments: Some(
                    json!({ "path": path.display().to_string() })
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            })
            .await?;

        assert_ne!(result.is_error, Some(true));
        let structured = result
            .structured_content
            .expect("structured_content on success");
        let summary = structured.get("summary").expect("summary");
        assert_eq!(summary.get("error_count").and_then(|v| v.as_u64()), Some(0));

        client.cancel().await?;
        server_handle.abort();
        Ok(())
    })
    .await
}

#[tokio::test]
async fn mcp_protocol_unknown_tool_returns_error_result() -> anyhow::Result<()> {
    with_isolated_data_dir_async(|| async {
        let (server_transport, client_transport) = tokio::io::duplex(4096);
        let server_handle = tokio::spawn(async move {
            let server = Spec42McpServer::new().serve(server_transport).await?;
            server.waiting().await?;
            anyhow::Ok(())
        });

        let client = TestMcpClient.serve(client_transport).await?;
        let result = client
            .call_tool(CallToolRequestParam {
                name: "nope".into(),
                arguments: None,
            })
            .await?;

        assert_eq!(result.is_error, Some(true));
        let structured = result.structured_content.expect("structured error content");
        let suggestion = structured["error"]["suggestion"]
            .as_str()
            .unwrap_or_default();
        assert!(suggestion.contains("list_tools"));

        client.cancel().await?;
        server_handle.abort();
        Ok(())
    })
    .await
}
