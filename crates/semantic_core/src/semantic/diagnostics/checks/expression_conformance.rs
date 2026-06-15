use std::collections::HashSet;

use url::Url;

use crate::semantic::diagnostics::helpers::{
    attribute_value_is_string_literal, condition_expression_is_boolean, declared_type_ref, diag,
    diagnostic_range, is_boolean_literal_value, is_synthetic, normalize_declared_type_ref,
    resolves_to_enum_def,
};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::semantic::units::{
    is_measurement_unit_compatible, quantity_value_to_unit_type_name,
    unit_type_for_quantity_type_name,
};
use crate::ResolveResult;
use crate::UnitRegistry;
use crate::{SemanticDiagnostic, SemanticGraph};

fn is_boolean_literal(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "false")
}

fn resolved_scalar_kind(graph: &SemanticGraph, node: &crate::SemanticNode) -> Option<&'static str> {
    let mut candidates = Vec::new();
    if let Some(type_ref) = declared_type_ref(node) {
        candidates.push(normalize_declared_type_ref(type_ref));
    }
    for target in graph.outgoing_typing_or_specializes_targets(node) {
        if let Some(type_ref) = declared_type_ref(target) {
            candidates.push(normalize_declared_type_ref(type_ref));
        }
        candidates.push(target.name.clone());
    }
    for candidate in candidates {
        if candidate.ends_with("::Boolean") || candidate == "Boolean" {
            return Some("Boolean");
        }
        if candidate.ends_with("::Real") || candidate == "Real" {
            return Some("Real");
        }
        if candidate.ends_with("::Integer") || candidate == "Integer" {
            return Some("Integer");
        }
    }
    None
}

fn enum_contains_value(graph: &SemanticGraph, enum_type_ref: &str, literal: &str) -> bool {
    let normalized = normalize_declared_type_ref(enum_type_ref);
    let literal = literal.trim().trim_matches('"').trim_matches('\'');
    graph
        .nodes_named(&normalized)
        .into_iter()
        .filter(|node| node.element_kind == "enum def")
        .flat_map(|node| graph.children_of(node))
        .any(|child| child.name == literal || child.name.ends_with(&format!("::{literal}")))
}

fn expected_unit_dimension_for_type(graph: &SemanticGraph, type_name: &str) -> Option<String> {
    unit_type_for_quantity_type_name(graph, type_name)
        .or_else(|| quantity_value_to_unit_type_name(type_name))
}

fn quantity_type_names_for_attribute(
    graph: &SemanticGraph,
    node: &crate::SemanticNode,
) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(type_ref) = declared_type_ref(node) {
        names.push(normalize_declared_type_ref(type_ref));
    }
    for target in graph.outgoing_typing_or_specializes_targets(node) {
        if let Some(type_ref) = declared_type_ref(target) {
            names.push(normalize_declared_type_ref(type_ref));
        }
        names.push(target.name.clone());
    }
    names
}

fn unit_dimensions_compatible(graph: &SemanticGraph, expected: &str, actual: &str) -> bool {
    is_measurement_unit_compatible(graph, expected, actual)
}

pub(in crate::semantic::diagnostics) fn collect_expression_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
    units: &UnitRegistry,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }

        if node.element_kind == "attribute" {
            let Some(value) = node.attributes.get("value").and_then(|v| v.as_str()) else {
                continue;
            };
            if let Some(scalar_kind) = resolved_scalar_kind(graph, node) {
                if scalar_kind == "Boolean" && !is_boolean_literal(value) && !value.contains("::") {
                    let key = format!("bool-val|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, None),
                            DiagnosticSeverity::Error,
                            "semantic",
                            "attribute_value_type_mismatch",
                            format!(
                                "Attribute '{}' expects Boolean but was assigned '{}'.",
                                node.name,
                                value.trim()
                            ),
                        ));
                    }
                } else if matches!(scalar_kind, "Real" | "Integer") && is_boolean_literal(value) {
                    let key = format!("num-val|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, None),
                            DiagnosticSeverity::Error,
                            "semantic",
                            "attribute_value_type_mismatch",
                            format!(
                                "Attribute '{}' expects numeric type {scalar_kind} but was assigned '{}'.",
                                node.name, value.trim()
                            ),
                        ));
                    }
                }
            }
            if let Some(type_ref) = declared_type_ref(node) {
                let normalized_type = normalize_declared_type_ref(type_ref);
                if resolves_to_enum_def(graph, node, type_ref)
                    && attribute_value_is_string_literal(value)
                {
                    let literal = value.trim().trim_matches('"').trim_matches('\'');
                    if !enum_contains_value(graph, type_ref, literal) {
                        let key = format!("enum|{}", node.id.qualified_name);
                        if seen.insert(key) {
                            diagnostics.push(diag(
                                uri,
                                diagnostic_range(graph, node, None),
                                DiagnosticSeverity::Error,
                                "semantic",
                                "invalid_enumeration_value",
                                format!(
                                    "Attribute '{}' uses enumeration literal '{}' which is not declared on enum {}.",
                                    node.name, literal, normalized_type
                                ),
                            ));
                        }
                    }
                }
            }

            if let Some(unit_start) = value.find('[') {
                if let Some(unit_end) = value[unit_start + 1..].find(']') {
                    let unit_expr = value[unit_start + 1..unit_start + 1 + unit_end].trim();
                    if unit_expr.is_empty() {
                        continue;
                    }
                    if !units.is_recognized_unit_expression(unit_expr) {
                        let key = format!("unknown_unit|{}", node.id.qualified_name);
                        if seen.insert(key) {
                            diagnostics.push(diag(
                                uri,
                                diagnostic_range(graph, node, None),
                                DiagnosticSeverity::Warning,
                                "semantic",
                                "unknown_unit_symbol",
                                format!(
                                    "Attribute '{}' value uses unit '[{}]' which is not in indexed quantity/unit catalogs.",
                                    node.name, unit_expr
                                ),
                            ));
                        }
                        continue;
                    }
                    let actual_dimension = units.unit_expression_dimension(unit_expr);
                    let expected_dimension = quantity_type_names_for_attribute(graph, node)
                        .iter()
                        .find_map(|type_name| expected_unit_dimension_for_type(graph, type_name));
                    if let (Some(expected), Some(actual)) = (expected_dimension, actual_dimension) {
                        if !unit_dimensions_compatible(graph, &expected, &actual) {
                            let key = format!("unit_dim|{}", node.id.qualified_name);
                            if seen.insert(key) {
                                diagnostics.push(diag(
                                    uri,
                                    diagnostic_range(graph, node, None),
                                    DiagnosticSeverity::Warning,
                                    "semantic",
                                    "incompatible_unit_dimension",
                                    format!(
                                        "Attribute '{}' expects quantity unit dimension '{}' but value unit '[{}]' has dimension '{}'.",
                                        node.name, expected, unit_expr, actual
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }

        if matches!(node.element_kind.as_str(), "constraint def" | "assert") {
            if let Some(status) = node
                .attributes
                .get("analysisEvaluationStatus")
                .and_then(|v| v.as_str())
            {
                if status == "analysis_evaluation_unresolved"
                    || node
                        .attributes
                        .get("analysisEvaluationError")
                        .and_then(|v| v.as_str())
                        .is_some_and(|err| err.contains("boolean") || err.contains("Boolean"))
                {
                    let key = format!("nonbool|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, None),
                            DiagnosticSeverity::Warning,
                            "semantic",
                            "non_boolean_expression",
                            format!(
                                "Constraint/assert on '{}' must evaluate to Boolean.",
                                node.name
                            ),
                        ));
                    }
                }
            }
        }

        if node.element_kind == "filter" {
            let owner_kind = node
                .attributes
                .get("filterOwnerKind")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !matches!(owner_kind, "view" | "view def") {
                continue;
            }
            let Some(condition) = node.attributes.get("condition").and_then(|v| v.as_str()) else {
                continue;
            };
            if condition_expression_is_boolean(node, condition) {
                continue;
            }
            let key = format!("view_filter|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "view_filter_non_boolean",
                    format!(
                        "View filter expression '{}' must be Boolean-valued.",
                        condition.trim()
                    ),
                ));
            }
        }

        if node.element_kind == "verify" {
            let Some(lhs) = node.attributes.get("lhs").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(rhs) = node.attributes.get("rhs").and_then(|v| v.as_str()) else {
                continue;
            };
            let lhs = lhs.trim();
            let rhs = rhs.trim();
            if lhs.is_empty() || rhs.is_empty() {
                continue;
            }
            let prefix = node
                .parent_id
                .as_ref()
                .and_then(|id| graph.get_node(id))
                .and_then(|parent| parent.id.qualified_name.rsplit_once("::").map(|(p, _)| p));
            let ResolveResult::Resolved(target_id) =
                resolve_expression_endpoint_strict(graph, uri, prefix, lhs)
            else {
                continue;
            };
            let Some(target) = graph.get_node(&target_id) else {
                continue;
            };
            if let Some(scalar_kind) = resolved_scalar_kind(graph, target) {
                let incompatible = match scalar_kind {
                    "Boolean" => !is_boolean_literal_value(rhs) && !rhs.contains("::"),
                    "Real" | "Integer" => {
                        attribute_value_is_string_literal(rhs) || is_boolean_literal_value(rhs)
                    }
                    _ => false,
                };
                if incompatible {
                    let key = format!("assign_val|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, Some(target)),
                            DiagnosticSeverity::Warning,
                            "semantic",
                            "assignment_value_incompatible",
                            format!(
                                "Assignment to '{}' expects {scalar_kind} but was assigned '{}'.",
                                lhs, rhs
                            ),
                        ));
                    }
                }
            }
        }

        if node.element_kind == "calc def" || node.element_kind == "calc" {
            let param_count = graph
                .children_of(node)
                .into_iter()
                .filter(|child| child.element_kind == "in out parameter")
                .count();
            if let Some(arg_count) = node
                .attributes
                .get("invocationArgCount")
                .and_then(|v| v.as_u64())
            {
                if (arg_count as usize) < param_count {
                    let key = format!("calc|{}", node.id.qualified_name);
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, node, None),
                            DiagnosticSeverity::Warning,
                            "semantic",
                            "calculation_binding_mismatch",
                            format!(
                                "Calculation invocation on '{}' provides {} argument(s) but {} parameter(s) are declared.",
                                node.name, arg_count, param_count
                            ),
                        ));
                    }
                }
            }
        }
    }

    diagnostics
}
