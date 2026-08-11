//! Diagnostics over the immutable semantic publication.
//!
//! This module is deliberately independent of the legacy mutable graph. It consumes only the
//! typed authored-reference outcomes and read-only node queries exposed by `ResolutionView`.

use std::collections::{BTreeMap, HashMap, HashSet};

use url::Url;

use sysml_model::semantic::text_span::{TextPosition, TextRange};
use sysml_model::{
    AuthoredReferenceId, ElementKind, ReferenceKind, RelationshipKind, ResolutionOutcome,
    SemanticDiagnosticInput, SemanticModel,
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
        let input = model.diagnostic_input();
        let mut has_unresolved = false;
        for fact in input.facts() {
            if fact.reference.source.uri != *uri {
                continue;
            }
            let Some(node) = input
                .nodes()
                .iter()
                .find(|node| node.id == fact.reference.source)
            else {
                continue;
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
        }
        diagnostics.extend(collect_inherited_value_diagnostics(&input, uri));
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
    input: &SemanticDiagnosticInput,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let nodes = input
        .nodes()
        .iter()
        .map(|node| (node.id.clone(), node))
        .collect::<HashMap<_, _>>();
    let mut relationships =
        BTreeMap::<(sysml_model::NodeId, RelationshipKind), Vec<sysml_model::NodeId>>::new();
    for relationship in input.relationships() {
        relationships
            .entry((relationship.source.clone(), relationship.kind.clone()))
            .or_default()
            .push(relationship.target.clone());
    }
    let mut diagnostics = Vec::new();
    for node in input.nodes().iter().filter(|node| node.id.uri == *uri) {
        let Some(value) = node
            .attributes
            .get("value")
            .and_then(|value| value.as_str())
        else {
            continue;
        };
        if node.element_kind == ElementKind::Ref
            || !node.declared_facts.relationships.redefinition.is_empty()
        {
            continue;
        }
        let Some(owner_id) = node.parent_id.as_ref() else {
            continue;
        };
        let Some(owner) = nodes.get(owner_id) else {
            continue;
        };
        let mut roots = relationships
            .get(&(owner.id.clone(), RelationshipKind::Typing))
            .cloned()
            .unwrap_or_default();
        if roots.is_empty() {
            roots.push(owner.id.clone());
        }
        let mut stack = roots;
        let mut visited = HashSet::new();
        let mut inherited = None;
        while let Some(type_id) = stack.pop() {
            if !visited.insert(type_id.clone()) {
                continue;
            }
            if let Some(candidate) = nodes.values().find(|candidate| {
                candidate.parent_id.as_ref() == Some(&type_id) && candidate.name == node.name
            }) {
                inherited = Some(*candidate);
                break;
            }
            stack.extend(
                relationships
                    .get(&(type_id, RelationshipKind::Specializes))
                    .into_iter()
                    .flatten()
                    .cloned(),
            );
        }
        let Some(inherited) = inherited else {
            continue;
        };
        diagnostics.push(reference_diagnostic(
            uri,
            node.range,
            &AuthoredReferenceId {
                source: node.id.clone(),
                kind: ReferenceKind::Redefinition,
                authored_ordinal: 0,
            },
            "implicit_redefinition_without_operator",
            DiagnosticSeverity::Error,
            format!(
                "Feature '{}' overrides inherited {} '{}' but is missing explicit redefinition ':>>'.",
                node.name, inherited.element_kind, inherited.name
            ),
        ));
        if node.element_kind != ElementKind::Attribute {
            continue;
        }
        if !is_string_literal(value) {
            continue;
        }
        let Some(type_ref) = inherited
            .declared_facts
            .relationships
            .typing
            .first()
            .map(|target| target.reference.as_str())
        else {
            continue;
        };
        if nodes.values().any(|candidate| {
            candidate.element_kind == ElementKind::EnumDef
                && (candidate.id.qualified_name == type_ref
                    || candidate.name == type_ref.rsplit("::").next().unwrap_or(type_ref))
        }) {
            diagnostics.push(reference_diagnostic(
                uri,
                node.range,
                &AuthoredReferenceId {
                    source: node.id.clone(),
                    kind: ReferenceKind::Redefinition,
                    authored_ordinal: 0,
                },
                "inherited_attribute_value_type_mismatch",
                DiagnosticSeverity::Error,
                format!(
                    "Feature '{}' is typed as enum '{}' but was assigned string literal {}; use an enumeration value.",
                    node.name, type_ref, value.trim()
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
