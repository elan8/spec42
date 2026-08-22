use std::path::PathBuf;
use std::sync::Arc;

use crate::host::config::Spec42Config;
use serde::Serialize;
use tower_lsp::lsp_types::Diagnostic;
use workspace::Spec42Engine;

mod built_workspace;
mod discovery;
mod report;

#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub parallel_enabled: bool,
    /// When true, skip semantic checks after parse errors and suppress shadowed semantic warnings (legacy `spec42 check` behavior).
    pub strict_diagnostics: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub workspace_root: Option<String>,
    pub resolved_library_paths: Vec<String>,
    pub documents: Vec<ValidatedDocument>,
    pub summary: ValidationSummary,
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticValidationReport {
    pub validation: ValidationReport,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidatedDocument {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ValidationSummary {
    pub document_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub information_count: usize,
}

pub fn validate_paths(
    engine: &Spec42Engine,
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<ValidationReport, String> {
    built_workspace::validate_paths(engine, config, request)
}

pub fn validate_paths_with_semantics(
    engine: &Spec42Engine,
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<SemanticValidationReport, String> {
    built_workspace::validate_paths_with_semantics(engine, config, request)
}

pub use built_workspace::{
    built_workspace_input_from_snapshot, semantic_report_from_built_workspace, BuiltWorkspaceInput,
};
