use super::*;

pub(crate) fn resolve_pending_target(
    graph: &SemanticGraph,
    source_node: &SemanticNode,
    target_qualified: &str,
    allowed_kinds: &[ElementKind],
) -> Vec<NodeId> {
    let mut resolved =
        resolve_type_reference_targets(graph, source_node, target_qualified, allowed_kinds);
    if resolved.is_empty() {
        if let Some(simple_name) = target_qualified.rsplit("::").next() {
            if simple_name != target_qualified {
                resolved =
                    resolve_type_reference_targets(graph, source_node, simple_name, allowed_kinds);
            }
        }
    }
    resolved
}

pub fn resolve_type_target_in_workspace(
    g: &SemanticGraph,
    context_node: &SemanticNode,
    type_ref: &str,
    allowed_target_kinds: &[ElementKind],
) -> Option<NodeId> {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return None;
    }
    resolve_type_reference_targets(g, context_node, &normalized_type_ref, allowed_target_kinds)
        .into_iter()
        .next()
}

/// Subject edge from a case/requirement to the resolved type of its `subject name : Type` declaration.
pub fn add_subject_relationship_to_declared_type_if_resolved(
    g: &mut SemanticGraph,
    case_node_id: &NodeId,
    type_ref: &str,
) {
    let Some(case_node) = g.get_node(case_node_id).cloned() else {
        return;
    };
    if let Some(subject_usage) = g
        .children_of(&case_node)
        .into_iter()
        .find(|child| child.element_kind == ElementKind::Subject)
    {
        if let Some(target_id) = g
            .outgoing_targets_by_kind(subject_usage, RelationshipKind::Typing)
            .into_iter()
            .find(|target| element_kind_allowed(&target.element_kind, SUBJECT_TYPE_TARGET_KINDS))
            .map(|target| target.id.clone())
        {
            add_semantic_edge_once(
                g,
                case_node_id,
                &target_id,
                SemanticEdge::plain(RelationshipKind::Subject),
            );
            return;
        }
    }
    let resolution_context = g
        .children_of(&case_node)
        .into_iter()
        .find(|child| child.element_kind == ElementKind::Subject)
        .cloned()
        .unwrap_or(case_node);
    let Some(target_id) = resolve_type_target_in_workspace(
        g,
        &resolution_context,
        type_ref,
        SUBJECT_TYPE_TARGET_KINDS,
    ) else {
        return;
    };
    add_semantic_edge_once(
        g,
        case_node_id,
        &target_id,
        SemanticEdge::plain(RelationshipKind::Subject),
    );
}

pub(crate) fn link_case_subject_relationships(g: &mut SemanticGraph) {
    const CASE_KINDS: &[ElementKind] = &[
        ElementKind::AnalysisDef,
        ElementKind::Analysis,
        ElementKind::VerificationDef,
        ElementKind::Verification,
        ElementKind::UseCaseDef,
        ElementKind::UseCase,
        ElementKind::ConcernDef,
        ElementKind::Concern,
        ElementKind::RequirementDef,
        ElementKind::Requirement,
    ];
    let node_ids: Vec<NodeId> = g.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(case_node) = g.get_node(&node_id).cloned() else {
            continue;
        };
        if !CASE_KINDS.contains(&case_node.element_kind) {
            continue;
        }
        let subject_type_refs: Vec<String> = g
            .children_of(&case_node)
            .into_iter()
            .filter(|child| child.element_kind == ElementKind::Subject)
            .filter_map(|child| {
                child
                    .attributes
                    .get("subjectType")
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
            })
            .collect();
        for type_ref in subject_type_refs {
            add_subject_relationship_to_declared_type_if_resolved(g, &node_id, &type_ref);
        }
    }
}
