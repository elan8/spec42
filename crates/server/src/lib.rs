//! Spec42 CLI and MCP shared implementation.

pub mod ai_tools;
pub mod api;
pub mod cli;
pub mod diagrams;
pub mod elk_layout;
pub mod environment;
pub mod mcp;
pub mod reports;
pub mod stdlib;
pub mod sysand;

use std::process::ExitCode;
use std::sync::Arc;

use ai_tools::{perform_explain_diagnostic, perform_model_summary};
use cli::{
    CheckArgs, Cli, Command, DiagramsCommand, DoctorArgs, ExplainDiagnosticArgs, ModelSummaryArgs,
    OutputFormat, StdlibCommand, SysandCommand,
};
use environment::{build_doctor_report, resolve_environment};
pub use environment::DoctorReport;
use kernel::{
    validate_paths, validate_paths_with_semantics, SemanticModelNode, SemanticModelProjection,
    SemanticModelRelationship, SemanticValidationReport, ValidationReport, ValidationRequest,
    ValidationSummary,
};
use mcp::schemas::Spec42GlobalParams;
use reports::{apply_baseline, emit_validation_report};
use serde::Serialize;
use std::path::PathBuf;
use stdlib::{managed_status, remove_standard_library};

/// Run validation for the given CLI environment and [`CheckArgs`] (same logic as `spec42 check`).
pub fn perform_check(cli: &Cli, args: &CheckArgs) -> Result<ValidationReport, String> {
    let references_stdlib = environment::workspace_references_standard_library(&args.path);
    let environment = resolve_environment(cli)?;
    let config = Arc::new(kernel::default_server_config());
    let mut report = validate_paths(
        &config,
        ValidationRequest {
            targets: vec![args.path.clone()],
            workspace_root: args.workspace_root.clone(),
            library_paths: environment.library_paths.clone(),
            parallel_enabled: true,
            strict_diagnostics: args.strict_diagnostics,
        },
    )?;
    if references_stdlib
        && environment.stdlib_path.is_none()
        && !cli.no_stdlib
        && !report
            .advice
            .iter()
            .any(|line| line.contains("standard library"))
    {
        report.advice.push(
            "This workspace references standard-library packages (for example ScalarValues or ISQ); run with the embedded/bundled standard library available or pass `--stdlib-path`."
                .to_string(),
        );
    }
    Ok(report)
}

/// Build a CLI value from MCP global parameters.
pub fn cli_from_global(global: &Spec42GlobalParams) -> Cli {
    Cli {
        config_path: global.config_path.as_ref().map(PathBuf::from),
        library_paths: global
            .library_paths
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        stdlib_path: global.stdlib_path.as_ref().map(PathBuf::from),
        no_stdlib: global.no_stdlib,
        stdio: false,
        command: None,
    }
}

/// Environment report (same as `spec42 doctor`).
pub fn perform_doctor(cli: &Cli) -> Result<DoctorReport, String> {
    let environment = resolve_environment(cli)?;
    build_doctor_report("doctor", &environment)
}

/// Validation with semantic graph projection (used by MCP model summary).
pub fn perform_check_with_semantics(
    cli: &Cli,
    args: &CheckArgs,
) -> Result<SemanticValidationReport, String> {
    let references_stdlib = environment::workspace_references_standard_library(&args.path);
    let environment = resolve_environment(cli)?;
    let config = Arc::new(kernel::default_server_config());
    let mut report = validate_paths_with_semantics(
        &config,
        ValidationRequest {
            targets: vec![args.path.clone()],
            workspace_root: args.workspace_root.clone(),
            library_paths: environment.library_paths.clone(),
            parallel_enabled: true,
            strict_diagnostics: args.strict_diagnostics,
        },
    )?;
    if references_stdlib
        && environment.stdlib_path.is_none()
        && !cli.no_stdlib
        && !report
            .validation
            .advice
            .iter()
            .any(|line| line.contains("standard library"))
    {
        report.validation.advice.push(
            "This workspace references standard-library packages (for example ScalarValues or ISQ); run with the embedded/bundled standard library available or pass `--stdlib-path`."
                .to_string(),
        );
    }
    Ok(report)
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelSummaryTruncation {
    pub nodes_total: usize,
    pub nodes_returned: usize,
    pub relationships_total: usize,
    pub relationships_returned: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelSummaryResponse {
    pub workspace_root: Option<String>,
    pub summary: ValidationSummary,
    pub nodes: Vec<SemanticModelNode>,
    pub relationships: Vec<SemanticModelRelationship>,
    pub truncation: ModelSummaryTruncation,
}

const SUMMARY_RELATIONSHIP_KINDS: &[&str] = &["typing", "connection", "reference"];

/// Compact semantic projection for agents (caps node count, filters relationship kinds).
pub fn build_model_summary(
    report: SemanticValidationReport,
    max_nodes: usize,
) -> ModelSummaryResponse {
    let nodes_total = report.semantic_model.nodes.len();
    let nodes: Vec<_> = report
        .semantic_model
        .nodes
        .into_iter()
        .take(max_nodes)
        .collect();

    let filtered: Vec<_> = report
        .semantic_model
        .relationships
        .into_iter()
        .filter(|rel| SUMMARY_RELATIONSHIP_KINDS.contains(&rel.kind.as_str()))
        .collect();
    let relationships_total = filtered.len();
    let relationships: Vec<_> = filtered
        .into_iter()
        .take(max_nodes.saturating_mul(2))
        .collect();

    ModelSummaryResponse {
        workspace_root: report.validation.workspace_root,
        summary: report.validation.summary,
        truncation: ModelSummaryTruncation {
            nodes_total,
            nodes_returned: nodes.len(),
            relationships_total,
            relationships_returned: relationships.len(),
        },
        nodes,
        relationships,
    }
}

/// Main CLI dispatcher (without panic handling): used by both the `spec42` binary and tests.
pub async fn run_cli(cli: Cli) -> Result<ExitCode, String> {
    if cli.stdio && cli.command.is_none() {
        return run_lsp(&cli).await;
    }
    if let Some(Command::Api { command }) = cli.command.clone() {
        return run_api(cli, &command).await;
    }
    match cli.command.as_ref() {
        None => run_lsp(&cli).await,
        Some(Command::Lsp) => run_lsp(&cli).await,
        Some(Command::Check(args)) => run_check(&cli, args),
        Some(Command::Doctor(args)) => run_doctor(&cli, args),
        Some(Command::ExplainDiagnostic(args)) => run_explain_diagnostic(&cli, args),
        Some(Command::ModelSummary(args)) => run_model_summary(&cli, args),
        Some(Command::Sysand { command }) => run_sysand(command),
        Some(Command::Stdlib { command }) => run_stdlib(&cli, command),
        Some(Command::Diagrams { command }) => run_diagrams(&cli, command),
        Some(Command::Api { .. }) => unreachable!("api command handled above"),
    }
}

async fn run_api(cli: Cli, command: &cli::ApiCommand) -> Result<ExitCode, String> {
    match command {
        cli::ApiCommand::Serve(args) => {
            api::run_api_serve(cli, args.clone()).await?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

async fn run_lsp(cli: &Cli) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let config = Arc::new(
        kernel::default_server_config()
            .with_default_library_paths(environment.library_paths.clone()),
    );
    kernel::run_lsp(config, "spec42").await;
    Ok(ExitCode::SUCCESS)
}

fn run_check(cli: &Cli, args: &CheckArgs) -> Result<ExitCode, String> {
    let report = perform_check(cli, args)?;
    let report = if let Some(baseline) = &args.baseline {
        apply_baseline(&report, baseline.as_path())?
    } else {
        report
    };

    emit_validation_report(&report, args.format)?;

    let failed = report.summary.error_count > 0
        || (args.warnings_as_errors && report.summary.warning_count > 0);

    Ok(if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn run_explain_diagnostic(cli: &Cli, args: &ExplainDiagnosticArgs) -> Result<ExitCode, String> {
    let response = perform_explain_diagnostic(
        cli,
        &ai_tools::ExplainDiagnosticArgs {
            code: args.code.clone(),
            path: args.path.clone(),
            workspace_root: args.workspace_root.clone(),
            line: args.line,
        },
    )?;
    match args.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&response).map_err(|err| {
                    format!("Failed to serialize explain-diagnostic response as JSON: {err}")
                })?
            );
        }
        OutputFormat::Text => print_explain_diagnostic(&response),
        other => {
            return Err(format!(
                "explain-diagnostic supports text and json output, not {other:?}."
            ));
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn run_model_summary(cli: &Cli, args: &ModelSummaryArgs) -> Result<ExitCode, String> {
    let summary = perform_model_summary(
        cli,
        &ai_tools::ModelSummaryArgs {
            path: args.path.clone(),
            workspace_root: args.workspace_root.clone(),
            max_nodes: args.max_nodes,
        },
    )?;
    match args.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&summary)
                    .map_err(|err| format!("Failed to serialize model-summary as JSON: {err}"))?
            );
        }
        OutputFormat::Text => print_model_summary(&summary),
        other => {
            return Err(format!(
                "model-summary supports text and json output, not {other:?}."
            ));
        }
    }
    Ok(ExitCode::SUCCESS)
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
        other => {
            return Err(format!(
                "Doctor supports text and json output, not {other:?}."
            ))
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn run_sysand(command: &SysandCommand) -> Result<ExitCode, String> {
    match command {
        SysandCommand::Status(args) => {
            let status = sysand::detect_sysand_status();
            match args.format {
                OutputFormat::Json => println!(
                    "{}",
                    serde_json::to_string_pretty(&status)
                        .map_err(|err| format!("Failed to serialize Sysand status: {err}"))?
                ),
                OutputFormat::Text => print_sysand_status(&status),
                other => {
                    return Err(format!(
                        "Sysand status supports text and json output, not {other:?}."
                    ))
                }
            }
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn run_diagrams(cli: &Cli, command: &DiagramsCommand) -> Result<ExitCode, String> {
    match command {
        DiagramsCommand::Export(args) => {
            let environment = resolve_environment(cli)?;
            let summary = diagrams::export_diagrams(args, &environment.library_paths)?;
            println!(
                "Exported {} diagram artifact(s) to {}",
                summary.exported,
                summary.output_dir.display()
            );
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn run_stdlib(cli: &Cli, command: &StdlibCommand) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let mut config = environment.standard_library.clone();
    match command {
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
            if let Some(path) = environment.stdlib_path.clone() {
                println!("{}", path.display());
                return Ok(ExitCode::SUCCESS);
            }
            let status = managed_status(&environment.standard_library_paths, &config)?;
            if status.is_installed {
                if let Some(path) = status.install_path {
                    println!("{path}");
                    return Ok(ExitCode::SUCCESS);
                }
            }
            return Err(
                "No standard library path is currently configured or installed.".to_string(),
            );
        }
        StdlibCommand::ClearCache => {
            let removed = remove_standard_library(&environment.standard_library_paths)?;
            if removed {
                println!(
                    "Cleared materialized standard library data from the spec42 data directory."
                );
            } else {
                println!("No materialized standard library data was found.");
            }
        }
    }
    Ok(ExitCode::SUCCESS)
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
        "resolved domain libraries: {}",
        report
            .resolved_domain_libraries_path
            .as_deref()
            .unwrap_or("(none)")
    );
    println!(
        "managed stdlib ready: {}",
        if report.standard_library_status.is_installed {
            "yes"
        } else {
            "no"
        }
    );
    if let Some(message) = &report.standard_library_status.status_message {
        println!("managed stdlib status: {message}");
    }
    println!("library paths:");
    for path in &report.library_paths {
        println!(
            "  - {} ({})",
            path.path,
            if path.exists { "exists" } else { "missing" }
        );
    }
    println!(
        "sysand: {}",
        if report.sysand.installed {
            "installed"
        } else {
            "not installed"
        }
    );
    if let Some(root) = &report.sysand.project_root {
        println!("sysand project: {root}");
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
    if let Some(message) = &status.status_message {
        println!("status: {message}");
    }
}

fn print_explain_diagnostic(response: &ai_tools::ExplainDiagnosticResponse) {
    println!("code: {}", response.code);
    if let Some(catalog) = &response.catalog {
        println!("severity: {}", catalog.severity);
        println!("alignment: {}", catalog.alignment);
        println!("meaning: {}", catalog.meaning);
        println!("typical fix: {}", catalog.typical_fix);
    } else {
        println!("(no catalog entry for this code)");
    }
    if !response.instances.is_empty() {
        println!("instances:");
        for inst in &response.instances {
            println!(
                "  {}:{}:{} — {}",
                inst.uri, inst.line, inst.character, inst.message
            );
        }
    }
}

fn print_model_summary(summary: &ModelSummaryResponse) {
    println!(
        "summary: {} error(s), {} warning(s), {} info",
        summary.summary.error_count,
        summary.summary.warning_count,
        summary.summary.information_count
    );
    println!(
        "nodes: {}/{} (truncated)",
        summary.truncation.nodes_returned, summary.truncation.nodes_total
    );
    println!(
        "relationships: {}/{}",
        summary.truncation.relationships_returned, summary.truncation.relationships_total
    );
}

fn print_sysand_status(status: &sysand::SysandStatus) {
    println!(
        "sysand: {}",
        if status.installed {
            "installed"
        } else {
            "not installed"
        }
    );
    println!(
        "executable: {}",
        status.executable_path.as_deref().unwrap_or("(none)")
    );
    println!(
        "version: {}",
        status.version.as_deref().unwrap_or("(unknown)")
    );
    println!(
        "project root: {}",
        status.project_root.as_deref().unwrap_or("(none)")
    );
    println!("manifest present: {}", status.manifest_present);
    println!("lock present: {}", status.lock_present);
    println!("dependency roots:");
    for root in &status.dependency_roots {
        println!("  - {root}");
    }
    for warning in &status.warnings {
        println!("warning: {warning}");
    }
}
