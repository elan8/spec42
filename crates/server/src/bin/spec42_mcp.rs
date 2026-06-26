use lsp_server::host::logging::init_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    spec42::mcp::server::run().await
}
