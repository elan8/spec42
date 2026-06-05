use std::collections::HashSet;

use url::Url;

use crate::semantic::diagnostics::helpers::{
    attribute_value_is_string_literal, declared_specializes_refs, declared_type_ref, diag,
    diagnostic_range, is_builtin_type_ref, is_synthetic, multiplicity_issue_message,
    normalize_declared_type_ref, parse_non_negative_bound, reference_token_range,
    resolves_to_enum_def, unresolved_type_diagnostic_range,
};
use crate::semantic::diagnostics::kind_rules::{
    allowed_specializes_target_kinds, allowed_subset_redefine_target_kinds,
    allowed_typing_target_kinds, is_compatible_kind,
};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::relationships::SPECIALIZES_TARGET_KINDS;
use crate::{
    resolve_inherited_member_via_type, resolve_type_reference_targets, ResolveResult,
    SemanticDiagnostic, SemanticGraph,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MultiplicityBounds {
    lower: i64,
    upper: Option<i64>,
}

fn parse_multiplicity_bounds(raw: Option<&str>) -> Option<MultiplicityBounds> {
    let raw = raw?.trim();
    if raw.is_empty() || multiplicity_issue_message(raw).is_some() {
        return None;
    }
    let normalized = raw
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();
    if let Some((lower_raw, upper_raw)) = normalized.split_once("..") {
        let lower = parse_non_negative_bound(lower_raw.trim()).ok()?;
        let upper = if upper_raw.trim() == "*" {
            None
        } else {
            Some(parse_non_negative_bound(upper_raw.trim()).ok()?)
        };
        return Some(MultiplicityBounds { lower, upper });
    }
    if normalized == "*" {
        return Some(MultiplicityBounds {
            lower: 0,
            upper: None,
        });
    }
    let exact = parse_non_negative_bound(normalized).ok()?;
    Some(MultiplicityBounds {
        lower: exact,
        upper: Some(exact),
    })
}

fn multiplicity_widens(child: MultiplicityBounds, parent: MultiplicityBounds) -> bool {
    if child.lower < parent.lower {
        return true;
    }
    match (child.upper, parent.upper) {
        (None, Some(_)) => true,
        (Some(child_upper), Some(parent_upper)) => child_upper > parent_upper,
        _ => false,
    }
}

fn collect_specialization_cycles(graph: &SemanticGraph, uri: &Url) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen_reports = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        let mut stack = vec![node.id.clone()];
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        fn dfs(
            graph: &SemanticGraph,
            current_id: &crate::NodeId,
            origin: &crate::NodeId,
            stack: &mut Vec<crate::NodeId>,
            visiting: &mut HashSet<crate::NodeId>,
            visited: &mut HashSet<crate::NodeId>,
            diagnostics: &mut Vec<SemanticDiagnostic>,
            seen_reports: &mut HashSet<String>,
            uri: &Url,
        ) {
            if !visiting.insert(current_id.clone()) {
                if stack.iter().any(|id| id == current_id) {
                    let key = format!("{}|{}", origin.qualified_name, current_id.qualified_name);
                    if seen_reports.insert(key) {
                        if let Some(node) = graph.get_node(origin) {
                            diagnostics.push(diag(
                                uri,
                                diagnostic_range(graph, node, None),
                                DiagnosticSeverity::Error,
                                "semantic",
                                "specialization_cycle",
                                format!(
                                    "Specialization cycle detected involving '{}' and '{}'.",
                                    origin.qualified_name, current_id.qualified_name
                                ),
                            ));
                        }
                    }
                }
                return;
            }
            if !visited.insert(current_id.clone()) {
                visiting.remove(current_id);
                return;
            }
            stack.push(current_id.clone());
            if let Some(node) = graph.get_node(current_id) {
                for specializes_ref in declared_specializes_refs(node) {
                    for target_id in resolve_type_reference_targets(
                        graph,
                        node,
                        &specializes_ref,
                        SPECIALIZES_TARGET_KINDS,
                    ) {
                        dfs(
                            graph,
                            &target_id,
                            origin,
                            stack,
                            visiting,
                            visited,
                            diagnostics,
                            seen_reports,
                            uri,
                        );
                    }
                }
            }
            stack.pop();
            visiting.remove(current_id);
        }

        dfs(
            graph,
            &node.id,
            &node.id,
            &mut stack,
            &mut visiting,
            &mut visited,
            &mut diagnostics,
            &mut seen_reports,
            uri,
        );
    }

    diagnostics
}

pub(in crate::semantic::diagnostics) fn collect_kind_compatibility_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }

        if let Some(type_ref) = declared_type_ref(node) {
            let normalized = normalize_declared_type_ref(type_ref);
            if !is_builtin_type_ref(&normalized) {
                for target in graph.outgoing_typing_or_specializes_targets(node) {
                    let allowed = allowed_typing_target_kinds(&node.element_kind);
                    if !allowed.is_empty()
                        && !is_compatible_kind(&target.element_kind, allowed)
                    {
                        let key = format!(
                            "type|{}|{}|{}",
                            node.id.qualified_name, type_ref, target.element_kind
                        );
                        if seen.insert(key) {
                            let range = unresolved_type_diagnostic_range(node, type_ref)
                                .unwrap_or_else(|| diagnostic_range(graph, node, None));
                            diagnostics.push(diag(
                                uri,
                                range,
                                DiagnosticSeverity::Warning,
                                "semantic",
                                "incompatible_type_kind",
                                format!(
                                    "'{}' cannot type '{}' with '{}'; expected a compatible {} definition.",
                                    node.element_kind,
                                    node.name,
                                    type_ref,
                                    node.element_kind.trim_end_matches(" def")
                                ),
                            ));
                        }
                    }
                }
            }
        }

        for specializes_ref in declared_specializes_refs(node) {
            let normalized = normalize_declared_type_ref(&specializes_ref);
            if normalized.is_empty() || is_builtin_type_ref(&normalized) {
                continue;
            }
            for target in resolve_type_reference_targets(
                graph,
                node,
                &specializes_ref,
                SPECIALIZES_TARGET_KINDS,
            )
            .into_iter()
            .filter_map(|id| graph.get_node(&id))
            {
                let allowed = allowed_specializes_target_kinds(&node.element_kind);
                if !allowed.is_empty()
                    && !is_compatible_kind(&target.element_kind, allowed)
                {
                    let key = format!(
                        "specializes|{}|{}|{}",
                        node.id.qualified_name, specializes_ref, target.element_kind
                    );
                    if seen.insert(key) {
                        let range = unresolved_type_diagnostic_range(node, &specializes_ref)
                            .unwrap_or_else(|| diagnostic_range(graph, node, None));
                        diagnostics.push(diag(
                            uri,
                            range,
                            DiagnosticSeverity::Warning,
                            "semantic",
                            "incompatible_specializes_kind",
                            format!(
                                "'{}' cannot specialize '{}' (resolved to '{}').",
                                node.name, specializes_ref, target.element_kind
                            ),
                        ));
                    }
                }
            }
        }

        let redefines_target = node
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .or_else(|| {
                if node.attributes.contains_key("value")
                    && node.parent_id.as_ref().and_then(|id| graph.get_node(id)).is_some_and(
                        |owner| !graph.outgoing_typing_or_specializes_targets(owner).is_empty(),
                    )
                {
                    Some(node.name.clone())
                } else {
                    None
                }
            });
        if let Some(trimmed) = redefines_target {
            if trimmed == node.id.qualified_name {
                continue;
            }
            let Some(owner_id) = node.parent_id.as_ref() else {
                continue;
            };
            let Some(owner) = graph.get_node(owner_id) else {
                continue;
            };
            match resolve_inherited_member_via_type(graph, owner, &trimmed) {
                ResolveResult::Resolved(target_id) => {
                    if let Some(target) = graph.get_node(&target_id) {
                        let allowed = allowed_subset_redefine_target_kinds(&node.element_kind);
                        if !allowed.is_empty()
                            && !is_compatible_kind(&target.element_kind, allowed)
                        {
                            let key =
                                format!("subset|{}|{}", node.id.qualified_name, target.element_kind);
                            if seen.insert(key) {
                                diagnostics.push(diag(
                                    uri,
                                    diagnostic_range(graph, node, Some(target)),
                                    DiagnosticSeverity::Warning,
                                    "semantic",
                                    "incompatible_subset_redefine_kind",
                                    format!(
                                        "'{}' cannot redefine/subset '{}' (target kind '{}').",
                                        node.name, trimmed, target.element_kind
                                    ),
                                ));
                            }
                        }

                        let child_bounds = parse_multiplicity_bounds(
                            node.attributes.get("multiplicity").and_then(|v| v.as_str()),
                        );
                        let parent_bounds = parse_multiplicity_bounds(
                            target.attributes.get("multiplicity").and_then(|v| v.as_str()),
                        );
                        if let (Some(child), Some(parent)) = (child_bounds, parent_bounds) {
                            if multiplicity_widens(child, parent) {
                                let key = format!("mult|{}", node.id.qualified_name);
                                if seen.insert(key) {
                                    diagnostics.push(diag(
                                        uri,
                                        diagnostic_range(graph, node, Some(target)),
                                        DiagnosticSeverity::Error,
                                        "semantic",
                                        "redefinition_multiplicity_widened",
                                        format!(
                                            "Feature '{}' widens inherited multiplicity on '{}'.",
                                            node.name, target.name
                                        ),
                                    ));
                                }
                            }
                        }

                        if node.element_kind == "attribute" {
                            if let Some(value) =
                                node.attributes.get("value").and_then(|v| v.as_str())
                            {
                                if attribute_value_is_string_literal(value) {
                                    if let Some(type_ref) = declared_type_ref(target) {
                                        if resolves_to_enum_def(graph, target, type_ref) {
                                            let key = format!("enum|{}", node.id.qualified_name);
                                            if seen.insert(key) {
                                                diagnostics.push(diag(
                                                    uri,
                                                    diagnostic_range(graph, node, Some(target)),
                                                    DiagnosticSeverity::Error,
                                                    "semantic",
                                                    "redefinition_type_incompatible",
                                                    format!(
                                                        "Feature '{}' assigns string literal {} but inherited '{}' is enum-typed as {}.",
                                                        node.name,
                                                        value.trim(),
                                                        target.name,
                                                        normalize_declared_type_ref(type_ref)
                                                    ),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            if let (Some(child_type), Some(parent_type)) = (
                                declared_type_ref(node),
                                declared_type_ref(target),
                            ) {
                                let child_norm = normalize_declared_type_ref(child_type);
                                let parent_norm = normalize_declared_type_ref(parent_type);
                                if !child_norm.is_empty()
                                    && !parent_norm.is_empty()
                                    && child_norm != parent_norm
                                    && !is_builtin_type_ref(&child_norm)
                                {
                                    let key = format!("rtype|{}", node.id.qualified_name);
                                    if seen.insert(key) {
                                        diagnostics.push(diag(
                                            uri,
                                            reference_token_range(node, child_type)
                                                .unwrap_or_else(|| diagnostic_range(graph, node, None)),
                                            DiagnosticSeverity::Error,
                                            "semantic",
                                            "redefinition_type_incompatible",
                                            format!(
                                                "Feature '{}' type '{}' is not conformant with inherited type '{}'.",
                                                node.name, child_norm, parent_norm
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                    let key = format!("redefines|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, None),
                            DiagnosticSeverity::Warning,
                            "semantic",
                            "unresolved_redefines_target",
                            format!(
                                "Redefines target '{}' on '{}' could not be resolved.",
                                trimmed, node.name
                            ),
                        ));
                    }
                }
            }
        }
    }

    diagnostics.extend(collect_specialization_cycles(graph, uri));
    diagnostics
}
