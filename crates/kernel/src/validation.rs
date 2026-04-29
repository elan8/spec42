use std::path::PathBuf;
use std::sync::Arc;

use crate::host::config::Spec42Config;
use serde::Serialize;
use tower_lsp::lsp_types::Diagnostic;

mod discovery;
mod pipeline;
mod report;

#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub parallel_enabled: bool,
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
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<ValidationReport, String> {
    pipeline::validate_paths(config, request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::NumberOrString;

    fn timer_like_model() -> &'static str {
        r#"
package KitchenTimer {
    private import ScalarValues::*;
    private import ISQ::DurationValue;

    part def Battery {
        attribute capacity : Real;
        attribute runtimeEstimate : DurationValue;
    }
}
"#
    }

    fn write_timer_fixture(temp: &tempfile::TempDir) -> PathBuf {
        let model_path = temp.path().join("KitchenTimer.sysml");
        std::fs::write(&model_path, timer_like_model()).expect("write timer fixture");
        model_path
    }

    fn write_stdlib_fixture(temp: &tempfile::TempDir) -> PathBuf {
        let stdlib_root = temp.path().join("sysml.library");
        std::fs::create_dir_all(&stdlib_root).expect("create stdlib root");
        std::fs::write(
            stdlib_root.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("write ScalarValues");
        std::fs::write(
            stdlib_root.join("ISQ.sysml"),
            "standard library package ISQ { attribute def DurationValue; }",
        )
        .expect("write ISQ");
        stdlib_root
    }

    #[test]
    fn validate_paths_suggests_library_advice_when_imported_types_are_unresolved() {
        let temp = tempfile::tempdir().expect("temp dir");
        let model_path = write_timer_fixture(&temp);
        let config = Arc::new(crate::default_server_config());

        let report = validate_paths(
            &config,
            ValidationRequest {
                targets: vec![model_path],
                workspace_root: None,
                library_paths: Vec::new(),
                parallel_enabled: false,
            },
        )
        .expect("validation report");

        let unresolved: Vec<&Diagnostic> = report.documents[0]
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved
                .iter()
                .any(|diagnostic| diagnostic.message.contains("Real")),
            "expected unresolved Real diagnostic: {unresolved:#?}"
        );
        assert!(
            unresolved
                .iter()
                .any(|diagnostic| diagnostic.message.contains("DurationValue")),
            "expected unresolved DurationValue diagnostic: {unresolved:#?}"
        );
        assert_eq!(report.advice.len(), 1);
        assert!(report.advice[0].contains("library roots"));
    }

    #[test]
    fn validate_paths_resolves_timer_library_types_when_stdlib_root_is_present() {
        let temp = tempfile::tempdir().expect("temp dir");
        let model_path = write_timer_fixture(&temp);
        let stdlib_root = write_stdlib_fixture(&temp);
        let config = Arc::new(crate::default_server_config());

        let report = validate_paths(
            &config,
            ValidationRequest {
                targets: vec![model_path],
                workspace_root: None,
                library_paths: vec![stdlib_root],
                parallel_enabled: false,
            },
        )
        .expect("validation report");

        let unresolved: Vec<&Diagnostic> = report.documents[0]
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved.is_empty(),
            "expected managed stdlib fixture to clear unresolved type refs: {unresolved:#?}"
        );
        assert!(report.advice.is_empty());
    }
}
