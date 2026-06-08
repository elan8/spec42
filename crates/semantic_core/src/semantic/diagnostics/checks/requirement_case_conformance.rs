use std::collections::HashSet;

use url::Url;

use crate::semantic::diagnostics::helpers::{diag, diagnostic_range, is_synthetic};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::model::RelationshipKind;
use crate::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::{resolve_type_reference_targets, ResolveResult, SemanticDiagnostic, SemanticGraph};

const VERIFIED_REQUIREMENT_TARGET_KINDS: &[&str] = &["requirement def", "requirement"];

fn is_requirement_kind(kind: &str) -> bool {
    matches!(kind, "requirement" | "requirement def")
}

fn is_use_case_kind(kind: &str) -> bool {
    matches!(kind, "use case" | "use case def")
}

fn is_view_kind(kind: &str) -> bool {
    matches!(kind, "view" | "view def")
}

fn is_viewpoint_kind(kind: &str) -> bool {
    matches!(kind, "viewpoint" | "viewpoint def")
}

fn container_prefix_for(node: &crate::SemanticNode) -> Option<&str> {
    node.id
        .qualified_name
        .rsplit_once("::")
        .map(|(prefix, _)| prefix)
}

pub(in crate::semantic::diagnostics) fn collect_requirement_case_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for (source_qn, target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::Satisfy {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let target_id = crate::NodeId::new(uri, target_qn.clone());
        let (Some(source_node), Some(target_node)) =
            (graph.get_node(&source_id), graph.get_node(&target_id))
        else {
            continue;
        };
        if is_view_kind(&source_node.element_kind) {
            continue;
        }
        let valid = if is_requirement_kind(&source_node.element_kind) {
            is_requirement_kind(&target_node.element_kind)
        } else if is_use_case_kind(&source_node.element_kind) {
            is_use_case_kind(&target_node.element_kind)
        } else {
            continue;
        };
        if valid || is_viewpoint_kind(&target_node.element_kind) {
            continue;
        }
        let key = format!("satisfy|{source_qn}|{target_qn}");
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, source_node, Some(target_node)),
            DiagnosticSeverity::Warning,
            "semantic",
            "satisfy_invalid_endpoint_kind",
            format!(
                "Satisfy relationship from '{}' ({}) to '{}' ({}) has incompatible endpoint kinds.",
                source_node.name,
                source_node.element_kind,
                target_node.name,
                target_node.element_kind
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "verified requirement" || is_synthetic(node) {
            continue;
        }
        let Some(requirement_ref) = node
            .attributes
            .get("verifiedRequirement")
            .and_then(|v| v.as_str())
        else {
            continue;
        };
        let targets = resolve_type_reference_targets(
            graph,
            node,
            requirement_ref,
            VERIFIED_REQUIREMENT_TARGET_KINDS,
        );
        if !targets.is_empty() {
            continue;
        }
        let key = format!("verified|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "verified_requirement_invalid_target",
            format!(
                "Verified requirement '{}' must resolve to a requirement definition or usage.",
                requirement_ref
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "verify" || is_synthetic(node) {
            continue;
        }
        let Some(lhs) = node.attributes.get("lhs").and_then(|v| v.as_str()) else {
            continue;
        };
        let lhs = lhs.trim();
        if lhs.is_empty() {
            continue;
        }
        let prefix = node
            .parent_id
            .as_ref()
            .and_then(|id| graph.get_node(id))
            .and_then(container_prefix_for);
        if matches!(
            resolve_expression_endpoint_strict(graph, uri, prefix, lhs),
            ResolveResult::Resolved(_)
        ) {
            continue;
        }
        let key = format!("assign|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "assignment_target_unresolved",
            format!(
                "Assignment target '{}' on verification case '{}' does not resolve to an assignable feature.",
                lhs, node.parent_id.as_ref().and_then(|id| graph.get_node(id)).map(|n| n.name.as_str()).unwrap_or("case")
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "include use case" || is_synthetic(node) {
            continue;
        }
        let allowed = ["use case def", "use case"];
        let Some(include_target) = node
            .attributes
            .get("includeTarget")
            .and_then(|v| v.as_str())
        else {
            continue;
        };
        let targets =
            resolve_type_reference_targets(graph, node, include_target, &allowed);
        if !targets.is_empty() {
            continue;
        }
        let key = format!("include|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "use_case_include_invalid_target",
            format!(
                "Include use case target '{}' must resolve to a use case definition or usage.",
                include_target
            ),
        ));
    }

    diagnostics
}
