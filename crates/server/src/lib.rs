#![recursion_limit = "256"]

//! Spec42 CLI and MCP shared implementation.

pub mod ai_tools;
pub mod cli;
pub mod diagnostic_catalog;
#[cfg(test)]
pub mod elk_layout;
pub mod environment;
pub mod generation;
pub mod headless_renderer;
pub mod host_snapshot;
pub mod kpar_libraries;
pub mod library_bundle;
pub mod library_status_rpc;
pub mod reports;
pub mod starter_workspace;
pub mod stdlib;
pub mod sysand;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use ai_tools::{perform_explain_diagnostic, perform_model_summary};
use cli::{
    BundleArgs, CheckArgs, Cli, Command, DoctorArgs, ExplainDiagnosticArgs, InitArgs,
    LibrariesCommand, ModelSummaryArgs, OutputFormat, StdlibCommand, SysandCommand, UnbundleArgs,
};
pub use environment::DoctorReport;
use environment::{build_doctor_report, build_engine, resolve_environment};
use workspace::{
    validate_paths, HostValidationReport, HostValidationSummary, ValidationRequest,
};
use reports::{apply_baseline, emit_validation_report};
use serde::Serialize;
use stdlib::{managed_status, remove_standard_library};

/// Run validation for the given CLI environment and [`CheckArgs`] (same logic as `spec42 check`).
pub fn perform_check(cli: &Cli, args: &CheckArgs) -> Result<HostValidationReport, String> {
    let references_stdlib = environment::workspace_references_standard_library(&args.path);
    let environment = resolve_environment(cli)?;
    let engine = build_engine(cli)?;
    let mut report = validate_paths(
        &engine,
        &[],
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

/// Environment report (same as `spec42 doctor`).
pub fn perform_doctor(cli: &Cli) -> Result<DoctorReport, String> {
    let environment = resolve_environment(cli)?;
    build_doctor_report("doctor", &environment)
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
    pub summary: HostValidationSummary,
    pub truncation: ModelSummaryTruncation,
}

/// Narrow summary while a typed model-summary projection is defined.
pub fn build_model_summary(report: HostValidationReport, _max_nodes: usize) -> ModelSummaryResponse {
    // TODO(follow-up): expose a bounded typed summary from PublishedModel. Do not recreate the
    // retired graph DTO here; until that owner exists, diagnostics are the complete supported
    // result and semantic nodes/relationships are explicitly absent.
    ModelSummaryResponse {
        workspace_root: report.workspace_root,
        summary: report.summary,
        truncation: ModelSummaryTruncation {
            nodes_total: 0,
            nodes_returned: 0,
            relationships_total: 0,
            relationships_returned: 0,
        },
    }
}

/// Main CLI dispatcher (without panic handling): used by both the `spec42` binary and tests.
pub async fn run_cli(cli: Cli) -> Result<ExitCode, String> {
    if cli.stdio && cli.command.is_none() {
        return run_lsp(&cli).await;
    }
    match cli.command.as_ref() {
        None => run_lsp(&cli).await,
        Some(Command::Lsp) => run_lsp(&cli).await,
        Some(Command::Check(args)) => run_check(&cli, args),
        Some(Command::Init(args)) => run_init(&cli, args),
        Some(Command::Generate(args)) => generation::run_generate(&cli, args),
        Some(Command::Doctor(args)) => run_doctor(&cli, args),
        Some(Command::ExplainDiagnostic(args)) => run_explain_diagnostic(&cli, args),
        Some(Command::ModelSummary(args)) => run_model_summary(&cli, args),
        Some(Command::Bundle(args)) => run_bundle(args),
        Some(Command::Unbundle(args)) => run_unbundle(args),
        Some(Command::Sysand { command }) => run_sysand(command),
        Some(Command::Stdlib { command }) => run_stdlib(&cli, command),
        Some(Command::Libraries { command }) => run_libraries(&cli, command),
    }
}

fn run_bundle(args: &BundleArgs) -> Result<ExitCode, String> {
    if !args.directory.is_dir() {
        return Err(format!(
            "bundle source directory does not exist: {}",
            args.directory.display()
        ));
    }
    let project_path = args.directory.join(kpar::PROJECT_FILE);
    let project_bytes = std::fs::read(&project_path).map_err(|error| {
        format!(
            "bundle requires {} as the project metadata authority: {error}",
            project_path.display()
        )
    })?;
    let project: kpar::Project = serde_json::from_slice(&project_bytes).map_err(|error| {
        format!(
            "bundle could not parse required project metadata {}: {error}",
            project_path.display()
        )
    })?;
    project
        .validate_identity()
        .map_err(|error| error.to_string())?;
    let output = args
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("{}-{}.kpar", project.name, project.version)));
    kpar::build_kpar(
        &kpar::PackOptions {
            project,
            source_roots: Vec::new(),
            named_source_roots: vec![(
                (args.archive_prefix.clone().unwrap_or_default()),
                args.directory.clone(),
            )],
            excludes: args.excludes.clone(),
            timestamp: kpar::ArchiveTimestamp::default(),
            compression: if args.no_compress {
                kpar::ArchiveCompression::Stored
            } else {
                kpar::ArchiveCompression::Deflated
            },
        },
        &output,
    )
    .map_err(|error| error.to_string())?;
    println!(
        "Bundled {} into {}",
        args.directory.display(),
        output.display()
    );
    Ok(ExitCode::SUCCESS)
}

fn run_unbundle(args: &UnbundleArgs) -> Result<ExitCode, String> {
    let archive = kpar::open_kpar_path(&args.archive).map_err(|error| error.to_string())?;
    archive
        .project()
        .validate_identity()
        .map_err(|error| error.to_string())?;
    let directory = args
        .directory
        .clone()
        .unwrap_or_else(|| PathBuf::from(&archive.project().name));
    let materialized = archive
        .materialize_to(&directory)
        .map_err(|error| error.to_string())?;
    println!(
        "Unbundled {} source file(s) into {}",
        materialized.source_files.len(),
        materialized.root.display()
    );
    Ok(ExitCode::SUCCESS)
}

fn run_init(cli: &Cli, args: &InitArgs) -> Result<ExitCode, String> {
    let scaffold = starter_workspace::scaffold(&args.path)?;
    let validation_args = CheckArgs {
        path: scaffold.root.clone(),
        workspace_root: Some(scaffold.root.clone()),
        format: OutputFormat::Text,
        warnings_as_errors: false,
        baseline: None,
        strict_diagnostics: false,
    };
    let report = perform_check(cli, &validation_args)?;
    if report.summary.error_count > 0 {
        let _ = emit_validation_report(&report, OutputFormat::Text);
        return Err(format!(
            "starter workspace was created at {} but failed validation with {} error(s)",
            scaffold.root.display(),
            report.summary.error_count
        ));
    }

    println!(
        "Created starter SysML v2 workspace at {} ({} files; validation passed).",
        scaffold.root.display(),
        scaffold.files_written
    );
    Ok(ExitCode::SUCCESS)
}

async fn run_lsp(cli: &Cli) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let engine = build_engine(cli)?;
    let config = Arc::new(
        lsp_server::default_server_config()
            .with_services(engine.services().clone())
            .with_default_library_paths(environment.library_paths.clone())
            .with_standard_library_paths(environment.stdlib_roots.clone())
            .with_custom_rpc_provider(library_status_rpc::library_status_rpc_provider(
                environment.standard_library.clone(),
                environment.standard_library_paths.clone(),
                environment.stdlib_source.clone(),
                environment.stdlib_path.clone(),
                environment.kpar_libraries.clone(),
            )),
    );
    lsp_server::run_lsp(config, "spec42").await;
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

fn run_libraries(cli: &Cli, command: &LibrariesCommand) -> Result<ExitCode, String> {
    let environment = resolve_environment(cli)?;
    let selected =
        |id: &Option<String>| -> Result<Vec<&library_catalog::KparLibraryComponent>, String> {
            match id {
                None => Ok(environment.kpar_libraries.iter().collect()),
                Some(wanted) => {
                    let matches: Vec<_> = environment
                        .kpar_libraries
                        .iter()
                        .filter(|library| library.id == *wanted)
                        .collect();
                    if matches.is_empty() {
                        return Err(format!(
                            "Unknown KPAR library id '{wanted}'. Registered: {}",
                            environment
                                .kpar_libraries
                                .iter()
                                .map(|library| library.id.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                    }
                    Ok(matches)
                }
            }
        };

    match command {
        LibrariesCommand::Status(args) => {
            for library in selected(&args.id)? {
                println!("[{}] {}", library.id, library.display_name);
                match library.source.as_deref() {
                    Some("disabled") => println!("  disabled: yes (not used for resolution)"),
                    // Overrides (`flag`/`env`) and ad-hoc `custom` libraries aren't reflected
                    // in the on-disk managed metadata, so report the resolved component
                    // directly instead of falling back to `managed_status`.
                    Some("flag") | Some("env") | Some("custom") => {
                        println!("  pinned version: {}", library.config.version);
                        println!(
                            "  resolved path: {}",
                            library
                                .path
                                .as_ref()
                                .map(|path| path.display().to_string())
                                .unwrap_or_else(|| "(none)".to_string())
                        );
                        println!(
                            "  source: {}",
                            library.source.as_deref().unwrap_or("(none)")
                        );
                    }
                    _ => {
                        let status =
                            kpar_libraries::managed_status(&library.paths, &library.config)?;
                        print_kpar_library_status(&status);
                    }
                }
                println!();
            }
        }
        LibrariesCommand::Path(args) => {
            for library in selected(&args.id)? {
                if library.source.as_deref() == Some("disabled") {
                    return Err(format!(
                        "KPAR library '{}' is disabled and has no active resolution path.",
                        library.id
                    ));
                }
                if let Some(path) = &library.path {
                    if args.id.is_some() {
                        println!("{}", path.display());
                    } else {
                        println!("{}={}", library.id, path.display());
                    }
                    continue;
                }
                let status = kpar_libraries::managed_status(&library.paths, &library.config)?;
                if status.is_installed {
                    if let Some(path) = status.install_path {
                        if args.id.is_some() {
                            println!("{path}");
                        } else {
                            println!("{}={path}", library.id);
                        }
                        continue;
                    }
                }
                return Err(format!(
                    "No path is currently configured or installed for KPAR library '{}'.",
                    library.id
                ));
            }
        }
        LibrariesCommand::ClearCache(args) => {
            let mut cleared = 0usize;
            for library in selected(&args.id)? {
                if kpar_libraries::remove_kpar_library(&library.paths)? {
                    cleared += 1;
                    println!(
                        "Cleared materialized {} data from the spec42 data directory.",
                        library.display_name
                    );
                }
            }
            if cleared == 0 {
                println!("No materialized KPAR library data was found.");
            }
        }
    }
    Ok(ExitCode::SUCCESS)
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
    if !report.stdlib_roots.is_empty() {
        println!("stdlib library roots ({}):", report.stdlib_roots.len());
        for root in &report.stdlib_roots {
            println!("  - {root}");
        }
    }
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
    if report.kpar_libraries.is_empty() {
        println!("kpar libraries: (none)");
    } else {
        println!("kpar libraries:");
        for library in &report.kpar_libraries {
            println!(
                "  - {} ({}) path={} source={} ready={}",
                library.id,
                library.display_name,
                library.path.as_deref().unwrap_or("(none)"),
                library.source_kind,
                if library.status.is_installed {
                    "yes"
                } else {
                    "no"
                }
            );
            if let Some(message) = &library.status.status_message {
                println!("      status: {message}");
            }
        }
    }
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

fn print_kpar_library_status(status: &kpar_libraries::KparLibraryStatus) {
    println!("  pinned version: {}", status.pinned_version);
    println!(
        "  installed version: {}",
        status.installed_version.as_deref().unwrap_or("(none)")
    );
    println!(
        "  install path: {}",
        status.install_path.as_deref().unwrap_or("(none)")
    );
    println!(
        "  ready: {}",
        if status.is_installed { "yes" } else { "no" }
    );
    println!("  source: {}", status.source.as_deref().unwrap_or("(none)"));
    println!(
        "  canonical managed: {}",
        if status.is_canonical_managed {
            "yes"
        } else {
            "no"
        }
    );
    if let Some(message) = &status.status_message {
        println!("  status: {message}");
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
