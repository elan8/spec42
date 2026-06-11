use std::sync::Arc;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};

use crate::analysis::diagnostics_adapter::semantic_to_lsp_diagnostic;
use crate::analysis::diagnostics_postprocess::{
    postprocess_document_diagnostics, DiagnosticsPostprocessOptions,
};
use crate::common::util;
use crate::host::config::{DiagnosticsHostContext, SemanticCheckProvider};
use crate::semantic::SemanticGraph;

pub(crate) fn collect_document_diagnostics(
    semantic_graph: &SemanticGraph,
    library_paths: &[Url],
    check_providers: &[Arc<dyn SemanticCheckProvider>],
    indexed_sources: &[(&Url, &str)],
    uri: &Url,
    text: &str,
    block_on_any_parse_issue: bool,
    postprocess: DiagnosticsPostprocessOptions,
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

    diagnostics.extend(
        semantic_core::collect_untyped_part_usage_diagnostics(uri, text)
            .into_iter()
            .map(semantic_to_lsp_diagnostic),
    );

    let allow_semantic_checks = if block_on_any_parse_issue {
        result.errors.is_empty()
    } else if postprocess.skip_semantic_on_parse_error {
        !has_parse_error
    } else {
        true
    };

    if allow_semantic_checks {
        let host_ctx = DiagnosticsHostContext { indexed_sources };
        for provider in check_providers {
            diagnostics.extend(provider.compute_diagnostics_with_context(
                semantic_graph,
                uri,
                host_ctx,
            ));
        }

        let has_unresolved_type_reference =
            has_semantic_code(&diagnostics, "unresolved_type_reference");
        let has_unresolved_import_target =
            has_semantic_code(&diagnostics, "unresolved_import_target");
        let has_unresolved_specializes_reference =
            has_semantic_code(&diagnostics, "unresolved_specializes_reference");

        if let Some(diagnostic) = semantic_core::missing_library_context_diagnostic(
            uri,
            text,
            has_unresolved_type_reference
                || has_unresolved_import_target
                || has_unresolved_specializes_reference,
            !library_paths.is_empty(),
        ) {
            diagnostics.push(semantic_to_lsp_diagnostic(diagnostic));
        }
    }

    postprocess_document_diagnostics(uri, diagnostics, postprocess)
}

pub(crate) fn validation_postprocess_options(strict: bool) -> DiagnosticsPostprocessOptions {
    DiagnosticsPostprocessOptions {
        suppress_semantic_after_parse_error: strict,
        skip_semantic_on_parse_error: strict,
    }
}

pub(crate) fn lsp_postprocess_options() -> DiagnosticsPostprocessOptions {
    DiagnosticsPostprocessOptions {
        suppress_semantic_after_parse_error: false,
        skip_semantic_on_parse_error: false,
    }
}

fn has_semantic_code(diagnostics: &[Diagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| {
        diagnostic.source.as_deref() == Some("semantic")
            && diagnostic.code.as_ref() == Some(&NumberOrString::String(code.to_string()))
    })
}
