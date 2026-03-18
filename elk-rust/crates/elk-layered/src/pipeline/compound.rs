//! Compound graph preprocessing and postprocessing for cross-hierarchy edges.
//!
//! Cross-hierarchy edges connect ports in different subtrees. We temporarily replace
//! their endpoints with hierarchical ports on boundary nodes so routing produces
//! boundary-to-boundary paths, then snap the path start/end back to the real ports.

use std::collections::BTreeMap;

use elk_core::{
    LayoutDirection, LayoutOptions, PortSide,
};
use elk_graph::{EdgeEndpoint, EdgeId, ElkGraph, NodeId, ShapeGeometry};

use crate::pipeline::util::endpoint_abs_center;

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
    graph: &mut ElkGraph,
    local_nodes: &std::collections::BTreeSet<NodeId>,
    options: &LayoutOptions,
) -> CompoundRoutingMap {
    let direction = options.layered.direction;
    let mut map = CompoundRoutingMap::default();
    let port_geom = ShapeGeometry {
        x: 0.0,
        y: 0.0,
        width: 8.0,
        height: 8.0,
    };

    // Collect cross-hierarchy edges to avoid borrow conflicts
    let mut to_process: Vec<(EdgeId, NodeId, NodeId, EdgeEndpoint, EdgeEndpoint)> = Vec::new();
    for edge in &graph.edges {
        let Some(original_source) = edge.sources.first().copied() else {
            continue;
        };
        let Some(original_target) = edge.targets.first().copied() else {
            continue;
        };
        let Some(effective_source) = nearest_ancestor_in_set(graph, original_source.node, local_nodes) else {
            continue;
        };
        let Some(effective_target) = nearest_ancestor_in_set(graph, original_target.node, local_nodes) else {
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
            && (original_source.node != effective_source || original_target.node != effective_target);

        if is_cross_hierarchy {
            to_process.push((
                edge.id,
                effective_source,
                effective_target,
                original_source,
                original_target,
            ));
        }
    }

    for (edge_id, effective_source, effective_target, original_source, original_target) in
        to_process
    {
        let hp_source_side = hierarchical_port_side(direction, true);
        let hp_target_side = hierarchical_port_side(direction, false);

        let hp_source = graph.add_port(effective_source, hp_source_side, port_geom);
        let hp_target = graph.add_port(effective_target, hp_target_side, port_geom);

        let edge = &mut graph.edges[edge_id.index()];
        if let Some(first) = edge.sources.first_mut() {
            *first = EdgeEndpoint::port(effective_source, hp_source);
        }
        if let Some(first) = edge.targets.first_mut() {
            *first = EdgeEndpoint::port(effective_target, hp_target);
        }

        map.originals.insert(edge_id, (original_source, original_target));
    }

    map
}

/// Postprocess: snap section start/end back to original port positions.
pub fn postprocess_cross_hierarchy_edges(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
) {
    for (edge_id, (original_source, original_target)) in &map.originals {
        // Compute desired endpoints before mutably borrowing the edge.
        let start_center = endpoint_abs_center(graph, *original_source);
        let end_center = endpoint_abs_center(graph, *original_target);

        let edge = &mut graph.edges[edge_id.index()];
        // Restore original endpoints so downstream consumers don't see synthetic hierarchical ports.
        if let Some(first) = edge.sources.first_mut() {
            *first = *original_source;
        }
        if let Some(first) = edge.targets.first_mut() {
            *first = *original_target;
        }
        if edge.sections.is_empty() {
            continue;
        }
        let sec_id = edge.sections[0];
        let section = &mut graph.edge_sections[sec_id.index()];
        section.start = start_center;
        section.end = end_center;
    }
}

fn nearest_ancestor_in_set(
    graph: &ElkGraph,
    node: NodeId,
    local_nodes: &std::collections::BTreeSet<NodeId>,
) -> Option<NodeId> {
    let mut current = Some(node);
    while let Some(candidate) = current {
        if local_nodes.contains(&candidate) {
            return Some(candidate);
        }
        current = graph.nodes[candidate.index()].parent;
    }
    None
}
