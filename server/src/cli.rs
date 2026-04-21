use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "spec42", version, about = "SysML v2 language server and CLI")]
pub struct Cli {
    #[arg(long = "config", global = true)]
    pub config_path: Option<PathBuf>,
    #[arg(long = "library-path", global = true)]
    pub library_paths: Vec<PathBuf>,
    #[arg(long = "stdlib-path", global = true)]
    pub stdlib_path: Option<PathBuf>,
    #[arg(long = "no-stdlib", global = true, default_value_t = false)]
    pub no_stdlib: bool,
    #[arg(long = "stdio", global = true, hide = true, default_value_t = false)]
    pub stdio: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Lsp,
    Check(CheckArgs),
    Doctor(DoctorArgs),
    Generate(GenerateArgs),
    Stdlib {
        #[command(subcommand)]
        command: StdlibCommand,
    },
}

#[derive(Debug, Clone, Args)]
pub struct CheckArgs {
    pub path: PathBuf,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Args)]
pub struct DoctorArgs {
    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub target: GenerateTarget,
}

#[derive(Debug, Clone, Subcommand)]
pub enum GenerateTarget {
    Ros2(GenerateRos2Args),
}

#[derive(Debug, Clone, Args)]
pub struct GenerateRos2Args {
    #[arg(long = "input")]
    pub input: PathBuf,
    #[arg(long = "output")]
    pub output: PathBuf,
    #[arg(long = "package-name")]
    pub package_name: Option<String>,
    #[arg(long = "workspace-root")]
    pub workspace_root: Option<PathBuf>,
    #[arg(long = "force", default_value_t = false)]
    pub force: bool,
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
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
            }
            other => panic!("expected check command, got {other:?}"),
        }
    }

    #[test]
    fn generate_ros2_command_parses_all_args() {
        let cli = Cli::parse_from([
            "spec42",
            "generate",
            "ros2",
            "--input",
            "domain-libraries/robotics/examples/inspection-rover/inspection-rover.sysml",
            "--output",
            "out",
            "--package-name",
            "inspection_rover_bringup",
            "--workspace-root",
            ".",
            "--force",
            "--dry-run",
        ]);
        match cli.command {
            Some(Command::Generate(GenerateArgs {
                target: GenerateTarget::Ros2(args),
            })) => {
                assert_eq!(
                    args.input,
                    PathBuf::from(
                        "domain-libraries/robotics/examples/inspection-rover/inspection-rover.sysml"
                    )
                );
                assert_eq!(args.output, PathBuf::from("out"));
                assert_eq!(
                    args.package_name,
                    Some("inspection_rover_bringup".to_string())
                );
                assert_eq!(args.workspace_root, Some(PathBuf::from(".")));
                assert!(args.force);
                assert!(args.dry_run);
            }
            other => panic!("expected generate ros2 command, got {other:?}"),
        }
    }
}
