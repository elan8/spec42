use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the `spec42_check` MCP tool (matches `spec42 check` flags).
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Spec42CheckParams {
    /// Path to a `.sysml` / `.kerml` file or directory to validate.
    pub path: String,
    /// Optional workspace root (same as CLI `--workspace-root`).
    pub workspace_root: Option<String>,
    /// Optional config file path (same as global `--config`).
    pub config_path: Option<String>,
    /// Optional extra library search paths (same as repeated `--library-path`).
    pub library_paths: Option<Vec<String>>,
    /// Optional standard library root override (same as `--stdlib-path`).
    pub stdlib_path: Option<String>,
    /// When true, disables standard library use (same as `--no-stdlib`).
    #[serde(default)]
    pub no_stdlib: bool,
}
