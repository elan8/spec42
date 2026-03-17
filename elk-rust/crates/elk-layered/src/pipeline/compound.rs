//! Compound graph preprocessing and postprocessing for cross-hierarchy edges.
//!
//! Cross-hierarchy edges connect ports in different subtrees. We temporarily replace
//! their endpoints with hierarchical ports on boundary nodes so routing produces
//! boundary-to-boundary paths, then snap the path start/end back to the real ports.

use std::collections::BTreeMap;

use elk_core::{
    EdgeEndpoint, EdgeId, Graph, LayoutDirection, LayoutOptions, NodeId, PortSide, Size,
};

/// Map from edge id to original (source, target) endpoints for compound edges.
/// Used to restore port-anchored start/end after routing.
#[derive(Clone, Debug, Default)]
pub struct CompoundRoutingMap {
    /// Original source and target for each compound edge.
    pub originals: BTreeMap<EdgeId, (EdgeEndpoint, EdgeEndpoint)>,
}

/// Infer hierarchical port side from layout direction.
/// Source boundary gets the "outgoing" side, target gets the "incoming" side.
fn hierarchical_port_side(direction: LayoutDirection, is_source: bool) -> PortSide {
    match direction {
        LayoutDirection::LeftToRight => {
            if is_source {
                PortSide::East
            } else {
                PortSide::West
            }
        }
        LayoutDirection::RightToLeft => {
            if is_source {
                PortSide::West
            } else {
                PortSide::East
            }
        }
        LayoutDirection::TopToBottom => {
            if is_source {
                PortSide::South
            } else {
                PortSide::North
            }
        }
        LayoutDirection::BottomToTop => {
            if is_source {
                PortSide::North
            } else {
                PortSide::South
            }
        }
    }
}

/// Preprocess cross-hierarchy edges: replace endpoints with hierarchical ports on boundaries.
/// Returns a map of original endpoints for postprocessing.
pub fn preprocess_cross_hierarchy_edges(
    graph: &mut Graph,
    local_nodes: &std::collections::BTreeSet<NodeId>,
    options: &LayoutOptions,
) -> CompoundRoutingMap {
    let direction = options.layered.direction;
    let mut map = CompoundRoutingMap::default();
    let port_size = Size::new(8.0, 8.0);

    // Collect cross-hierarchy edges to avoid borrow conflicts
    let mut to_process: Vec<(EdgeId, NodeId, NodeId, EdgeEndpoint, EdgeEndpoint)> = Vec::new();
    for edge in &graph.edges {
        let Some(effective_source) =
            graph.nearest_ancestor_in_set(edge.source.node, local_nodes)
        else {
            continue;
        };
        let Some(effective_target) =
            graph.nearest_ancestor_in_set(edge.target.node, local_nodes)
        else {
            continue;
        };

        // Only treat as cross-hierarchy when the endpoints belong to *different* local ancestors
        // at the current layout level.
        //
        // If both endpoints share the same effective ancestor, that edge should be routed inside
        // that ancestor's child subgraph during recursive layout (IncludeChildren). Lifting it to
        // this level would collapse it into a self-loop and prevent the child layout from ever
        // seeing the edge.
        let is_cross_hierarchy = effective_source != effective_target
            && (edge.source.node != effective_source || edge.target.node != effective_target);

        if is_cross_hierarchy {
            to_process.push((
                edge.id,
                effective_source,
                effective_target,
                edge.source,
                edge.target,
            ));
        }
    }

    for (edge_id, effective_source, effective_target, original_source, original_target) in
        to_process
    {
        let hp_source_side = hierarchical_port_side(direction, true);
        let hp_target_side = hierarchical_port_side(direction, false);

        let hp_source = graph.add_port(effective_source, hp_source_side, port_size);
        let hp_target = graph.add_port(effective_target, hp_target_side, port_size);

        graph.port_mut(hp_source).is_hierarchical = true;
        graph.port_mut(hp_target).is_hierarchical = true;

        let edge = graph.edge_mut(edge_id);
        edge.source = EdgeEndpoint::port(effective_source, hp_source);
        edge.target = EdgeEndpoint::port(effective_target, hp_target);

        map.originals.insert(edge_id, (original_source, original_target));
    }

    map
}

/// Postprocess: snap section start/end back to original port positions.
pub fn postprocess_cross_hierarchy_edges(
    graph: &mut Graph,
    map: &CompoundRoutingMap,
) {
    for (edge_id, (original_source, original_target)) in &map.originals {
        // Compute desired endpoints before mutably borrowing the edge.
        let start_center = if let Some(p) = original_source.port {
            graph.port(p).bounds.center()
        } else {
            graph.node(original_source.node).bounds.center()
        };
        let end_center = if let Some(p) = original_target.port {
            graph.port(p).bounds.center()
        } else {
            graph.node(original_target.node).bounds.center()
        };

        let edge = graph.edge_mut(*edge_id);
        // Restore original endpoints so downstream consumers don't see synthetic hierarchical ports.
        edge.source = *original_source;
        edge.target = *original_target;
        if edge.sections.is_empty() {
            continue;
        }
        let section = &mut edge.sections[0];
        section.start = start_center;
        section.end = end_center;
    }
}
