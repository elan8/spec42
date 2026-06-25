use std::collections::HashSet;

use url::Url;

use crate::semantic::diagnostics::helpers::{diag, diagnostic_range, is_synthetic};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::model::RelationshipKind;
use crate::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::{resolve_type_reference_targets, ResolveResult, SemanticDiagnostic, SemanticGraph};

const VERIFIED_REQUIREMENT_TARGET_KINDS: &[&str] = &["requirement def", "requirement"];

fn is_requirement_kind(kind: &crate::ElementKind) -> bool {
    matches!(kind.as_str(), "requirement" | "requirement def")
}

fn is_requirement_satisfy_target_kind(kind: &crate::ElementKind) -> bool {
    matches!(
        kind.as_str(),
        "requirement"
            | "requirement def"
            | "part"
            | "part def"
            | "action"
            | "action def"
            | "port"
            | "port def"
            | "interface"
            | "attribute"
            | "attribute def"
            | "item"
            | "item def"
            | "flow"
            | "flow def"
            | "state"
            | "state def"
    )
}

fn is_use_case_kind(kind: &crate::ElementKind) -> bool {
    matches!(kind.as_str(), "use case" | "use case def")
}

fn is_view_kind(kind: &crate::ElementKind) -> bool {
    matches!(kind.as_str(), "view" | "view def")
}

fn is_viewpoint_kind(kind: &crate::ElementKind) -> bool {
    matches!(kind.as_str(), "viewpoint" | "viewpoint def")
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
            is_requirement_satisfy_target_kind(&target_node.element_kind)
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
        let targets = resolve_type_reference_targets(graph, node, include_target, &allowed);
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

    for node in graph.nodes_for_uri(uri) {
        if !matches!(
            node.element_kind.as_str(),
            "requirement" | "requirement def"
        ) || is_synthetic(node)
        {
            continue;
        }
        let Some(constraints) = node.attributes.get("analysisConstraints") else {
            continue;
        };
        let items = match constraints {
            serde_json::Value::Array(items) => items,
            _ => continue,
        };
        for item in items {
            let Some(kind) = item.get("kind").and_then(|v| v.as_str()) else {
                continue;
            };
            if kind != "require_constraint" {
                continue;
            }
            let expression = item
                .get("expression")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim();
            if expression.is_empty() {
                let key = format!("constraint_expr|{}", node.id.qualified_name);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, node, None),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "requirement_constraint_invalid_membership",
                        format!(
                            "Requirement '{}' has a require constraint without an expression body.",
                            node.name
                        ),
                    ));
                }
                continue;
            }
            if let Some(params) = item.get("params").and_then(|v| v.as_array()) {
                for param in params {
                    let direction = param
                        .get("direction")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let param_type = param
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim();
                    if !matches!(direction, "in" | "out" | "inout") || param_type.is_empty() {
                        let key = format!(
                            "constraint_param|{}|{}",
                            node.id.qualified_name,
                            param.get("name").and_then(|v| v.as_str()).unwrap_or("")
                        );
                        if seen.insert(key) {
                            diagnostics.push(diag(
                                uri,
                                diagnostic_range(graph, node, None),
                                DiagnosticSeverity::Warning,
                                "semantic",
                                "requirement_constraint_invalid_membership",
                                format!(
                                    "Requirement '{}' require constraint parameter is missing direction or type.",
                                    node.name
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "verification def" && node.element_kind != "verification" {
            continue;
        }
        let verdict_count = node
            .attributes
            .get("verdictCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        let objective_count = node
            .attributes
            .get("objectiveCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        let verified_count = graph
            .children_of(node)
            .into_iter()
            .filter(|child| child.element_kind == "verified requirement")
            .count();

        if verdict_count > 1 {
            let key = format!("verdict_multi|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "verification_case_invalid_shape",
                    format!(
                        "Verification case '{}' declares multiple verdict/return clauses.",
                        node.name
                    ),
                ));
            }
        }
        if verified_count > 0 && objective_count == 0 {
            let key = format!("objective_missing|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "verification_case_invalid_shape",
                    format!(
                        "Verification case '{}' verifies requirements but declares no objective.",
                        node.name
                    ),
                ));
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if !matches!(
            node.element_kind.as_str(),
            "verification def" | "verification" | "analysis def" | "analysis"
        ) {
            continue;
        }
        let has_subject = node
            .attributes
            .get("hasSubject")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let subject_count = graph
            .children_of(node)
            .into_iter()
            .filter(|child| child.element_kind == "subject")
            .count();
        let objectives: Vec<_> = graph
            .children_of(node)
            .into_iter()
            .filter(|child| child.element_kind == "objective")
            .collect();

        let needs_subject = objectives.iter().any(|objective| {
            objective
                .attributes
                .get("objectiveBindingKind")
                .and_then(|v| v.as_str())
                == Some("verification_subject")
        });
        if needs_subject && !has_subject {
            let key = format!("case_subject|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "case_subject_missing",
                    format!(
                        "Case '{}' has objectives bound to a subject but no subject is declared.",
                        node.name
                    ),
                ));
            }
        }
        if needs_subject && subject_count > 1 {
            let key = format!("case_subject_multi|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "case_objective_binding_cardinality",
                    format!(
                        "Case '{}' declares {subject_count} subjects but objectives expect a single subject.",
                        node.name
                    ),
                ));
            }
        }

        let needs_analysis_result = objectives.iter().any(|objective| {
            objective
                .attributes
                .get("objectiveBindingKind")
                .and_then(|v| v.as_str())
                == Some("analysis_result")
        });
        if needs_analysis_result {
            let analysis_result_count = node
                .attributes
                .get("analysisResultCount")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let local_results = graph
                .children_of(node)
                .into_iter()
                .filter(|child| child.element_kind == "analysis result")
                .count();
            let has_inherited_result = node.attributes.contains_key("analysisExpression")
                || objectives.iter().any(|objective| {
                    objective
                        .attributes
                        .get("objectiveBoundTo")
                        .and_then(|v| v.as_str())
                        .is_some_and(|value| !value.trim().is_empty())
                });
            let total = analysis_result_count.max(local_results as u64);
            if total == 0 && !has_inherited_result {
                let key = format!("analysis_result|{}", node.id.qualified_name);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, node, None),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "case_objective_binding_cardinality",
                        format!(
                            "Analysis case '{}' has objectives bound to analysis result but no return ref is declared.",
                            node.name
                        ),
                    ));
                }
            }
        }
    }

    diagnostics
}
