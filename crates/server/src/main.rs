use std::process::ExitCode;

use clap::Parser;
use lsp_server::host::logging::init_tracing;
use spec42::cli::Cli;
use spec42::run_cli;

/// Larger than the OS-provided main thread stack (1 MiB on Windows -- vs. 8 MiB typical on Linux/
/// macOS) so ordinary, non-adversarial parsing work can't overflow it. Debug builds in particular
/// don't inline nom's heavily-generic combinator chains, so every grammar production is a real
/// stack frame; a real, valid SysML file with only a few levels of structural nesting has been
/// observed to overflow Windows' 1 MiB default well before hitting any actual depth limit. Rust
/// threads spawned via `std::thread` (including tokio's own worker/blocking pools) already default
/// to a larger stack than that, so this crash is specific to whichever thread the OS itself started
/// the process on -- work around it by never running real work there.
const MAIN_STACK_SIZE: usize = 64 * 1024 * 1024;

fn main() -> ExitCode {
    std::thread::Builder::new()
        .name("spec42-main".into())
        .stack_size(MAIN_STACK_SIZE)
        .spawn(run)
        .expect("failed to spawn spec42 main thread")
        .join()
        .expect("spec42 main thread panicked")
}

fn run() -> ExitCode {
    let runtime = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    runtime.block_on(async {
        init_tracing();
        let cli = Cli::parse();
        match run_cli(cli).await {
            Ok(code) => code,
            Err(message) => {
                eprintln!("{message}");
                ExitCode::from(2)
            }
        }
    })
}
