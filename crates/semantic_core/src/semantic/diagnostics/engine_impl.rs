//! Semantic validation beyond syntax: port connectivity, type compatibility, etc.
//!
//! These checks use the semantic graph (parts, ports, connections) to report
//! diagnostics such as: unconnected ports, connection to non-port, port type mismatch.

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
use url::Url;

use crate::semantic::diagnostics::checks::import_resolution::{
    has_import_in_scope, import_target, import_target_resolves,
};
use crate::semantic::diagnostics::helpers::*;
use crate::semantic::diagnostics::relationship_endpoint_messages::builder_relationship_diagnostic_to_emit;
use crate::semantic::diagnostics::types::{DiagnosticRelatedInfo, DiagnosticSeverity};
use crate::semantic::relationships::SPECIALIZES_TARGET_KINDS;
use crate::{
    resolve_inherited_member_via_type, RelationshipKind, ResolveResult, SemanticDiagnostic,
    SemanticGraph,
};

const RULE6_ALLOWED_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "enum def",
    "alias",
    "kermlDecl",
    "view def",
    "viewpoint def",
];

fn is_view_kind(kind: &str) -> bool {
    matches!(kind, "view" | "view def")
}

fn is_viewpoint_kind(kind: &str) -> bool {
    matches!(kind, "viewpoint" | "viewpoint def")
}

/// Returns LSP diagnostics for semantic rules in the given document.
/// Only runs when the document has been parsed and merged into the graph.
pub fn compute_semantic_diagnostics(graph: &SemanticGraph, uri: &Url) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let total_start = Instant::now();
    let mut section_timings = Vec::<(String, u128, usize)>::new();
    let mut import_scope_cache = HashMap::<String, bool>::new();
    let mut rule6_resolution_cache = HashMap::<(String, String), bool>::new();
    let mut rule7_resolution_cache = HashMap::<(String, String), bool>::new();
    let mut rule6_graph_name_fallback_cache = HashMap::<String, bool>::new();
    let mut rule7_graph_name_fallback_cache = HashMap::<String, bool>::new();

    // 0) Explicit builder diagnostics (e.g. ambiguous endpoint resolution).
    let t0 = Instant::now();
    let d0 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "diagnostic" {
            continue;
        }
        let code = node
            .attributes
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic_diagnostic");
        let message = node
            .attributes
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic diagnostic")
            .to_string();
        let Some((emit_code, emit_message)) =
            builder_relationship_diagnostic_to_emit(graph, uri, node, code, &message)
        else {
            continue;
        };
        let range = extract_single_quoted_value(&emit_message)
            .and_then(|reference| reference_token_range(node, &reference))
            .unwrap_or_else(|| diagnostic_range(graph, node, None));
        let mut diagnostic = diag(
            uri,
            range,
            DiagnosticSeverity::Warning,
            "semantic",
            &emit_code,
            emit_message,
        );
        if let Some(related) = resolved_endpoint_related_information(graph, node) {
            diagnostic.related_information.push(related);
        }
        diagnostics.push(diagnostic);
    }
    section_timings.push((
        "0_builder_diagnostics".to_string(),
        t0.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d0),
    ));

    // 1) Connection endpoints must be ports; port types must be compatible
    let t1 = Instant::now();
    let d1 = diagnostics.len();
    let connection_occurrences = graph.connect_statement_edges_for_uri(uri);
    for (src_id, tgt_id, connect) in connection_occurrences {
        let connection_range = connect.range;
        if let (Some(src), Some(tgt)) = (graph.get_node(&src_id), graph.get_node(&tgt_id)) {
            let both_part_like = is_part_like(&src.element_kind) && is_part_like(&tgt.element_kind);
            if !both_part_like {
                if !is_port_like(&src.element_kind) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, src, Some(tgt)),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "connection_endpoint_not_port",
                        format!(
                            "Connection source '{}' is not a port (element kind: {}).",
                            src.name, src.element_kind
                        ),
                    ));
                }
                if !is_port_like(&tgt.element_kind) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, tgt, Some(src)),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "connection_endpoint_not_port",
                        format!(
                            "Connection target '{}' is not a port (element kind: {}).",
                            tgt.name, tgt.element_kind
                        ),
                    ));
                }
            }
            if is_port_like(&src.element_kind) && is_port_like(&tgt.element_kind) {
                if let Some(msg) = port_compatibility_mismatch(graph, src, tgt) {
                    diagnostics.push(diag(
                        uri,
                        connection_range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "port_type_mismatch",
                        msg,
                    ));
                }
            }
        }
    }
    section_timings.push((
        "1_connection_endpoints_and_port_compatibility".to_string(),
        t1.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d1),
    ));

    // 2) Unconnected ports (ports in this URI that are not an endpoint of any connection)
    let t2 = Instant::now();
    let d2 = diagnostics.len();
    let connected_port_keys: HashSet<String> = graph
        .connection_edge_node_pairs_for_uri(uri)
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .filter_map(|id| graph.get_node(&id))
        .filter_map(port_anchor_key)
        .collect();

    for node in graph.nodes_for_uri(uri) {
        if is_port_like(&node.element_kind)
            && node.element_kind == "port"
            && !is_synthetic(node)
            && is_declaration_port(graph, node)
            && !node.attributes.contains_key("redefines")
            && !node.attributes.contains_key("subsetsFeature")
            && port_anchor_key(node)
                .as_ref()
                .is_some_and(|key| !connected_port_keys.contains(key))
        {
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Information,
                "semantic",
                "unconnected_port",
                format!("Port '{}' is not connected to any other port.", node.name),
            ));
        }
    }
    section_timings.push((
        "2_unconnected_ports".to_string(),
        t2.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d2),
    ));

    // 3) Duplicate connections (same textual connect endpoints repeated)
    let t3 = Instant::now();
    let d3 = diagnostics.len();
    let mut seen_connections: HashSet<String> = HashSet::new();
    for (src_id, tgt_id, connect) in graph.connect_statement_edges_for_uri(uri) {
        let key = connection_duplicate_key(
            Some(connect.source_expression.as_str()),
            Some(connect.target_expression.as_str()),
            &src_id,
            &tgt_id,
        );
        if !seen_connections.insert(key) {
            diagnostics.push(diag(
                uri,
                connect.range,
                DiagnosticSeverity::Information,
                "semantic",
                "duplicate_connection",
                "Duplicate connection between the same two endpoints.".to_string(),
            ));
        }
    }
    section_timings.push((
        "3_duplicate_connections".to_string(),
        t3.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d3),
    ));

    // 4) Multiplicity validation (syntax and interval sanity)
    let t4 = Instant::now();
    let d4 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if let Some(multiplicity) = node.attributes.get("multiplicity").and_then(|v| v.as_str()) {
            if let Some(message) = multiplicity_issue_message(multiplicity) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "invalid_multiplicity",
                    format!("Invalid multiplicity on '{}': {message}", node.name),
                ));
            }
        }
    }
    section_timings.push((
        "4_multiplicity_validation".to_string(),
        t4.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d4),
    ));

    // 5) Import targets should resolve to known namespace/member declarations.
    let t5 = Instant::now();
    let d5 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "import" || import_target_resolves(graph, node) {
            continue;
        }
        let Some(target) = import_target(node) else {
            continue;
        };
        diagnostics.push(diag(
            uri,
            reference_token_range(node, target)
                .unwrap_or_else(|| diagnostic_range(graph, node, None)),
            DiagnosticSeverity::Warning,
            "semantic",
            "unresolved_import_target",
            format!(
                "Imported package/member '{}' could not be resolved in the semantic graph.",
                target
            ),
        ));
    }
    section_timings.push((
        "5_unresolved_import_targets".to_string(),
        t5.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d5),
    ));

    // 6) Stronger typing checks: declarations that name a type should resolve via typing/specializes.
    let t6 = Instant::now();
    let d6 = diagnostics.len();
    let mut unresolved_seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        let Some(type_ref) = declared_type_ref(node) else {
            continue;
        };
        let normalized_type_ref = normalize_declared_type_ref(type_ref);
        if is_builtin_type_ref(&normalized_type_ref) {
            continue;
        }
        let has_resolved_type = !graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty();
        let resolved_via_import_scope = *rule6_resolution_cache
            .entry((node.id.qualified_name.clone(), normalized_type_ref.clone()))
            .or_insert_with(|| {
                !crate::resolve_type_reference_targets(graph, node, type_ref, RULE6_ALLOWED_KINDS)
                    .is_empty()
            });
        let allow_graph_name_fallback = !*import_scope_cache
            .entry(node.id.qualified_name.clone())
            .or_insert_with(|| has_import_in_scope(graph, node));
        let resolved_via_graph_name_fallback = if allow_graph_name_fallback {
            *rule6_graph_name_fallback_cache
                .entry(normalized_type_ref.clone())
                .or_insert_with(|| {
                    graph
                        .nodes_named(&normalized_type_ref)
                        .iter()
                        .any(|candidate| {
                            candidate.id.uri == *uri
                                && matches!(
                                    candidate.element_kind.as_str(),
                                    "part def"
                                        | "port def"
                                        | "interface"
                                        | "item def"
                                        | "attribute def"
                                        | "action def"
                                        | "actor def"
                                        | "occurrence def"
                                        | "flow def"
                                        | "allocation def"
                                        | "state def"
                                        | "requirement def"
                                        | "use case def"
                                        | "concern def"
                                        | "enum def"
                                        | "alias"
                                        | "kermlDecl"
                                        | "view def"
                                        | "viewpoint def"
                                )
                        })
                })
        } else {
            false
        };
        if has_resolved_type
            || resolved_via_import_scope
            || (allow_graph_name_fallback && resolved_via_graph_name_fallback)
        {
            continue;
        }
        let Some(range) = unresolved_type_diagnostic_range(node, type_ref) else {
            continue;
        };
        let key = format!(
            "{}|{}|{}:{}:{}:{}:{}",
            node.id.qualified_name,
            normalized_type_ref,
            range.start.line,
            range.start.character,
            range.end.line,
            range.end.character,
            node.name
        );
        if !unresolved_seen.insert(key) {
            continue;
        }
        let is_ref_usage = node
            .attributes
            .get("refType")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_some();
        let (code, message) = if is_ref_usage {
            (
                "unresolved_ref_type_reference",
                format!(
                    "Reference type '{}' for ref '{}' could not be resolved in the semantic graph (owner: '{}').",
                    type_ref,
                    node.name,
                    node.id.qualified_name
                ),
            )
        } else {
            (
                "unresolved_type_reference",
                format!(
                    "Type reference '{}' for '{}' could not be resolved in the semantic graph.",
                    type_ref, node.name
                ),
            )
        };
        diagnostics.push(diag(
            uri,
            range,
            DiagnosticSeverity::Warning,
            "semantic",
            code,
            message,
        ));
    }
    section_timings.push((
        "6_unresolved_type_references".to_string(),
        t6.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d6),
    ));

    // 7) Specialization references should resolve to known definitions.
    let t7 = Instant::now();
    let d7 = diagnostics.len();
    let mut unresolved_specializes_seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        for specializes_ref in declared_specializes_refs(node) {
            let normalized = normalize_declared_type_ref(&specializes_ref);
            if normalized.is_empty() || is_builtin_type_ref(&normalized) {
                continue;
            }
            let resolved_via_import_scope = *rule7_resolution_cache
                .entry((node.id.qualified_name.clone(), normalized.clone()))
                .or_insert_with(|| {
                    !crate::resolve_type_reference_targets(
                        graph,
                        node,
                        &specializes_ref,
                        SPECIALIZES_TARGET_KINDS,
                    )
                    .is_empty()
                });
            let allow_graph_name_fallback = !*import_scope_cache
                .entry(node.id.qualified_name.clone())
                .or_insert_with(|| has_import_in_scope(graph, node));
            let resolved_via_graph_name_fallback = if allow_graph_name_fallback {
                *rule7_graph_name_fallback_cache
                    .entry(normalized.clone())
                    .or_insert_with(|| {
                        graph.nodes_named(&normalized).iter().any(|candidate| {
                            candidate.id.uri == *uri
                                && SPECIALIZES_TARGET_KINDS
                                    .contains(&candidate.element_kind.as_str())
                        })
                    })
            } else {
                false
            };
            if resolved_via_import_scope
                || (allow_graph_name_fallback && resolved_via_graph_name_fallback)
            {
                continue;
            }
            let Some(range) = unresolved_type_diagnostic_range(node, &specializes_ref) else {
                continue;
            };
            let key = format!(
                "{}|{}|{}:{}:{}:{}:{}",
                node.id.qualified_name,
                normalized,
                range.start.line,
                range.start.character,
                range.end.line,
                range.end.character,
                node.name
            );
            if !unresolved_specializes_seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(uri,
                range,
                DiagnosticSeverity::Warning,
                "semantic",
                "unresolved_specializes_reference",
                format!(
                    "Specializes reference '{}' for '{}' could not be resolved in the semantic graph.",
                    specializes_ref, node.name
                ),
            ));
        }
    }
    section_timings.push((
        "7_unresolved_specializes_references".to_string(),
        t7.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d7),
    ));

    // 8) Redefines consistency, when the parser/graph captures a `redefines` attribute.
    let t8 = Instant::now();
    let d8 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        let Some(redefines_raw) = node.attributes.get("redefines").and_then(|v| v.as_str()) else {
            continue;
        };
        if redefines_raw.trim().is_empty() {
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' has an empty redefines target.", node.name),
            ));
            continue;
        }
        if redefines_raw.trim() == node.id.qualified_name {
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' cannot redefine itself.", node.name),
            ));
        }
    }
    section_timings.push((
        "8_redefines_consistency".to_string(),
        t8.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d8),
    ));

    // 9) Inherited feature value assignment must use explicit redefinition (`:>>`).
    let t9 = Instant::now();
    let d9 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind == "ref" {
            continue;
        }
        if !node.attributes.contains_key("value") || node.attributes.contains_key("redefines") {
            continue;
        }
        let Some(owner_id) = node.parent_id.as_ref() else {
            continue;
        };
        let Some(owner) = graph.get_node(owner_id) else {
            continue;
        };
        let feature_name = node.name.trim();
        if feature_name.is_empty() {
            continue;
        }
        let ResolveResult::Resolved(target_id) =
            resolve_inherited_member_via_type(graph, owner, feature_name)
        else {
            continue;
        };
        let Some(target) = graph.get_node(&target_id) else {
            continue;
        };
        if target.name.trim() != feature_name {
            continue;
        }
        diagnostics.push(diag(uri,
            diagnostic_range(graph, node, Some(target)),
            DiagnosticSeverity::Error,
            "semantic",
            "implicit_redefinition_without_operator",
            format!(
                "Feature '{}' overrides inherited {} '{}' but is missing explicit redefinition ':>>'.",
                feature_name, target.element_kind, target.name
            ),
        ));
    }
    section_timings.push((
        "9_implicit_redefinition_without_operator".to_string(),
        t9.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d9),
    ));

    // 9b) Inherited attribute values must match declared enum types (not string literals).
    let t9b = Instant::now();
    let d9b = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "attribute" {
            continue;
        }
        let Some(value) = node.attributes.get("value").and_then(|v| v.as_str()) else {
            continue;
        };
        if !attribute_value_is_string_literal(value) {
            continue;
        }
        let Some(owner_id) = node.parent_id.as_ref() else {
            continue;
        };
        let Some(owner) = graph.get_node(owner_id) else {
            continue;
        };
        let feature_name = node.name.trim();
        if feature_name.is_empty() {
            continue;
        }
        let ResolveResult::Resolved(target_id) =
            resolve_inherited_member_via_type(graph, owner, feature_name)
        else {
            continue;
        };
        let Some(target) = graph.get_node(&target_id) else {
            continue;
        };
        let Some(type_ref) = declared_type_ref(target) else {
            continue;
        };
        if is_builtin_type_ref(type_ref) {
            continue;
        }
        if !resolves_to_enum_def(graph, target, type_ref) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, Some(target)),
            DiagnosticSeverity::Error,
            "semantic",
            "inherited_attribute_value_type_mismatch",
            format!(
                "Feature '{}' is typed as enum '{}' but was assigned string literal {}; use an enumeration value (for example {type_ref}::approved).",
                feature_name,
                normalize_declared_type_ref(type_ref),
                value.trim()
            ),
        ));
    }
    section_timings.push((
        "9b_inherited_attribute_value_type_mismatch".to_string(),
        t9b.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d9b),
    ));

    // 10) Allocation usage conformance checks.
    let t10 = Instant::now();
    let d10 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "allocation" {
            continue;
        }
        if node.attributes.contains_key("allocationType")
            && graph
                .outgoing_targets_by_kind(node, RelationshipKind::Typing)
                .iter()
                .any(|target| target.element_kind != "allocation def")
        {
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "allocation_type_not_allocation_def",
                format!(
                    "Allocation '{}' has a type that does not resolve to an allocation definition.",
                    node.name
                ),
            ));
        }
        let has_source = node
            .attributes
            .get("allocationSource")
            .and_then(|v| v.as_str())
            .is_some_and(|value| !value.trim().is_empty());
        let has_target = node
            .attributes
            .get("allocationTarget")
            .and_then(|v| v.as_str())
            .is_some_and(|value| !value.trim().is_empty());
        if has_source ^ has_target {
            diagnostics.push(diag(uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "invalid_allocation_endpoints",
                format!(
                    "Allocation '{}' must declare both source and target endpoints when using 'allocate ... to ...'.",
                    node.name
                ),
            ));
        }
    }
    section_timings.push((
        "10_allocation_conformance".to_string(),
        t10.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d10),
    ));

    // 10b) Viewpoint conformance: view satisfy targets must be viewpoints.
    let t10b = Instant::now();
    let d10b = diagnostics.len();
    for (source_qn, target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::Satisfy {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let target_id = crate::NodeId::new(uri, target_qn.clone());
        let Some(source_node) = graph.get_node(&source_id) else {
            continue;
        };
        let Some(target_node) = graph.get_node(&target_id) else {
            continue;
        };
        if !is_view_kind(&source_node.element_kind) {
            continue;
        }
        if is_viewpoint_kind(&target_node.element_kind) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, source_node, Some(target_node)),
            DiagnosticSeverity::Warning,
            "semantic",
            "viewpoint_conformance_invalid_target_kind",
            format!(
                "View '{}' satisfies '{}', but viewpoint conformance targets must be viewpoint definitions/usages (got '{}').",
                source_node.name, target_node.name, target_node.element_kind
            ),
        ));
    }
    section_timings.push((
        "10b_viewpoint_conformance".to_string(),
        t10b.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d10b),
    ));

    // 11) Verdict normalization and domain validation.
    let t11 = Instant::now();
    let d11 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        // Match evaluation: constraint/calc *definitions* are templates; requirement defs
        // can carry inline `require constraint` analysis on the same node.
        let is_analysis_template_def =
            matches!(node.element_kind.as_str(), "constraint def" | "calc def");
        if let Some(status) = node
            .attributes
            .get("analysisEvaluationStatus")
            .and_then(|value| value.as_str())
        {
            if is_analysis_template_def {
                continue;
            }
            if status == "failed_constraint"
                || node
                    .attributes
                    .get("analysisConstraintPassed")
                    .and_then(|value| value.as_bool())
                    == Some(false)
            {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "analysis_constraint_failed",
                    format!(
                        "Analysis constraint(s) on '{}' evaluated to false.",
                        node.name
                    ),
                ));
            } else if status == "incomplete" {
                diagnostics.push(diag(uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Information,
                    "semantic",
                    "analysis_evaluation_incomplete",
                    format!(
                        "Analysis constraint(s) on '{}' depend on declared value(s) that have not been assigned.",
                        node.name
                    ),
                ));
            } else if status != "ok" {
                let detail = node
                    .attributes
                    .get("analysisEvaluationError")
                    .and_then(|value| value.as_str())
                    .unwrap_or("analysis expression could not be evaluated");
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "analysis_evaluation_unresolved",
                    format!(
                        "Could not evaluate analysis expression(s) on '{}': {detail}",
                        node.name
                    ),
                ));
            }
        }
    }
    section_timings.push((
        "11_analysis_evaluation_status".to_string(),
        t11.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d11),
    ));

    // 12) Verdict normalization and domain validation.
    let t12 = Instant::now();
    let d12 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "verdict" {
            continue;
        }
        let Some(raw_token) = node
            .attributes
            .get("rawVerdictToken")
            .and_then(|v| v.as_str())
        else {
            continue;
        };
        let normalized = raw_token.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }
        if !matches!(
            normalized.as_str(),
            "pass" | "fail" | "inconclusive" | "error"
        ) {
            diagnostics.push(diag(uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "invalid_verdict_value",
                format!(
                    "Verdict '{}' is not in the SysML verdict domain (pass, fail, inconclusive, error).",
                    raw_token
                ),
            ));
        }
    }
    section_timings.push((
        "12_verdict_domain_validation".to_string(),
        t12.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d12),
    ));

    // 13) Case-kind objective binding diagnostics.
    let t13 = Instant::now();
    let d13 = diagnostics.len();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "objective" {
            continue;
        }
        let Some(binding_kind) = node
            .attributes
            .get("objectiveBindingKind")
            .and_then(|v| v.as_str())
        else {
            continue;
        };
        if node.attributes.contains_key("objectiveBoundTo") {
            continue;
        }
        if binding_kind == "case_result_default" {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "objective_binding_unresolved",
            objective_binding_unresolved_message(&node.name, binding_kind),
        ));
    }
    section_timings.push((
        "13_objective_binding".to_string(),
        t13.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d13),
    ));

    let t14 = Instant::now();
    let d14 = diagnostics.len();
    super::pending_relationship_diagnostics::append_unresolved_pending_relationship_diagnostics(
        graph,
        uri,
        &mut diagnostics,
    );
    section_timings.push((
        "14_unresolved_pending_relationships".to_string(),
        t14.elapsed().as_millis(),
        diagnostics.len().saturating_sub(d14),
    ));

    if std::env::var("SEMANTIC_CORE_TIMING")
        .map(|value| {
            let value = value.trim().to_ascii_lowercase();
            value == "1" || value == "true" || value == "yes" || value == "on"
        })
        .unwrap_or(false)
    {
        section_timings.sort_by_key(|(_, ms, _)| std::cmp::Reverse(*ms));
        let top_sections = section_timings
            .iter()
            .take(6)
            .map(|(name, ms, count)| format!("{name}:{ms}ms:{count}diag"))
            .collect::<Vec<_>>()
            .join(" | ");
        println!(
            "TIMING semantic_diag_rule_breakdown uri={} total_ms={} total_diags={} top6={}",
            uri,
            total_start.elapsed().as_millis(),
            diagnostics.len(),
            top_sections
        );
    }

    diagnostics
}

fn extract_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn resolved_endpoint_related_information(
    graph: &SemanticGraph,
    diagnostic_node: &crate::SemanticNode,
) -> Option<DiagnosticRelatedInfo> {
    let qn = diagnostic_node
        .attributes
        .get("resolvedEndpoint")
        .and_then(|value| value.as_str())?;
    let id = crate::NodeId::new(&diagnostic_node.id.uri, qn);
    let node = graph.get_node(&id)?;
    Some(DiagnosticRelatedInfo {
        uri: node.id.uri.clone(),
        range: node.range,
        message: format!(
            "Resolved endpoint '{}' ({})",
            node.id.qualified_name, node.element_kind
        ),
    })
}
