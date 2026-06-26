//! Host-facing validation report (portable, tower-lsp-free).

use sysml_model::SemanticDiagnostic;

#[derive(Debug, Clone)]
pub struct HostValidatedDocument {
    pub uri: String,
    pub diagnostics: Vec<SemanticDiagnostic>,
}

#[derive(Debug, Clone, Default)]
pub struct HostValidationSummary {
    pub document_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub information_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct HostValidationReport {
    pub workspace_root: Option<String>,
    pub resolved_library_paths: Vec<String>,
    pub documents: Vec<HostValidatedDocument>,
    pub summary: HostValidationSummary,
}
