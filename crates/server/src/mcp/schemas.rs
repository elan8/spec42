use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Global CLI flags shared by MCP tools (parity with `spec42` global options).
#[derive(Debug, Clone, Default, Deserialize, Serialize, JsonSchema)]
pub struct Spec42GlobalParams {
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

/// Parameters for the `spec42_check` MCP tool (matches `spec42 check` flags).
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Spec42CheckParams {
    /// Path to a `.sysml` / `.kerml` file or directory to validate.
    pub path: String,
    /// Optional workspace root (same as CLI `--workspace-root`).
    pub workspace_root: Option<String>,
    #[serde(flatten)]
    pub global: Spec42GlobalParams,
    /// When true, include `semantic_model` in the validation report (can be large).
    #[serde(default)]
    pub include_semantic_model: bool,
}

/// Parameters for `spec42_doctor` (same global flags as CLI `spec42 doctor`).
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Spec42DoctorParams {
    #[serde(flatten)]
    pub global: Spec42GlobalParams,
}

/// Parameters for `spec42_model_summary`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Spec42ModelSummaryParams {
    /// Path to a `.sysml` / `.kerml` file or directory.
    pub path: String,
    /// Optional workspace root (same as CLI `--workspace-root`).
    pub workspace_root: Option<String>,
    #[serde(flatten)]
    pub global: Spec42GlobalParams,
    /// Maximum number of semantic nodes to return (default 500).
    #[serde(default = "default_max_nodes")]
    pub max_nodes: usize,
}

fn default_max_nodes() -> usize {
    500
}

/// Parameters for `spec42_explain_diagnostic`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Spec42ExplainDiagnosticParams {
    /// Diagnostic code (e.g. `unresolved_type_reference`).
    pub code: String,
    /// Optional file or directory to list matching diagnostics from a check run.
    pub path: Option<String>,
    /// Optional workspace root when `path` is set.
    pub workspace_root: Option<String>,
    /// Optional 0-based line number to filter instances.
    pub line: Option<u32>,
    #[serde(flatten)]
    pub global: Spec42GlobalParams,
}
