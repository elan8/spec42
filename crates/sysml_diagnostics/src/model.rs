//! Diagnostics over the immutable semantic publication.
//!
//! This module is deliberately independent of the legacy mutable graph. It consumes only the
//! typed authored-reference outcomes and read-only node queries exposed by `ResolutionView`.

use url::Url;

use sysml_model::semantic::text_span::{TextPosition, TextRange};
use sysml_model::{AuthoredReferenceId, ReferenceKind, ResolutionOutcome, SemanticModel};

use crate::ordering::canonicalize_diagnostics;
use crate::shared_rules::{
    collect_untyped_part_usage_diagnostics, missing_library_context_diagnostic,
};
use crate::types::{DiagnosticSeverity, DiagnosticsOptions, SemanticDiagnostic};

/// Collects source and canonical-resolution diagnostics from one immutable semantic model.
///
/// This is the model-native diagnostics entry point for the snapshot runner and future hosts.
/// It does not materialize or inspect `SemanticGraph`, `ResolutionState`, resolver indexes, or
/// pending queues. More specialized checks should be added here as typed model facts migrate into
/// the publication contract.
pub fn collect_document_diagnostics_from_model(
    model: &SemanticModel,
    has_library_paths: bool,
    uri: &Url,
    text: &str,
    skip_semantic_on_parse_error: bool,
    _options: DiagnosticsOptions,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = parse_diagnostics(uri, text);
    diagnostics.extend(collect_untyped_part_usage_diagnostics(uri, text));
    let has_parse_error = diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
    });
    if !(skip_semantic_on_parse_error && has_parse_error) {
        let mut has_unresolved = false;
        model.view().visit_facts(|fact| {
            if fact.reference.source.uri != *uri {
                return;
            }
            let Some(node) = model.view().node(&fact.reference.source) else {
                return;
            };
            let range = fact.authored_range.unwrap_or(node.range);
            match &fact.outcome {
                ResolutionOutcome::Resolved { .. } => {}
                ResolutionOutcome::Unresolved => {
                    has_unresolved = true;
                    diagnostics.push(reference_diagnostic(
                        uri,
                        range,
                        &fact.reference,
                        "unresolved_reference",
                        DiagnosticSeverity::Warning,
                        format!("unresolved reference {:?}", fact.authored_target),
                    ));
                }
                ResolutionOutcome::Ambiguous { .. } => {
                    has_unresolved = true;
                    diagnostics.push(reference_diagnostic(
                        uri,
                        range,
                        &fact.reference,
                        "ambiguous_reference",
                        DiagnosticSeverity::Error,
                        format!("ambiguous reference {:?}", fact.authored_target),
                    ));
                }
                ResolutionOutcome::UnsupportedFiltered => diagnostics.push(reference_diagnostic(
                    uri,
                    range,
                    &fact.reference,
                    "unsupported_reference",
                    DiagnosticSeverity::Warning,
                    format!("unsupported reference {:?}", fact.authored_target),
                )),
            }
        });
        if let Some(diagnostic) =
            missing_library_context_diagnostic(uri, text, has_unresolved, has_library_paths)
        {
            diagnostics.push(diagnostic);
        }
    }
    canonicalize_diagnostics(&mut diagnostics);
    diagnostics
}

fn reference_diagnostic(
    uri: &Url,
    range: TextRange,
    reference: &AuthoredReferenceId,
    _code: &str,
    severity: DiagnosticSeverity,
    message: String,
) -> SemanticDiagnostic {
    SemanticDiagnostic {
        uri: uri.clone(),
        range,
        severity,
        source: "semantic".to_string(),
        code: diagnostic_code(reference.kind, severity),
        message,
        related_information: Vec::new(),
    }
}

fn diagnostic_code(kind: ReferenceKind, severity: DiagnosticSeverity) -> String {
    let base = match kind {
        ReferenceKind::FeatureTyping => "unresolved_type_reference",
        ReferenceKind::Specialization => "unresolved_specializes_reference",
        ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
            "unresolved_import_target"
        }
        _ => match severity {
            DiagnosticSeverity::Error => "ambiguous_reference",
            _ => "unresolved_reference",
        },
    };
    base.to_string()
}

fn parse_diagnostics(uri: &Url, text: &str) -> Vec<SemanticDiagnostic> {
    sysml_v2_parser::parse_with_diagnostics(text)
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
