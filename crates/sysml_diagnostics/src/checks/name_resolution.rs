use std::collections::{HashMap, HashSet};

use url::Url;

use crate::checks::import_resolution::has_import_in_scope;
use crate::helpers::{
    declared_specializes_refs, declared_type_ref, diag, diagnostic_range, is_builtin_type_ref,
    is_synthetic, normalize_declared_type_ref, unresolved_type_diagnostic_range,
};
use crate::types::DiagnosticSeverity;
use crate::SemanticDiagnostic;
use sysml_model::semantic::import_resolution::resolve_imported_node_ids_for_simple_name;
use sysml_model::semantic::kinds::{
    is_metadata_restriction_attribute, is_namespace, RULE6_ALLOWED_KINDS,
};
use sysml_model::semantic::model::node_matches_simple_name;
use sysml_model::semantic::relationships::SPECIALIZES_TARGET_KINDS;
use sysml_model::{resolve_type_reference_targets, ElementKind, SemanticGraph, SemanticNode};

fn is_def_or_usage_kind(kind: &sysml_model::ElementKind) -> bool {
    matches!(
        kind,
        sysml_model::ElementKind::PartDef
            | sysml_model::ElementKind::PortDef
            | sysml_model::ElementKind::ItemDef
            | sysml_model::ElementKind::AttributeDef
            | sysml_model::ElementKind::ActionDef
            | sysml_model::ElementKind::StateDef
            | sysml_model::ElementKind::RequirementDef
            | sysml_model::ElementKind::UseCaseDef
            | sysml_model::ElementKind::AnalysisDef
            | sysml_model::ElementKind::VerificationDef
            | sysml_model::ElementKind::ViewDef
            | sysml_model::ElementKind::ViewpointDef
            | sysml_model::ElementKind::ConcernDef
            | sysml_model::ElementKind::EnumDef
            | sysml_model::ElementKind::MetadataDef
            | sysml_model::ElementKind::InterfaceDef
            | sysml_model::ElementKind::Package
    )
}

fn invalid_qualified_name_segment(
    graph: &SemanticGraph,
    type_ref: &str,
) -> Option<(String, String)> {
    let normalized = normalize_declared_type_ref(type_ref);
    if !normalized.contains("::") {
        return None;
    }
    let segments: Vec<&str> = normalized.split("::").filter(|s| !s.is_empty()).collect();
    if segments.len() < 2 {
        return None;
    }
    for end in 0..segments.len() - 1 {
        let prefix = segments[..=end].join("::");
        let Some(ids) = graph.node_ids_for_qualified_name(&prefix) else {
            return Some((
                segments[end].to_string(),
                format!("segment '{}' does not resolve", segments[end]),
            ));
        };
        if ids.is_empty() {
            return Some((
                segments[end].to_string(),
                format!("segment '{}' does not resolve", segments[end]),
            ));
        }
        let namespace_ok = ids.iter().all(|id| {
            graph
                .get_node(id)
                .map(|node| is_namespace(&node.element_kind))
                .unwrap_or(false)
        });
        if !namespace_ok {
            return Some((
                segments[end].to_string(),
                format!(
                    "segment '{}' is not a namespace (resolved to '{}')",
                    segments[end],
                    graph
                        .get_node(&ids[0])
                        .map(|n| n.element_kind.as_str())
                        .unwrap_or("unknown")
                ),
            ));
        }
    }
    None
}

fn is_ambiguous_simple_name(graph: &SemanticGraph, node: &SemanticNode, name: &str) -> bool {
    if name.contains("::") {
        return false;
    }
    let imported = resolve_imported_node_ids_for_simple_name(graph, node, name);
    if imported.len() > 1 {
        return true;
    }
    let local_matches: Vec<_> = graph
        .nodes_by_uri
        .get(&node.id.uri)
        .into_iter()
        .flatten()
        .filter_map(|id| graph.get_node(id))
        .filter(|candidate| {
            node_matches_simple_name(candidate, name)
                && candidate.id.uri == node.id.uri
                && is_def_or_usage_kind(&candidate.element_kind)
        })
        .collect();
    imported.len() + local_matches.len() > 1
}

fn collect_duplicate_namespace_members(
    graph: &SemanticGraph,
    uri: &Url,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    let mut seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if !matches!(
            node.element_kind,
            sysml_model::ElementKind::Package
                | sysml_model::ElementKind::PartDef
                | sysml_model::ElementKind::RequirementDef
                | sysml_model::ElementKind::UseCaseDef
        ) {
            continue;
        }
        let mut counts: HashMap<(String, String), usize> = HashMap::new();
        for child in graph.children_of(node) {
            if matches!(
                child.element_kind,
                sysml_model::ElementKind::Import
                    | sysml_model::ElementKind::Diagnostic
                    | sysml_model::ElementKind::Filter
            ) || matches!(child.element_kind.as_str(), "doc" | "comment")
            {
                continue;
            }
            if child.name.trim().is_empty() || child.name.starts_with('_') {
                continue;
            }
            if child.element_kind == sysml_model::ElementKind::Alias {
                continue;
            }
            *counts
                .entry((child.name.clone(), child.element_kind.as_str().to_string()))
                .or_default() += 1;
        }
        for ((name, kind), count) in counts {
            if count < 2 {
                continue;
            }
            let key = format!("{}|{}|{}|{}", node.id.qualified_name, name, kind, count);
            if !seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "duplicate_namespace_member",
                format!(
                    "Namespace '{}' declares '{}' ({}) {} times; member names must be unique within a namespace.",
                    node.name, name, kind, count
                ),
            ));
        }
    }
}

pub(crate) fn collect_name_resolution_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut import_scope_cache = HashMap::<String, bool>::new();
    let mut rule6_resolution_cache = HashMap::<(String, String), bool>::new();
    let mut rule7_resolution_cache = HashMap::<(String, String), bool>::new();
    let mut rule6_graph_name_fallback_cache = HashMap::<String, bool>::new();
    let mut rule7_graph_name_fallback_cache = HashMap::<String, bool>::new();
    let mut unresolved_seen: HashSet<String> = HashSet::new();
    let mut unresolved_specializes_seen: HashSet<String> = HashSet::new();
    let mut ambiguous_seen: HashSet<String> = HashSet::new();
    let mut invalid_qn_seen: HashSet<String> = HashSet::new();

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

        if is_metadata_restriction_attribute(node) {
            continue;
        }

        if let Some((segment, reason)) = invalid_qualified_name_segment(graph, type_ref) {
            let key = format!("{}|{}|{}", node.id.qualified_name, type_ref, segment);
            if invalid_qn_seen.insert(key) {
                let range = unresolved_type_diagnostic_range(node, type_ref)
                    .unwrap_or_else(|| diagnostic_range(graph, node, None));
                diagnostics.push(diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "invalid_qualified_name_segment",
                    format!(
                        "Qualified name '{}' has invalid segment '{}': {reason}.",
                        type_ref, segment
                    ),
                ));
            }
            continue;
        }

        let has_resolved_type = !graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty();
        let resolved_via_import_scope = *rule6_resolution_cache
            .entry((node.id.qualified_name.clone(), normalized_type_ref.clone()))
            .or_insert_with(|| {
                !resolve_type_reference_targets(graph, node, type_ref, RULE6_ALLOWED_KINDS)
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
                                && RULE6_ALLOWED_KINDS.contains(&candidate.element_kind)
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

        // Connection ends redefined via `::>` (BNF-derived syntax) point at a nested feature
        // path (e.g. `sensorAcquisition.run.lidarScanOut`), not a type name. `flow` statement
        // endpoints already accept these same dotted feature chains without validating them as
        // type references (see `add_expression_edge_if_both_exist`'s non-`Connection` branch,
        // which silently no-ops rather than diagnosing an unresolved flow endpoint); treat
        // connection ends the same way instead of flagging the path as an unresolved type.
        if node.element_kind == ElementKind::InterfaceEnd && type_ref.contains('.') {
            continue;
        }

        let lookup_name = normalized_type_ref
            .rsplit("::")
            .next()
            .unwrap_or(normalized_type_ref.as_str());
        if is_ambiguous_simple_name(graph, node, lookup_name) {
            let key = format!("{}|{}", node.id.qualified_name, lookup_name);
            if ambiguous_seen.insert(key) {
                let range = unresolved_type_diagnostic_range(node, type_ref)
                    .unwrap_or_else(|| diagnostic_range(graph, node, None));
                diagnostics.push(diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "ambiguous_name_reference",
                    format!(
                        "Reference '{}' for '{}' is ambiguous in the current scope; use a qualified name.",
                        lookup_name, node.name
                    ),
                ));
            }
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

    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        for specializes_ref in declared_specializes_refs(node) {
            let normalized = normalize_declared_type_ref(&specializes_ref);
            if normalized.is_empty() || is_builtin_type_ref(&normalized) {
                continue;
            }
            if let Some((segment, reason)) = invalid_qualified_name_segment(graph, &specializes_ref)
            {
                let key = format!("{}|{}|{}", node.id.qualified_name, specializes_ref, segment);
                if invalid_qn_seen.insert(key) {
                    let range = unresolved_type_diagnostic_range(node, &specializes_ref)
                        .unwrap_or_else(|| diagnostic_range(graph, node, None));
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "invalid_qualified_name_segment",
                        format!(
                            "Qualified specializes name '{}' has invalid segment '{}': {reason}.",
                            specializes_ref, segment
                        ),
                    ));
                }
                continue;
            }
            let resolved_via_import_scope = *rule7_resolution_cache
                .entry((node.id.qualified_name.clone(), normalized.clone()))
                .or_insert_with(|| {
                    !resolve_type_reference_targets(
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
                                && SPECIALIZES_TARGET_KINDS.contains(&candidate.element_kind)
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

            let lookup_name = normalized
                .rsplit("::")
                .next()
                .unwrap_or(normalized.as_str());
            if is_ambiguous_simple_name(graph, node, lookup_name) {
                let key = format!("{}|specializes|{}", node.id.qualified_name, lookup_name);
                if ambiguous_seen.insert(key) {
                    let range = unresolved_type_diagnostic_range(node, &specializes_ref)
                        .unwrap_or_else(|| diagnostic_range(graph, node, None));
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "ambiguous_name_reference",
                        format!(
                            "Specializes reference '{}' for '{}' is ambiguous; use a qualified name.",
                            lookup_name, node.name
                        ),
                    ));
                }
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
            diagnostics.push(diag(
                uri,
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

    collect_duplicate_namespace_members(graph, uri, &mut diagnostics);
    diagnostics
}
