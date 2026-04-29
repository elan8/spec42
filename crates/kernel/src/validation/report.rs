use tower_lsp::lsp_types::{DiagnosticSeverity, NumberOrString};

use super::{ValidatedDocument, ValidationSummary};

const LIBRARY_ADVICE_CODES: [&str; 4] = [
    "missing_library_context",
    "unresolved_type_reference",
    "unresolved_import_target",
    "unresolved_specializes_reference",
];

pub(super) fn summarize(documents: &[ValidatedDocument]) -> ValidationSummary {
    let mut summary = ValidationSummary {
        document_count: documents.len(),
        ..ValidationSummary::default()
    };
    for document in documents {
        for diagnostic in &document.diagnostics {
            match diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR) {
                DiagnosticSeverity::ERROR => summary.error_count += 1,
                DiagnosticSeverity::WARNING => summary.warning_count += 1,
                DiagnosticSeverity::INFORMATION | DiagnosticSeverity::HINT => {
                    summary.information_count += 1
                }
                _ => {}
            }
        }
    }
    summary
}

pub(super) fn build_advice(documents: &[ValidatedDocument], no_library_paths: bool) -> Vec<String> {
    if !no_library_paths {
        return Vec::new();
    }
    let should_suggest_library_roots = documents.iter().any(|document| {
        document.diagnostics.iter().any(|diagnostic| match diagnostic.code.as_ref() {
            Some(NumberOrString::String(code)) => LIBRARY_ADVICE_CODES.contains(&code.as_str()),
            _ => false,
        })
    });
    if should_suggest_library_roots {
        vec![
            "Configure SysML library roots: ensure the standard library is available (bundled materialization, or pass `--stdlib-path` / `SPEC42_STDLIB_PATH` / `--library-path` explicitly)."
                .to_string(),
        ]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::{
        Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url,
    };

    use crate::validation::ValidatedDocument;

    use super::{build_advice, summarize};

    fn make_diagnostic(severity: Option<DiagnosticSeverity>, code: Option<&str>) -> Diagnostic {
        Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            severity,
            code: code.map(|value| NumberOrString::String(value.to_string())),
            message: "message".to_string(),
            ..Diagnostic::default()
        }
    }

    fn make_document(diagnostics: Vec<Diagnostic>) -> ValidatedDocument {
        ValidatedDocument {
            uri: Url::parse("file:///demo.sysml")
                .expect("uri")
                .to_string(),
            diagnostics,
        }
    }

    #[test]
    fn summarize_counts_diagnostic_severities() {
        let summary = summarize(&[
            make_document(vec![
                make_diagnostic(None, None),
                make_diagnostic(Some(DiagnosticSeverity::WARNING), None),
                make_diagnostic(Some(DiagnosticSeverity::INFORMATION), None),
                make_diagnostic(Some(DiagnosticSeverity::HINT), None),
            ]),
            make_document(vec![make_diagnostic(Some(DiagnosticSeverity::ERROR), None)]),
        ]);
        assert_eq!(summary.document_count, 2);
        assert_eq!(summary.error_count, 2);
        assert_eq!(summary.warning_count, 1);
        assert_eq!(summary.information_count, 2);
    }

    #[test]
    fn build_advice_triggers_on_library_related_codes_only_when_paths_missing() {
        let document = make_document(vec![make_diagnostic(
            Some(DiagnosticSeverity::ERROR),
            Some("unresolved_import_target"),
        )]);
        let advice = build_advice(&[document.clone()], true);
        assert_eq!(advice.len(), 1);
        assert!(advice[0].contains("Configure SysML library roots"));

        let no_advice_when_paths_exist = build_advice(&[document], false);
        assert!(no_advice_when_paths_exist.is_empty());

        let unrelated = make_document(vec![make_diagnostic(
            Some(DiagnosticSeverity::ERROR),
            Some("other_code"),
        )]);
        let no_advice_for_unrelated = build_advice(&[unrelated], true);
        assert!(no_advice_for_unrelated.is_empty());
    }
}
