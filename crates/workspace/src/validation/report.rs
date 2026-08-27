//! Summary counts and advice over a batch report's own neutral diagnostic values.

use crate::snapshot::{HostValidatedDocument, HostValidationSummary};

const LIBRARY_ADVICE_CODES: [&str; 4] = [
    "missing_library_context",
    "unresolved_type_reference",
    "unresolved_import_target",
    "unresolved_specializes_reference",
];

/// The one summariser, shared with the snapshot's eager validation report.
pub(super) fn summarize(documents: &[HostValidatedDocument]) -> HostValidationSummary {
    crate::snapshot::summarize_validated_documents(documents)
}

pub(super) fn build_advice(
    documents: &[HostValidatedDocument],
    cascade_dominated: bool,
    no_library_paths: bool,
) -> Vec<String> {
    let mut advice = Vec::new();
    if cascade_dominated {
        advice.push(
            "Many errors may be cascades from a few root syntax issues; fix the earliest error in each file first."
                .to_string(),
        );
    }
    if !no_library_paths {
        return advice;
    }
    let should_suggest_library_roots = documents.iter().any(|document| {
        document
            .diagnostics
            .iter()
            .any(|diagnostic| LIBRARY_ADVICE_CODES.contains(&diagnostic.code.as_str()))
    });
    if should_suggest_library_roots {
        advice.push(
            "Configure SysML library roots: ensure the standard library is available (bundled materialization, or pass `--stdlib-path` / `SPEC42_STDLIB_PATH` / `--library-path` explicitly)."
                .to_string(),
        );
    }
    advice
}

#[cfg(test)]
mod tests {
    use sysml_diagnostics::{DiagnosticSeverity, SemanticDiagnostic};
    use sysml_query::resolved_slice::{TextPosition, TextRange};
    use sysml_query::source::Url;

    use super::{build_advice, summarize};
    use crate::snapshot::HostValidatedDocument;

    fn make_diagnostic(severity: DiagnosticSeverity, code: &str) -> SemanticDiagnostic {
        SemanticDiagnostic {
            uri: Url::parse("file:///demo.sysml").expect("uri"),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            severity,
            source: "spec42".to_string(),
            code: code.to_string(),
            message: "message".to_string(),
            unresolved_reference_target: None,
            related_information: Vec::new(),
        }
    }

    fn make_document(diagnostics: Vec<SemanticDiagnostic>) -> HostValidatedDocument {
        HostValidatedDocument {
            uri: "file:///demo.sysml".to_string(),
            diagnostics,
        }
    }

    #[test]
    fn summarize_counts_diagnostic_severities() {
        let summary = summarize(&[
            make_document(vec![
                make_diagnostic(DiagnosticSeverity::Error, "a"),
                make_diagnostic(DiagnosticSeverity::Warning, "a"),
                make_diagnostic(DiagnosticSeverity::Information, "a"),
            ]),
            make_document(vec![make_diagnostic(DiagnosticSeverity::Error, "a")]),
        ]);
        assert_eq!(summary.document_count, 2);
        assert_eq!(summary.error_count, 2);
        assert_eq!(summary.warning_count, 1);
        assert_eq!(summary.information_count, 1);
    }

    #[test]
    fn build_advice_triggers_on_library_related_codes_only_when_paths_missing() {
        let document = make_document(vec![make_diagnostic(
            DiagnosticSeverity::Error,
            "unresolved_import_target",
        )]);
        let advice = build_advice(std::slice::from_ref(&document), false, true);
        assert_eq!(advice.len(), 1);
        assert!(advice[0].contains("Configure SysML library roots"));

        let no_advice_when_paths_exist = build_advice(&[document], false, false);
        assert!(no_advice_when_paths_exist.is_empty());

        let unrelated = make_document(vec![make_diagnostic(
            DiagnosticSeverity::Error,
            "other_code",
        )]);
        let no_advice_for_unrelated = build_advice(&[unrelated], false, true);
        assert!(no_advice_for_unrelated.is_empty());
    }
}
