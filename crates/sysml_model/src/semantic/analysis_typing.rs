//! Post-link propagation of typed analysis / verification case context onto usages.

use std::collections::HashSet;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    DeclaredAnalysisCaseFacts, DeclaredAnalysisConstraint, ElementKind, NodeId, RelationshipKind,
    SemanticNode,
};
use crate::semantic::relationships::{resolve_type_target_in_workspace, SPECIALIZES_TARGET_KINDS};

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
        let Some(def_facts) = def_node.declared_facts.analysis_case.as_ref() else {
            continue;
        };
        if def_facts.constraints.is_empty() {
            continue;
        }
        let constraints = def_facts.constraints.clone();
        let expression = def_facts
            .expression
            .as_deref()
            .map(str::trim)
            .filter(|expr| !expr.is_empty())
            .map(str::to_string);
        if let Some(usage_mut) = graph.get_node_mut(&node_id) {
            let facts = usage_mut
                .declared_facts
                .analysis_case
                .get_or_insert_with(DeclaredAnalysisCaseFacts::default);
            facts.constraints = constraints;
            if let Some(expression) = expression {
                facts.expression = Some(expression);
            }
        }
    }
}

fn usage_has_analysis_constraints(node: &SemanticNode) -> bool {
    node.declared_facts
        .analysis_case
        .as_ref()
        .is_some_and(|facts| !facts.constraints.is_empty())
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
            .declared_facts
            .relationships
            .specializes
            .first()
            .map(|target| target.reference.as_str())
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
    let result = effective_case_result(graph, &def_id).cloned();
    if let Some(usage_mut) = graph.get_node_mut(usage_id) {
        usage_mut
            .declared_facts
            .analysis_case
            .get_or_insert_with(DeclaredAnalysisCaseFacts::default)
            .expression = Some(expression);
        if let Some(result) = result.as_ref() {
            usage_mut
                .attributes
                .insert("analysisResultCount".to_string(), serde_json::json!(1));
            usage_mut.attributes.insert(
                "inheritedAnalysisResult".to_string(),
                serde_json::json!(result.id.qualified_name.as_str()),
            );
            if let Some(mode) = result
                .attributes
                .get("analysisResultMode")
                .and_then(|value| value.as_str())
            {
                usage_mut
                    .attributes
                    .insert("analysisResultMode".to_string(), serde_json::json!(mode));
            }
            if let Some(type_name) = result
                .attributes
                .get("returnType")
                .and_then(|value| value.as_str())
            {
                usage_mut.attributes.insert(
                    "analysisResultType".to_string(),
                    serde_json::json!(type_name),
                );
            }
        }
    }
    if let Some(result) = result.as_ref() {
        let objective_ids = graph
            .get_node(usage_id)
            .map(|usage| {
                graph
                    .children_of(usage)
                    .into_iter()
                    .filter(|child| child.element_kind == ElementKind::Objective)
                    .map(|child| child.id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        for objective_id in objective_ids {
            if let Some(objective) = graph.get_node_mut(&objective_id) {
                objective
                    .declared_facts
                    .analysis_case
                    .get_or_insert_with(DeclaredAnalysisCaseFacts::default)
                    .objective_bound_to = Some(result.id.qualified_name.clone());
            }
        }
    }
}

fn effective_case_result<'a>(
    graph: &'a SemanticGraph,
    case_def_id: &NodeId,
) -> Option<&'a SemanticNode> {
    let case_def = graph.get_node(case_def_id)?;
    if let Some(result) = graph
        .children_of(case_def)
        .into_iter()
        .find(|child| child.element_kind == ElementKind::AnalysisResult)
    {
        return Some(result);
    }
    let inherited = inherited_case_result_qualified(graph, case_def_id)?;
    graph
        .node_ids_by_qualified_name
        .get(&inherited)?
        .iter()
        .find_map(|id| graph.get_node(id))
}

fn usage_has_local_analysis_expression(usage: &SemanticNode) -> bool {
    if usage
        .declared_facts
        .analysis_case
        .as_ref()
        .and_then(|facts| facts.expression.as_deref())
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
            .declared_facts
            .relationships
            .specializes
            .first()
            .map(|target| target.reference.as_str())
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
        .declared_facts
        .analysis_case
        .as_ref()
        .and_then(|facts| facts.expression.as_deref())
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
            .declared_facts
            .relationships
            .specializes
            .first()
            .map(|target| target.reference.as_str())
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
            if let Some(expression) = result_node
                .expression_text
                .value
                .as_deref()
                .map(str::trim)
                .filter(|expression| !expression.is_empty())
            {
                return Some(expression.to_string());
            }
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
            .declared_facts
            .relationships
            .specializes
            .first()
            .map(|target| target.reference.as_str())
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
            .declared_facts
            .analysis_case
            .as_ref()
            .and_then(|facts| facts.expression.as_deref())
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
        let assert_constraints: Vec<DeclaredAnalysisConstraint> = graph
            .children_of(&owner)
            .iter()
            .filter(|child| child.element_kind == ElementKind::AssertConstraint)
            .filter_map(|child| {
                // `expression` here is the `AssertConstraint` child's own key, owned by a
                // different B9 chunk; read exactly as before and fold into this node's typed
                // `analysisConstraints` fact.
                let expression = child
                    .attributes
                    .get("expression")
                    .and_then(|value| value.as_str())
                    .map(str::trim)
                    .filter(|expr| !expr.is_empty())?;
                Some(DeclaredAnalysisConstraint::AssertConstraint {
                    expression: expression.to_string(),
                })
            })
            .collect();
        if assert_constraints.is_empty() {
            continue;
        }
        let Some(owner_mut) = graph.get_node_mut(&node_id) else {
            continue;
        };
        let facts = owner_mut
            .declared_facts
            .analysis_case
            .get_or_insert_with(DeclaredAnalysisCaseFacts::default);
        facts
            .constraints
            .retain(|entry| !matches!(entry, DeclaredAnalysisConstraint::AssertConstraint { .. }));
        facts.constraints.extend(assert_constraints);
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
