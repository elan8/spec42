//! Post-link propagation of typed analysis / verification case context onto usages.

use std::collections::HashSet;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId, RelationshipKind, SemanticNode};
use crate::semantic::relationships::{resolve_type_target_in_workspace, SPECIALIZES_TARGET_KINDS};

const ANALYSIS_EXPRESSION_KEY: &str = "analysisExpression";
const ANALYSIS_CONSTRAINTS_KEY: &str = "analysisConstraints";
const CASE_DEF_KINDS: &[ElementKind] = &[ElementKind::AnalysisDef, ElementKind::VerificationDef];

/// Prepares analysis evaluation metadata after workspace linking.
pub fn prepare_analysis_evaluation_context(graph: &mut SemanticGraph) {
    propagate_typed_case_context(graph);
    propagate_typed_requirement_context(graph);
    aggregate_assert_constraints(graph);
}

/// After workspace relationship linking, copy inherited analysis expressions from typed
/// case definitions onto usages that do not declare a local `return ref`.
pub fn propagate_typed_case_context(graph: &mut SemanticGraph) {
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = graph.get_node(&node_id).cloned() else {
            continue;
        };
        if !matches!(
            node.element_kind,
            ElementKind::Analysis | ElementKind::Verification
        ) {
            continue;
        }
        propagate_case_usage_from_typing(graph, &node_id, &node);
    }
}

/// Copies `analysisConstraints` from a typed `requirement def` when the usage has none.
pub fn propagate_typed_requirement_context(graph: &mut SemanticGraph) {
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = graph.get_node(&node_id).cloned() else {
            continue;
        };
        if node.element_kind != ElementKind::Requirement {
            continue;
        }
        if usage_has_analysis_constraints(&node) {
            continue;
        }
        let Some(def_id) = typed_requirement_definition_id(graph, &node) else {
            continue;
        };
        let Some(def_node) = graph.get_node(&def_id).cloned() else {
            continue;
        };
        let Some(constraints) = def_node.attributes.get(ANALYSIS_CONSTRAINTS_KEY).cloned() else {
            continue;
        };
        if let Some(usage_mut) = graph.get_node_mut(&node_id) {
            usage_mut
                .attributes
                .insert(ANALYSIS_CONSTRAINTS_KEY.to_string(), constraints);
        }
        if let Some(expression) = def_node
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|expr| !expr.is_empty())
        {
            if let Some(usage_mut) = graph.get_node_mut(&node_id) {
                usage_mut.attributes.insert(
                    ANALYSIS_EXPRESSION_KEY.to_string(),
                    serde_json::json!(expression),
                );
            }
        }
    }
}

fn usage_has_analysis_constraints(node: &SemanticNode) -> bool {
    node.attributes
        .get(ANALYSIS_CONSTRAINTS_KEY)
        .and_then(|value| value.as_array())
        .is_some_and(|items| !items.is_empty())
}

pub(crate) fn typed_requirement_definition_id(
    graph: &SemanticGraph,
    usage: &SemanticNode,
) -> Option<NodeId> {
    if usage.element_kind != ElementKind::Requirement {
        return None;
    }
    graph
        .outgoing_targets_by_kind(usage, RelationshipKind::Typing)
        .into_iter()
        .find(|target| target.element_kind == ElementKind::RequirementDef)
        .map(|target| target.id.clone())
}

pub(crate) fn typed_requirement_definition_scope_prefixes(
    graph: &SemanticGraph,
    usage: &SemanticNode,
) -> Vec<String> {
    let Some(mut current_id) = typed_requirement_definition_id(graph, usage) else {
        return Vec::new();
    };
    let mut prefixes = Vec::new();
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(current_id.clone()) {
            break;
        }
        let Some(current) = graph.get_node(&current_id) else {
            break;
        };
        prefixes.push(current.id.qualified_name.clone());
        let Some(specializes_ref) = current
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            break;
        };
        let Some(parent_id) = resolve_type_target_in_workspace(
            graph,
            current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        ) else {
            break;
        };
        if graph
            .get_node(&parent_id)
            .map(|node| node.element_kind.clone())
            != Some(ElementKind::RequirementDef)
        {
            break;
        }
        current_id = parent_id;
    }
    prefixes
}

fn propagate_case_usage_from_typing(
    graph: &mut SemanticGraph,
    usage_id: &NodeId,
    usage: &SemanticNode,
) {
    if usage_has_local_analysis_expression(usage) {
        return;
    }
    let Some(def_id) = typed_case_definition_id(graph, usage) else {
        return;
    };
    let Some(expression) = resolve_case_definition_expression(graph, &def_id) else {
        return;
    };
    if let Some(usage_mut) = graph.get_node_mut(usage_id) {
        usage_mut.attributes.insert(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            serde_json::json!(expression),
        );
    }
}

fn usage_has_local_analysis_expression(usage: &SemanticNode) -> bool {
    if usage
        .attributes
        .get(ANALYSIS_EXPRESSION_KEY)
        .and_then(|value| value.as_str())
        .is_some_and(|expr| !expr.trim().is_empty())
    {
        return true;
    }
    usage
        .attributes
        .get("analysisResultCount")
        .and_then(|value| value.as_u64())
        .is_some_and(|count| count > 0)
}

/// Qualified names of the typed case definition and each `:>` ancestor (nearest first).
pub(crate) fn typed_case_definition_scope_prefixes(
    graph: &SemanticGraph,
    usage: &SemanticNode,
) -> Vec<String> {
    let Some(mut current_id) = typed_case_definition_id(graph, usage) else {
        return Vec::new();
    };
    let mut prefixes = Vec::new();
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(current_id.clone()) {
            break;
        }
        let Some(current) = graph.get_node(&current_id) else {
            break;
        };
        prefixes.push(current.id.qualified_name.clone());
        let Some(specializes_ref) = current
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            break;
        };
        let Some(parent_id) = resolve_type_target_in_workspace(
            graph,
            current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        ) else {
            break;
        };
        if !graph
            .get_node(&parent_id)
            .map(|node| CASE_DEF_KINDS.contains(&node.element_kind))
            .unwrap_or(false)
        {
            break;
        }
        current_id = parent_id;
    }
    prefixes
}

pub(crate) fn typed_case_definition_id(
    graph: &SemanticGraph,
    usage: &SemanticNode,
) -> Option<NodeId> {
    if !matches!(
        usage.element_kind,
        ElementKind::Analysis | ElementKind::Verification
    ) {
        return None;
    }
    let expected_def_kind = match usage.element_kind {
        ElementKind::Analysis => ElementKind::AnalysisDef,
        ElementKind::Verification => ElementKind::VerificationDef,
        _ => return None,
    };
    graph
        .outgoing_targets_by_kind(usage, RelationshipKind::Typing)
        .into_iter()
        .find(|target| target.element_kind == expected_def_kind)
        .map(|target| target.id.clone())
}

pub(crate) fn resolve_case_definition_expression(
    graph: &SemanticGraph,
    def_id: &NodeId,
) -> Option<String> {
    if let Some(expression) = graph
        .get_node(def_id)?
        .attributes
        .get(ANALYSIS_EXPRESSION_KEY)
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|expr| !expr.is_empty())
    {
        return Some(expression.to_string());
    }
    let inherited_result = inherited_case_result_qualified(graph, def_id);
    inherited_case_expression(graph, def_id, inherited_result.as_deref())
}

pub(crate) fn inherited_case_result_qualified(
    graph: &SemanticGraph,
    case_def_id: &NodeId,
) -> Option<String> {
    let mut current_id = case_def_id.clone();
    let mut seen = HashSet::new();
    loop {
        let specializes_ref = graph
            .get_node(&current_id)?
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())?;
        if !seen.insert(current_id.clone()) {
            return None;
        }
        let current = graph.get_node(&current_id)?.clone();
        let target_id = resolve_type_target_in_workspace(
            graph,
            &current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        )?;
        let target = graph.get_node(&target_id)?;
        if !CASE_DEF_KINDS.contains(&target.element_kind) {
            return None;
        }
        for child in graph.children_of(target) {
            if child.element_kind == ElementKind::AnalysisResult {
                return Some(child.id.qualified_name.clone());
            }
        }
        current_id = target_id;
    }
}

pub(crate) fn inherited_case_expression(
    graph: &SemanticGraph,
    case_def_id: &NodeId,
    inherited_result_qualified: Option<&str>,
) -> Option<String> {
    if let Some(result_qualified) = inherited_result_qualified {
        let result_id = NodeId::new(&case_def_id.uri, result_qualified);
        if let Some(result_node) = graph.get_node(&result_id) {
            if let Some(body) = result_node
                .attributes
                .get("returnBody")
                .and_then(|value| value.as_str())
            {
                let expression = strip_analysis_return_body(body);
                if !expression.is_empty() {
                    return Some(expression);
                }
            }
        }
    }
    let mut current_id = case_def_id.clone();
    let mut seen = HashSet::new();
    loop {
        let specializes_ref = graph
            .get_node(&current_id)?
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())?;
        if !seen.insert(current_id.clone()) {
            return None;
        }
        let current = graph.get_node(&current_id)?.clone();
        let target_id = resolve_type_target_in_workspace(
            graph,
            &current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        )?;
        let target = graph.get_node(&target_id)?;
        if !CASE_DEF_KINDS.contains(&target.element_kind) {
            return None;
        }
        if let Some(expression) = target
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(expression.to_string());
        }
        current_id = target_id;
    }
}

/// Projects `assert constraint` children onto the parent `analysisConstraints` array.
pub fn aggregate_assert_constraints(graph: &mut SemanticGraph) {
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(owner) = graph.get_node(&node_id).cloned() else {
            continue;
        };
        let assert_constraints: Vec<serde_json::Value> = graph
            .children_of(&owner)
            .iter()
            .filter(|child| child.element_kind == ElementKind::AssertConstraint)
            .filter_map(|child| {
                let expression = child
                    .attributes
                    .get("expression")
                    .and_then(|value| value.as_str())
                    .map(str::trim)
                    .filter(|expr| !expr.is_empty())?;
                Some(serde_json::json!({
                    "kind": "assert_constraint",
                    "expression": expression,
                }))
            })
            .collect();
        if assert_constraints.is_empty() {
            continue;
        }
        let Some(owner_mut) = graph.get_node_mut(&node_id) else {
            continue;
        };
        let mut merged: Vec<serde_json::Value> = owner_mut
            .attributes
            .get(ANALYSIS_CONSTRAINTS_KEY)
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|entry| {
                entry.get("kind").and_then(|value| value.as_str()) != Some("assert_constraint")
            })
            .collect();
        merged.extend(assert_constraints);
        owner_mut.attributes.insert(
            ANALYSIS_CONSTRAINTS_KEY.to_string(),
            serde_json::Value::Array(merged),
        );
    }
}

pub(crate) fn strip_analysis_return_body(body: &str) -> String {
    let mut trimmed = body.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        trimmed = trimmed[1..trimmed.len() - 1].trim();
    }
    let without_return = trimmed
        .strip_prefix("return")
        .map(str::trim)
        .unwrap_or(trimmed);
    without_return.trim_end_matches(';').trim().to_string()
}
