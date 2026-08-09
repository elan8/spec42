use crate::semantic::SemanticGraph;
use sysml_model::ImportTargetResolution;

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

pub(super) fn resolve_import_target(
    graph: &SemanticGraph,
    import_node: &crate::semantic::SemanticNode,
) -> ImportTargetResolution {
    sysml_model::resolve_import_target(graph, import_node)
}
