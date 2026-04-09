mod cli;
mod environment;
mod stdlib;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use cli::{CheckArgs, Cli, Command, DoctorArgs, OutputFormat, StdlibCommand};
use environment::{build_doctor_report, resolve_environment};
use spec42_core::host::logging::init_tracing;
use spec42_core::{validate_paths, ValidationReport, ValidationRequest};
use stdlib::{
    install_standard_library, load_managed_metadata, managed_status, remove_standard_library,
};

#[tokio::main]
async fn main() -> ExitCode {
    init_tracing();
    let cli = Cli::parse();
    match run(cli).await {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

async fn run(cli: Cli) -> Result<ExitCode, String> {
    match cli.command.as_ref() {
        None => run_lsp().await,
        Some(Command::Lsp) => run_lsp().await,
        Some(Command::Check(args)) => run_check(&cli, args),
        Some(Command::Doctor(args)) => run_doctor(&cli, args),
        Some(Command::Stdlib { command }) => run_stdlib(&cli, command),
    }
}

async fn run_lsp() -> Result<ExitCode, String> {
    let config = Arc::new(spec42_core::default_server_config());
    spec42_core::run_lsp(config, "spec42").await;
    Ok(ExitCode::SUCCESS)
}

fn run_check(cli: &Cli, args: &CheckArgs) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let config = Arc::new(spec42_core::default_server_config());
    let report = validate_paths(
        &config,
        ValidationRequest {
            targets: vec![args.path.clone()],
            workspace_root: args.workspace_root.clone(),
            library_paths: environment.library_paths.clone(),
            parallel_enabled: true,
        },
    )?;

    match args.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report)
                    .map_err(|err| format!("Failed to serialize report as JSON: {err}"))?
            );
        }
        OutputFormat::Text => print_check_report(&report),
    }

    Ok(if report.summary.error_count > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn run_doctor(cli: &Cli, args: &DoctorArgs) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let report = build_doctor_report("doctor", &environment)?;
    match args.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report)
                    .map_err(|err| format!("Failed to serialize doctor report as JSON: {err}"))?
            );
        }
        OutputFormat::Text => print_doctor_report(&report),
    }
    Ok(ExitCode::SUCCESS)
}

fn run_stdlib(cli: &Cli, command: &StdlibCommand) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let mut config = environment.standard_library.clone();
    match command {
        StdlibCommand::Install(args) => {
            if let Some(version) = &args.version {
                config.version = version.clone();
            }
            if let Some(repo) = &args.repo {
                config.repo = repo.clone();
            }
            if let Some(content_path) = &args.content_path {
                config.content_path = content_path.clone();
            }
            let metadata = install_standard_library(&environment.standard_library_paths, &config)?;
            println!(
                "Installed SysML standard library {} to {}",
                metadata.installed_version, metadata.install_path
            );
        }
        StdlibCommand::Status(args) => {
            if let Some(version) = &args.version {
                config.version = version.clone();
            }
            if let Some(repo) = &args.repo {
                config.repo = repo.clone();
            }
            if let Some(content_path) = &args.content_path {
                config.content_path = content_path.clone();
            }
            let status = managed_status(&environment.standard_library_paths, &config)?;
            print_stdlib_status(&status);
        }
        StdlibCommand::Path(args) => {
            if let Some(version) = &args.version {
                config.version = version.clone();
            }
            if let Some(repo) = &args.repo {
                config.repo = repo.clone();
            }
            if let Some(content_path) = &args.content_path {
                config.content_path = content_path.clone();
            }
            let resolved_path = environment.stdlib_path.clone().or_else(|| {
                load_managed_metadata(&environment.standard_library_paths)
                    .ok()
                    .flatten()
                    .map(|metadata| PathBuf::from(metadata.install_path))
            });
            if let Some(path) = resolved_path {
                println!("{}", path.display());
                return Ok(ExitCode::SUCCESS);
            }
            return Err(
                "No standard library path is currently configured or installed.".to_string(),
            );
        }
        StdlibCommand::Remove => {
            let removed = remove_standard_library(&environment.standard_library_paths)?;
            if removed {
                println!("Removed managed SysML standard library.");
            } else {
                println!("No managed SysML standard library installation was found.");
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn print_check_report(report: &ValidationReport) {
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            let severity = diagnostic.severity.map(severity_label).unwrap_or("error");
            let code = diagnostic
                .code
                .as_ref()
                .map(number_or_string_label)
                .unwrap_or_default();
            println!(
                "{}:{}:{}: {}{}{}",
                document.uri,
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1,
                severity,
                if code.is_empty() { "" } else { " [" },
                if code.is_empty() {
                    diagnostic.message.clone()
                } else {
                    format!("{code}] {}", diagnostic.message)
                }
            );
        }
    }
    println!(
        "Checked {} document(s): {} error(s), {} warning(s), {} info(s)",
        report.summary.document_count,
        report.summary.error_count,
        report.summary.warning_count,
        report.summary.information_count
    );
    for advice in &report.advice {
        println!("Advice: {advice}");
    }
}

fn print_doctor_report(report: &environment::DoctorReport) {
    println!("spec42 {}", report.version);
    println!("mode: {}", report.mode);
    println!(
        "config file: {}",
        report.config_file_used.as_deref().unwrap_or("(none)")
    );
    println!("config dir: {}", report.config_dir);
    println!("data dir: {}", report.data_dir);
    println!(
        "resolved stdlib: {}",
        report.resolved_stdlib_path.as_deref().unwrap_or("(none)")
    );
    println!(
        "stdlib source: {}",
        report.stdlib_source.as_deref().unwrap_or("(none)")
    );
    println!("stdlib source kind: {}", report.stdlib_source_kind);
    println!(
        "legacy VS Code fallback: {}",
        if report.used_legacy_vscode_fallback {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "managed stdlib ready: {}",
        if report.standard_library_status.is_installed {
            "yes"
        } else {
            "no"
        }
    );
    println!("library paths:");
    for path in &report.library_paths {
        println!(
            "  - {} ({})",
            path.path,
            if path.exists { "exists" } else { "missing" }
        );
    }
}

fn print_stdlib_status(status: &stdlib::StandardLibraryStatus) {
    println!("pinned version: {}", status.pinned_version);
    println!(
        "installed version: {}",
        status.installed_version.as_deref().unwrap_or("(none)")
    );
    println!(
        "install path: {}",
        status.install_path.as_deref().unwrap_or("(none)")
    );
    println!("ready: {}", if status.is_installed { "yes" } else { "no" });
    println!("source: {}", status.source.as_deref().unwrap_or("(none)"));
    println!(
        "canonical managed: {}",
        if status.is_canonical_managed {
            "yes"
        } else {
            "no"
        }
    );
}

fn severity_label(severity: tower_lsp::lsp_types::DiagnosticSeverity) -> &'static str {
    match severity {
        tower_lsp::lsp_types::DiagnosticSeverity::ERROR => "error",
        tower_lsp::lsp_types::DiagnosticSeverity::WARNING => "warning",
        tower_lsp::lsp_types::DiagnosticSeverity::INFORMATION => "info",
        tower_lsp::lsp_types::DiagnosticSeverity::HINT => "hint",
        _ => "unknown",
    }
}

fn number_or_string_label(value: &tower_lsp::lsp_types::NumberOrString) -> String {
    match value {
        tower_lsp::lsp_types::NumberOrString::String(value) => value.clone(),
        tower_lsp::lsp_types::NumberOrString::Number(value) => value.to_string(),
    }
}
