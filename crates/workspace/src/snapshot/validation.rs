//! Host-facing validation report (portable, tower-lsp-free).

use sysml_diagnostics::{DiagnosticSeverity, ReportingPolicy, SemanticDiagnostic};
use sysml_query::resolved_slice::PublishedModel;

use super::discovery::path_to_file_url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostValidatedDocument {
    pub uri: String,
    pub diagnostics: Vec<SemanticDiagnostic>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostValidationSummary {
    pub document_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub information_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostValidationReport {
    pub workspace_root: Option<String>,
    pub resolved_library_paths: Vec<String>,
    pub documents: Vec<HostValidatedDocument>,
    pub summary: HostValidationSummary,
}

pub(crate) fn collect_host_validation_report(
    model: &PublishedModel,
    target_files: &[std::path::PathBuf],
    workspace_root: Option<&std::path::Path>,
    library_paths_display: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> crate::error::WorkspaceResult<HostValidationReport> {
    let policy = ReportingPolicy::strict(strict_diagnostics);
    let documents = target_files
        .iter()
        .map(|path| {
            let uri = path_to_file_url(path)?;
            Ok(HostValidatedDocument {
                uri: uri.to_string(),
                diagnostics: sysml_diagnostics::document_diagnostics(model, &uri, policy),
            })
        })
        .collect::<crate::error::WorkspaceResult<Vec<_>>>()?;
    let summary = summarize(&documents);
    Ok(HostValidationReport {
        workspace_root: workspace_root.map(|path| path.display().to_string()),
        resolved_library_paths: library_paths_display
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        documents,
        summary,
    })
}

fn summarize(documents: &[HostValidatedDocument]) -> HostValidationSummary {
    let mut summary = HostValidationSummary {
        document_count: documents.len(),
        ..HostValidationSummary::default()
    };
    for diagnostic in documents.iter().flat_map(|document| &document.diagnostics) {
        match diagnostic.severity {
            DiagnosticSeverity::Error => summary.error_count += 1,
            DiagnosticSeverity::Warning => summary.warning_count += 1,
            DiagnosticSeverity::Information => summary.information_count += 1,
        }
    }
    summary
}
