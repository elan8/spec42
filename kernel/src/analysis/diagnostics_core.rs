use std::sync::Arc;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};

use crate::common::util;
use crate::host::config::SemanticCheckProvider;
use crate::semantic::SemanticGraph;

pub(crate) fn collect_document_diagnostics(
    semantic_graph: &SemanticGraph,
    library_paths: &[Url],
    check_providers: &[Arc<dyn SemanticCheckProvider>],
    uri: &Url,
    text: &str,
    block_on_any_parse_issue: bool,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let result = util::parse_for_editor(text);

    let has_parse_error = result.errors.iter().any(|error| {
        error
            .severity
            .unwrap_or(sysml_v2_parser::DiagnosticSeverity::Error)
            == sysml_v2_parser::DiagnosticSeverity::Error
    });

    for error in &result.errors {
        let range = error
            .to_lsp_range()
            .map(|(sl, sc, el, ec)| Range {
                start: Position::new(sl, sc),
                end: Position::new(el, ec),
            })
            .unwrap_or_else(|| Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            });
        let severity = error
            .severity
            .map(|severity| match severity {
                sysml_v2_parser::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                sysml_v2_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
            })
            .unwrap_or(DiagnosticSeverity::ERROR);
        diagnostics.push(Diagnostic {
            range,
            severity: Some(severity),
            code: error.code.clone().map(NumberOrString::String),
            code_description: None,
            source: Some("sysml".to_string()),
            message: error.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    for usage in util::untyped_part_usage_diagnostics(text) {
        diagnostics.push(Diagnostic {
            range: usage.range,
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: format!("Part '{}' has no declared type.", usage.name),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    let allow_semantic_checks = if block_on_any_parse_issue {
        result.errors.is_empty()
    } else {
        !has_parse_error
    };

    if allow_semantic_checks {
        for provider in check_providers {
            diagnostics.extend(provider.compute_diagnostics(semantic_graph, uri));
        }

        let has_unresolved_type_reference = has_semantic_code(&diagnostics, "unresolved_type_reference");
        let has_unresolved_import_target = has_semantic_code(&diagnostics, "unresolved_import_target");
        let has_unresolved_specializes_reference =
            has_semantic_code(&diagnostics, "unresolved_specializes_reference");

        if (has_unresolved_type_reference
            || has_unresolved_import_target
            || has_unresolved_specializes_reference)
            && library_paths.is_empty()
        {
            if let Some(import_range) = util::import_statement_ranges(text).into_iter().next() {
                diagnostics.push(Diagnostic {
                    range: import_range,
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(NumberOrString::String("missing_library_context".to_string())),
                    code_description: None,
                    source: Some("semantic".to_string()),
                    message: "This document imports external library symbols, but no SysML library paths are configured or indexed. Install or configure a library if these references should resolve.".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }

    diagnostics
}

fn has_semantic_code(diagnostics: &[Diagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| {
        diagnostic.source.as_deref() == Some("semantic")
            && diagnostic.code.as_ref() == Some(&NumberOrString::String(code.to_string()))
    })
}
