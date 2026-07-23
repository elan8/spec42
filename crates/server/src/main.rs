use std::process::ExitCode;

use clap::Parser;
use lsp_server::host::logging::init_tracing;
use spec42::cli::Cli;
use spec42::run_cli;

#[tokio::main]
async fn main() -> ExitCode {
    init_tracing();
    let cli = Cli::parse();
    match run_cli(cli).await {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
