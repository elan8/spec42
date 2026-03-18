use elk_graph::{ElkGraph, NodeId};

/// Return the real top-level nodes (children of the synthetic root).
#[must_use]
pub fn top_level_nodes(graph: &ElkGraph) -> Vec<NodeId> {
    graph.nodes[graph.root.index()].children.clone()
}

/// Walk up the parent chain until a node in `set` is found.
#[must_use]
pub fn nearest_ancestor_in_set(
    graph: &ElkGraph,
    node: NodeId,
    set: &std::collections::BTreeSet<NodeId>,
) -> Option<NodeId> {
    let mut current = Some(node);
    while let Some(candidate) = current {
        if set.contains(&candidate) {
            return Some(candidate);
        }
        current = graph.nodes[candidate.index()].parent;
    }
    None
}

