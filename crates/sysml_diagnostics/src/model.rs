//! Diagnostics over the immutable semantic publication.
//!
//! This module is deliberately independent of the legacy mutable graph. It consumes only the
//! typed authored-reference outcomes and read-only node queries exposed by `ResolutionView`.

use url::Url;

use sysml_model::semantic::text_span::{TextPosition, TextRange};
use sysml_model::{
    AuthoredReferenceId, ElementKind, ReferenceKind, RelationshipKind, ResolutionOutcome,
    SemanticModel,
};

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
        let input = model.resolution_diagnostics();
        let mut has_unresolved = false;
        for fact in input.references() {
            if fact.source.uri != *uri {
                continue;
            }
            let reference = AuthoredReferenceId {
                source: fact.source.clone(),
                kind: fact.kind,
                authored_ordinal: fact.authored_ordinal,
            };
            let range = fact.authored_range.unwrap_or(fact.source_range);
            match &fact.outcome {
                ResolutionOutcome::Resolved { .. } => {}
                ResolutionOutcome::Unresolved => {
                    has_unresolved = true;
                    diagnostics.push(reference_diagnostic(
                        uri,
                        range,
                        &reference,
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
                        &reference,
                        "ambiguous_reference",
                        DiagnosticSeverity::Error,
                        format!("ambiguous reference {:?}", fact.authored_target),
                    ));
                }
                ResolutionOutcome::UnsupportedFiltered => diagnostics.push(reference_diagnostic(
                    uri,
                    range,
                    &reference,
                    "unsupported_reference",
                    DiagnosticSeverity::Warning,
                    format!("unsupported reference {:?}", fact.authored_target),
                )),
            }
        }
        diagnostics.extend(collect_inherited_value_diagnostics(model, uri));
        diagnostics.extend(collect_category_diagnostics(model, uri));
        if let Some(diagnostic) =
            missing_library_context_diagnostic(uri, text, has_unresolved, has_library_paths)
        {
            diagnostics.push(diagnostic);
        }
    }
    canonicalize_diagnostics(&mut diagnostics);
    diagnostics
}

fn collect_inherited_value_diagnostics(
    model: &SemanticModel,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    for fact in model
        .structural_diagnostics()
        .inherited_features()
        .iter()
        .filter(|fact| fact.feature.uri == *uri)
    {
        let reference = AuthoredReferenceId {
            source: fact.feature.clone(),
            kind: ReferenceKind::Redefinition,
            authored_ordinal: 0,
        };
        diagnostics.push(reference_diagnostic(
            uri,
            fact.range,
            &reference,
            "implicit_redefinition_without_operator",
            DiagnosticSeverity::Error,
            format!(
                "Feature '{}' overrides inherited {} '{}' but is missing explicit redefinition ':>>'.",
                fact.feature_name, fact.inherited_feature_kind, fact.inherited_feature_name
            ),
        ));
        if fact.feature_kind != ElementKind::Attribute {
            continue;
        }
        let Some(value) = fact.authored_value.as_deref() else {
            continue;
        };
        if !is_string_literal(value) {
            continue;
        }
        let Some(type_ref) = fact.inherited_type.as_deref() else {
            continue;
        };
        if fact.inherited_is_enum {
            diagnostics.push(reference_diagnostic(
                uri,
                fact.range,
                &reference,
                "inherited_attribute_value_type_mismatch",
                DiagnosticSeverity::Error,
                format!(
                    "Feature '{}' is typed as enum '{}' but was assigned string literal {}; use an enumeration value.",
                    fact.feature_name, type_ref, value.trim()
                ),
            ));
        }
    }
    diagnostics
}

fn is_string_literal(value: &str) -> bool {
    let value = value.trim();
    value.starts_with('"') && value.ends_with('"') && value.len() >= 2
}

/// Model-native conformance checks whose inputs are owned by semantic categories. These checks
/// intentionally consume projections rather than recover meaning from serialized attributes or
/// rebuild endpoint graphs in the diagnostics crate.
fn collect_category_diagnostics(model: &SemanticModel, uri: &Url) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();

    for relationship in model
        .connection_diagnostics()
        .relationships()
        .iter()
        .filter(|relationship| relationship.source.uri == *uri)
    {
        let source_port = matches!(
            relationship.source_kind,
            ElementKind::Port | ElementKind::PortDef | ElementKind::InterfaceEnd
        );
        let target_port = matches!(
            relationship.target_kind,
            ElementKind::Port | ElementKind::PortDef | ElementKind::InterfaceEnd
        );
        let source_part = matches!(
            relationship.source_kind,
            ElementKind::Part | ElementKind::PartDef | ElementKind::Item
        );
        let target_part = matches!(
            relationship.target_kind,
            ElementKind::Part | ElementKind::PartDef | ElementKind::Item
        );
        if !(source_port || target_port || (source_part && target_part)) {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: relationship.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "connection_context_invalid".to_string(),
                message: "Connection endpoints are not in a connectable structural context."
                    .to_string(),
                related_information: Vec::new(),
            });
        }
    }

    for relationship in model
        .behavior_diagnostics()
        .relationships()
        .iter()
        .filter(|relationship| relationship.source.uri == *uri)
    {
        if relationship.kind == RelationshipKind::Perform
            && !matches!(
                relationship.target_kind,
                ElementKind::Action | ElementKind::ActionDef | ElementKind::Perform
            )
        {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: relationship.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "perform_target_invalid_kind".to_string(),
                message: "Perform target must resolve to an action definition or usage."
                    .to_string(),
                related_information: Vec::new(),
            });
        }
    }

    for relationship in model
        .requirement_case_diagnostics()
        .relationships()
        .iter()
        .filter(|relationship| relationship.source.uri == *uri)
    {
        if relationship.kind == RelationshipKind::Satisfy
            && !matches!(
                relationship.target_kind,
                ElementKind::Requirement
                    | ElementKind::RequirementDef
                    | ElementKind::VerifiedRequirement
            )
        {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: relationship.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "satisfy_target_invalid_kind".to_string(),
                message: "Satisfy target must resolve to a requirement.".to_string(),
                related_information: Vec::new(),
            });
        }
    }

    for relationship in model
        .view_diagnostics()
        .relationships()
        .iter()
        .filter(|relationship| relationship.source.uri == *uri)
    {
        if relationship.kind == RelationshipKind::Satisfy
            && matches!(
                relationship.source_kind,
                ElementKind::View | ElementKind::ViewDef
            )
            && !matches!(
                relationship.target_kind,
                ElementKind::Viewpoint | ElementKind::ViewpointDef
            )
        {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: relationship.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "viewpoint_conformance_invalid_target_kind".to_string(),
                message: "View satisfy target must resolve to a viewpoint.".to_string(),
                related_information: Vec::new(),
            });
        }
    }

    // Evaluation and units are published as typed results. A consumer can report these facts
    // without parsing expression text; unit-specific policy remains owned by the evaluator.
    for fact in model
        .expression_diagnostics()
        .facts()
        .iter()
        .filter(|fact| fact.owner.uri == *uri)
    {
        if fact.analysis_passed == Some(false) {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: fact.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "analysis_constraint_failed".to_string(),
                message: "Analysis constraint evaluated to false.".to_string(),
                related_information: Vec::new(),
            });
        } else if fact.analysis_status == Some(sysml_model::EvaluationStatus::Unresolved) {
            diagnostics.push(SemanticDiagnostic {
                uri: uri.clone(),
                range: fact.range,
                severity: DiagnosticSeverity::Warning,
                source: "semantic".to_string(),
                code: "analysis_evaluation_unresolved".to_string(),
                message: fact
                    .analysis_error
                    .clone()
                    .unwrap_or_else(|| "Analysis expression could not be evaluated.".to_string()),
                related_information: Vec::new(),
            });
        }
    }
    diagnostics
}

fn reference_diagnostic(
    uri: &Url,
    range: TextRange,
    reference: &AuthoredReferenceId,
    code: &str,
    severity: DiagnosticSeverity,
    message: String,
) -> SemanticDiagnostic {
    SemanticDiagnostic {
        uri: uri.clone(),
        range,
        severity,
        source: "semantic".to_string(),
        code: if code.starts_with("unresolved_") {
            diagnostic_code(reference.kind, severity)
        } else {
            code.to_string()
        },
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
