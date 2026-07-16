use super::*;

pub(crate) fn try_wire_derivation_connection(
    g: &mut SemanticGraph,
    uri: &Url,
    connection_node_id: &NodeId,
) {
    let Some(connection) = g.get_node(connection_node_id) else {
        return;
    };
    if connection
        .attributes
        .get("connectionAnnotation")
        .and_then(|value| value.as_str())
        != Some("derivation")
    {
        return;
    }
    let scope_prefix = connection
        .parent_id
        .as_ref()
        .and_then(|parent_id| g.get_node(parent_id))
        .map(|parent| parent.id.qualified_name.as_str());

    let Some(original_id) =
        resolve_derivation_end_target(g, uri, scope_prefix, connection_node_id, "#original")
    else {
        return;
    };
    let Some(derived_id) =
        resolve_derivation_end_target(g, uri, scope_prefix, connection_node_id, "#derive")
    else {
        return;
    };

    add_semantic_edge_once(
        g,
        &original_id,
        &derived_id,
        SemanticEdge::plain(RelationshipKind::Derivation),
    );
    if let Some(connection) = g.get_node_mut(connection_node_id) {
        connection.attributes.insert(
            "derivationOriginal".to_string(),
            serde_json::json!(normalize_for_lookup(&original_id.qualified_name)),
        );
        connection.attributes.insert(
            "derivationDerived".to_string(),
            serde_json::json!(normalize_for_lookup(&derived_id.qualified_name)),
        );
    }
}

fn resolve_derivation_end_target(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    connection_node_id: &NodeId,
    end_name: &str,
) -> Option<NodeId> {
    let end = g
        .child_named(connection_node_id, end_name)
        .into_iter()
        .next()?;
    if let Some(target) = g
        .outgoing_targets_by_kind(end, RelationshipKind::Typing)
        .into_iter()
        .next()
    {
        return Some(target.id.clone());
    }
    let type_ref = end.attributes.get("endType")?.as_str()?;
    match resolve_expression_endpoint_strict(g, uri, container_prefix, type_ref) {
        ResolveResult::Resolved(id) => Some(id),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => {
            resolve_type_target_in_workspace(g, end, type_ref, TYPING_TARGET_KINDS)
        }
    }
}

/// Result of attempting to add a semantic edge between two nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddSemanticEdgeResult {
    Added,
    SkippedSameKind,
    DuplicateConnect,
}
