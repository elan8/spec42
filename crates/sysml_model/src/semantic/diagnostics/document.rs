use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::text_span::{TextPosition, TextRange};
use crate::UnitRegistry;

use super::engine::collect_diagnostics_from_graph_with_unit_registry;
use super::shared_rules::{
    collect_untyped_part_usage_diagnostics, missing_library_context_diagnostic,
};
use super::types::{DiagnosticSeverity, DiagnosticsOptions, SemanticDiagnostic};

/// Assembles the full diagnostic set for one document: parse errors, untyped-part-usage hints,
/// semantic graph diagnostics (skipped when `skip_semantic_on_parse_error` and a parse error was
/// found), and a missing-library-context hint. Shared by `workspace`'s `HostValidationReport`
/// and `lsp_server`'s LSP-typed diagnostics (batch validation and live editing) — previously two
/// independently-maintained implementations, see `docs/architecture-audit.md` P1-2.
pub fn collect_document_diagnostics(
    graph: &SemanticGraph,
    unit_registry: &UnitRegistry,
    has_library_paths: bool,
    uri: &Url,
    text: &str,
    skip_semantic_on_parse_error: bool,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = parse_diagnostics(uri, text);
    diagnostics.extend(collect_untyped_part_usage_diagnostics(uri, text));

    let has_parse_error = diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
    });
    let allow_semantic = if skip_semantic_on_parse_error {
        !has_parse_error
    } else {
        true
    };

    if allow_semantic {
        diagnostics.extend(collect_diagnostics_from_graph_with_unit_registry(
            graph,
            uri,
            DiagnosticsOptions::default(),
            unit_registry,
        ));

        let has_unresolved = has_semantic_code(&diagnostics, "unresolved_type_reference")
            || has_semantic_code(&diagnostics, "unresolved_import_target")
            || has_semantic_code(&diagnostics, "unresolved_specializes_reference");

        if let Some(diagnostic) =
            missing_library_context_diagnostic(uri, text, has_unresolved, has_library_paths)
        {
            diagnostics.push(diagnostic);
        }
    }

    diagnostics
}

fn parse_diagnostics(uri: &Url, text: &str) -> Vec<SemanticDiagnostic> {
    let result = sysml_v2_parser::parse_with_diagnostics(text);

    result
        .errors
        .into_iter()
        .map(|error| {
            let severity = match error
                .severity
                .unwrap_or(sysml_v2_parser::DiagnosticSeverity::Error)
            {
                sysml_v2_parser::DiagnosticSeverity::Error => DiagnosticSeverity::Error,
                sysml_v2_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::Warning,
            };
            SemanticDiagnostic {
                uri: uri.clone(),
                range: error
                    .to_lsp_range()
                    .map(|(sl, sc, el, ec)| {
                        TextRange::new(TextPosition::new(sl, sc), TextPosition::new(el, ec))
                    })
                    .unwrap_or_else(|| {
                        TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 0))
                    }),
                severity,
                source: "sysml".to_string(),
                code: error.code.unwrap_or_else(|| "parse_error".to_string()),
                message: error.message,
                related_information: Vec::new(),
            }
        })
        .collect()
}

fn has_semantic_code(diagnostics: &[SemanticDiagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}
