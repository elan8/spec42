use std::process::ExitCode;

use clap::Parser;
use lsp_server::host::logging::init_tracing;
use spec42::cli::Cli;
use spec42::run_cli;

const CLI_STACK_SIZE: usize = 32 * 1024 * 1024;

fn main() -> ExitCode {
    init_tracing();
    let cli = Cli::parse();

    let result = std::thread::Builder::new()
        .name("spec42-cli".into())
        .stack_size(CLI_STACK_SIZE)
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|error| format!("failed to start the Spec42 runtime: {error}"))?;
            runtime.block_on(run_cli(cli))
        })
        .map_err(|error| format!("failed to start the Spec42 CLI thread: {error}"))
        .and_then(|handle| {
            handle
                .join()
                .map_err(|_| "the Spec42 CLI thread panicked".to_string())
        })
        .and_then(|result| result);

    match result {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
