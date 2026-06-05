//! Subprocess smoke for the `spec42-mcp` binary (stdio transport).

mod common;

use common::with_isolated_data_dir_async;
use rmcp::model::CallToolRequestParam;
use rmcp::transport::TokioChildProcess;
use rmcp::ServiceExt;

#[tokio::test]
async fn spec42_mcp_binary_lists_tools_and_doctor() -> anyhow::Result<()> {
    with_isolated_data_dir_async(|| async {
        let bin = env!("CARGO_BIN_EXE_spec42-mcp");
        let transport = TokioChildProcess::new(tokio::process::Command::new(bin))?;

        let client = ().serve(transport).await?;
        let tools = client.list_all_tools().await?;
        let names: Vec<String> = tools.into_iter().map(|t| t.name.to_string()).collect();
        for expected in spec42::mcp::server::MCP_TOOL_NAMES {
            assert!(
                names.iter().any(|n| n == expected),
                "binary MCP missing tool {expected} in {names:?}"
            );
        }

        let doctor = client
            .call_tool(CallToolRequestParam {
                name: "spec42_doctor".into(),
                arguments: Some(serde_json::Map::new()),
            })
            .await?;
        assert_ne!(doctor.is_error, Some(true));
        let structured = doctor.structured_content.expect("doctor json");
        assert!(structured.get("version").and_then(|v| v.as_str()).is_some());

        client.cancel().await?;
        anyhow::Ok(())
    })
    .await
}
