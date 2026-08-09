use crate::semantic::SemanticGraph;
use sysml_model::ElementKind;
use sysml_model::semantic::import_resolution::{import_target, is_import_all};

pub(super) fn has_import_in_scope(
    graph: &SemanticGraph,
    node: &crate::semantic::SemanticNode,
) -> bool {
    let mut current = Some(node.id.clone());
    while let Some(node_id) = current {
        let Some(scope_node) = graph.get_node(&node_id) else {
            break;
        };
        if graph
            .children_of(scope_node)
            .into_iter()
            .any(|child| child.element_kind == ElementKind::Import)
        {
            return true;
        }
        current = scope_node.parent_id.clone();
    }
    false
}

fn is_namespace_kind(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::Package
            | ElementKind::RequirementDef
            | ElementKind::Requirement
            | ElementKind::UseCaseDef
            | ElementKind::UseCase
            | ElementKind::AnalysisDef
            | ElementKind::Analysis
            | ElementKind::VerificationDef
            | ElementKind::Verification
            | ElementKind::ConcernDef
            | ElementKind::Concern
    )
}


fn normalized_namespace_target(target: &str) -> String {
    target
        .trim()
        .trim_end_matches("::**")
        .trim_end_matches("::*")
        .trim()
        .to_string()
}

fn normalized_membership_target(target: &str) -> String {
    target.trim().trim_end_matches("::**").trim().to_string()
}

fn has_node_with_qualified_name_or_disambiguated_variant(
    graph: &SemanticGraph,
    base: &str,
) -> bool {
    if graph
        .nodes_by_uri
        .values()
        .flatten()
        .any(|id| id.qualified_name == base)
    {
        return true;
    }
    let disambiguated_prefix = format!("{base}#");
    graph
        .nodes_by_uri
        .values()
        .flatten()
        .any(|id| id.qualified_name.starts_with(&disambiguated_prefix))
}

pub(super) fn import_target_resolves(
    graph: &SemanticGraph,
    import_node: &crate::semantic::SemanticNode,
) -> bool {
    let Some(target) = import_target(import_node) else {
        return false;
    };

    if is_import_all(import_node) {
        let namespace_target = normalized_namespace_target(target);
        return graph
            .nodes_by_uri
            .values()
            .flatten()
            .filter(|id| id.qualified_name == namespace_target)
            .filter_map(|id| graph.get_node(id))
            .any(|node| is_namespace_kind(&node.element_kind));
    }

    let membership_target = normalized_membership_target(target);
    if has_node_with_qualified_name_or_disambiguated_variant(graph, &membership_target) {
        return true;
    }

    if let Some((namespace_target, member_name)) = membership_target.rsplit_once("::") {
        return graph
            .nodes_by_uri
            .values()
            .flatten()
            .filter(|id| id.qualified_name == namespace_target)
            .filter_map(|id| graph.get_node(id))
            .filter(|node| is_namespace_kind(&node.element_kind))
            .flat_map(|namespace| graph.children_of(namespace))
            .any(|child| child.element_kind != ElementKind::Import && child.name == member_name);
    }

    false
}
