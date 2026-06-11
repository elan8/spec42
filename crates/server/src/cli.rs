use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command(name = "spec42", version, about = "SysML v2 language server and CLI")]
pub struct Cli {
    #[arg(long = "config", global = true)]
    pub config_path: Option<PathBuf>,
    #[arg(long = "library-path", global = true)]
    pub library_paths: Vec<PathBuf>,
    #[arg(long = "stdlib-path", global = true)]
    pub stdlib_path: Option<PathBuf>,
    #[arg(long = "domain-libraries-path", global = true)]
    pub domain_libraries_path: Option<PathBuf>,
    #[arg(long = "no-stdlib", global = true, default_value_t = false)]
    pub no_stdlib: bool,
    #[arg(long = "stdio", global = true, hide = true, default_value_t = false)]
    pub stdio: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Lsp,
    Check(CheckArgs),
    Doctor(DoctorArgs),
    /// Explain a diagnostic code (same as MCP `spec42_explain_diagnostic`).
    ExplainDiagnostic(ExplainDiagnosticArgs),
    /// Compact semantic graph summary (same as MCP `spec42_model_summary`).
    ModelSummary(ModelSummaryArgs),
    Sysand {
        #[command(subcommand)]
        command: SysandCommand,
    },
    Stdlib {
        #[command(subcommand)]
        command: StdlibCommand,
    },
    DomainLibraries {
        #[command(subcommand)]
        command: DomainLibrariesCommand,
    },
    Diagrams {
        #[command(subcommand)]
        command: DiagramsCommand,
    },
    /// Read-only HTTP API for workspace semantics.
    Api {
        #[command(subcommand)]
        command: ApiCommand,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ApiCommand {
    /// Start the read-only HTTP API server.
    Serve(crate::api::ApiServeArgs),
}

#[derive(Debug, Clone, Args)]
pub struct CheckArgs {
    pub path: PathBuf,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
    #[arg(long = "warnings-as-errors", default_value_t = false)]
    pub warnings_as_errors: bool,
    #[arg(long = "baseline")]
    pub baseline: Option<PathBuf>,
    #[arg(
        long = "strict-diagnostics",
        default_value_t = false,
        help = "Legacy check mode: skip semantic diagnostics after parse errors and suppress shadowed unresolved warnings"
    )]
    pub strict_diagnostics: bool,
}

#[derive(Debug, Clone, Args)]
pub struct DoctorArgs {
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Args)]
pub struct ExplainDiagnosticArgs {
    /// Diagnostic code (for example `unresolved_type_reference`).
    #[arg(long = "code")]
    pub code: String,
    /// Optional file or directory to list matching diagnostics from validation.
    #[arg(long = "path")]
    pub path: Option<PathBuf>,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    /// Optional 0-based line number to filter instances.
    #[arg(long = "line")]
    pub line: Option<u32>,
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
}

fn default_max_nodes() -> usize {
    500
}

#[derive(Debug, Clone, Args)]
pub struct ModelSummaryArgs {
    pub path: PathBuf,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    #[arg(long = "max-nodes", default_value_t = default_max_nodes())]
    pub max_nodes: usize,
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Args)]
pub struct StdlibStatusArgs {
    #[arg(long = "version")]
    pub version: Option<String>,
    #[arg(long = "repo")]
    pub repo: Option<String>,
    #[arg(long = "content-path")]
    pub content_path: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum StdlibCommand {
    /// Show pinned vs installed standard library metadata.
    Status(StdlibStatusArgs),
    /// Print the resolved standard library directory path.
    Path(StdlibStatusArgs),
    /// Delete materialized standard-library files from the data directory (they are re-created from the embedded copy on next use).
    ClearCache,
}

#[derive(Debug, Clone, Subcommand)]
pub enum DomainLibrariesCommand {
    /// Show pinned vs installed domain libraries metadata.
    Status(StdlibStatusArgs),
    /// Print the resolved domain libraries directory path.
    Path(StdlibStatusArgs),
    /// Delete materialized domain-library files from the data directory (they are re-created from the embedded copy on next use).
    ClearCache,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SysandCommand {
    /// Show optional Sysand package-manager integration status.
    Status(SysandStatusArgs),
}

#[derive(Debug, Clone, Args)]
pub struct SysandStatusArgs {
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Subcommand)]
pub enum DiagramsCommand {
    /// Export deterministic shared-view payloads for CI documentation workflows.
    Export(DiagramExportArgs),
}

#[derive(Debug, Clone, Args)]
pub struct DiagramExportArgs {
    pub path: PathBuf,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    #[arg(long = "view", default_value = "all")]
    pub view: String,
    /// Explicit SysML view usage name (for example `gridStructure` or `StedinRijnmondGridExpansion::Views::gridStructure`).
    #[arg(long = "selected-view")]
    pub selected_view: Option<String>,
    #[arg(long = "format", value_enum, default_value_t = DiagramExportFormat::Svg)]
    pub format: DiagramExportFormat,
    #[arg(long = "output")]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Sarif,
    Junit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagramExportFormat {
    Svg,
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_invocation_keeps_lsp_compatibility() {
        let cli = Cli::parse_from(["spec42"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn stdio_flag_parses_for_legacy_editor_compatibility() {
        let cli = Cli::parse_from(["spec42", "--stdio"]);
        assert!(cli.stdio);
        assert!(cli.command.is_none());
    }

    #[test]
    fn check_command_parses_workspace_root_and_format() {
        let cli = Cli::parse_from([
            "spec42",
            "check",
            "models",
            "--workspace-root",
            "workspace",
            "--format",
            "json",
        ]);
        match cli.command {
            Some(Command::Check(args)) => {
                assert_eq!(args.path, PathBuf::from("models"));
                assert_eq!(args.workspace_root, Some(PathBuf::from("workspace")));
                assert_eq!(args.format, OutputFormat::Json);
                assert!(!args.warnings_as_errors);
                assert!(args.baseline.is_none());
            }
            other => panic!("expected check command, got {other:?}"),
        }
    }

    #[test]
    fn check_command_parses_ci_flags() {
        let cli = Cli::parse_from([
            "spec42",
            "check",
            "models",
            "--format",
            "sarif",
            "--warnings-as-errors",
            "--baseline",
            "baseline.json",
        ]);
        match cli.command {
            Some(Command::Check(args)) => {
                assert_eq!(args.format, OutputFormat::Sarif);
                assert!(args.warnings_as_errors);
                assert_eq!(args.baseline, Some(PathBuf::from("baseline.json")));
            }
            other => panic!("expected check command, got {other:?}"),
        }
    }

    #[test]
    fn sysand_status_command_parses() {
        let cli = Cli::parse_from(["spec42", "sysand", "status", "--format", "json"]);
        match cli.command {
            Some(Command::Sysand {
                command: SysandCommand::Status(args),
            }) => assert_eq!(args.format, OutputFormat::Json),
            other => panic!("expected sysand status command, got {other:?}"),
        }
    }

    #[test]
    fn explain_diagnostic_command_parses() {
        let cli = Cli::parse_from([
            "spec42",
            "explain-diagnostic",
            "--code",
            "unresolved_type_reference",
            "--path",
            "model.sysml",
            "--format",
            "json",
        ]);
        match cli.command {
            Some(Command::ExplainDiagnostic(args)) => {
                assert_eq!(args.code, "unresolved_type_reference");
                assert_eq!(args.path, Some(PathBuf::from("model.sysml")));
                assert_eq!(args.format, OutputFormat::Json);
            }
            other => panic!("expected explain-diagnostic command, got {other:?}"),
        }
    }

    #[test]
    fn model_summary_command_parses() {
        let cli = Cli::parse_from([
            "spec42",
            "model-summary",
            "models",
            "--workspace-root",
            "workspace",
            "--max-nodes",
            "1",
            "--format",
            "json",
        ]);
        match cli.command {
            Some(Command::ModelSummary(args)) => {
                assert_eq!(args.path, PathBuf::from("models"));
                assert_eq!(args.workspace_root, Some(PathBuf::from("workspace")));
                assert_eq!(args.max_nodes, 1);
                assert_eq!(args.format, OutputFormat::Json);
            }
            other => panic!("expected model-summary command, got {other:?}"),
        }
    }

    #[test]
    fn api_serve_command_parses() {
        let cli = Cli::parse_from([
            "spec42",
            "api",
            "serve",
            "--workspace-root",
            "workspace",
            "--bind",
            "127.0.0.1:9999",
        ]);
        match cli.command {
            Some(Command::Api {
                command: ApiCommand::Serve(args),
            }) => {
                assert_eq!(args.workspace_root, PathBuf::from("workspace"));
                assert_eq!(
                    args.bind,
                    "127.0.0.1:9999".parse().expect("socket addr")
                );
                assert!(!args.allow_remote);
            }
            other => panic!("expected api serve command, got {other:?}"),
        }
    }

    #[test]
    fn diagrams_export_command_parses() {
        let cli = Cli::parse_from([
            "spec42",
            "diagrams",
            "export",
            "models",
            "--view",
            "general-view",
            "--selected-view",
            "gridStructure",
            "--format",
            "json",
            "--output",
            "out",
        ]);
        match cli.command {
            Some(Command::Diagrams {
                command: DiagramsCommand::Export(args),
            }) => {
                assert_eq!(args.path, PathBuf::from("models"));
                assert_eq!(args.view, "general-view");
                assert_eq!(args.selected_view.as_deref(), Some("gridStructure"));
                assert_eq!(args.format, DiagramExportFormat::Json);
                assert_eq!(args.output, PathBuf::from("out"));
            }
            other => panic!("expected diagrams export command, got {other:?}"),
        }
    }
}
