//! Compound graph preprocessing and postprocessing for cross-hierarchy edges.
//!
//! Cross-hierarchy edges connect ports in different subtrees. We temporarily replace
//! their endpoints with hierarchical ports on boundary nodes so routing produces
//! boundary-to-boundary paths, then snap the path start/end back to the real ports.

use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutOptions, PortSide};
use elk_graph::{EdgeEndpoint, EdgeId, ElkGraph, NodeId, ShapeGeometry};

use crate::pipeline::util::endpoint_abs_center;

/// Map from edge id to original (source, target) endpoints for compound edges.
/// Used to restore port-anchored start/end after routing.
#[derive(Clone, Debug, Default)]
pub struct CompoundRoutingMap {
    /// Original source and target for each compound edge.
    pub originals: BTreeMap<EdgeId, (EdgeEndpoint, EdgeEndpoint)>,
}

impl CompoundRoutingMap {
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.originals.len()
    }
}

fn hierarchical_port_side_for_edge(
    _direction: LayoutDirection,
    source_center: elk_core::Point,
    target_center: elk_core::Point,
    is_source: bool,
) -> PortSide {
    let dx = target_center.x - source_center.x;
    let dy = target_center.y - source_center.y;
    if dx.abs() >= dy.abs() {
        if is_source {
            if dx >= 0.0 { PortSide::East } else { PortSide::West }
        } else if dx >= 0.0 {
            PortSide::West
        } else {
            PortSide::East
        }
    } else if is_source {
        if dy >= 0.0 { PortSide::South } else { PortSide::North }
    } else if dy >= 0.0 {
        PortSide::North
    } else {
        PortSide::South
    }
}

fn place_hierarchical_port_on_boundary(
    graph: &mut ElkGraph,
    node_id: NodeId,
    port_id: elk_graph::PortId,
    side: PortSide,
) {
    let n = graph.nodes[node_id.index()].geometry;
    let p = &mut graph.ports[port_id.index()].geometry;
    match side {
        PortSide::North => {
            p.x = (n.width - p.width).max(0.0) / 2.0;
            p.y = -p.height / 2.0;
        }
        PortSide::South => {
            p.x = (n.width - p.width).max(0.0) / 2.0;
            p.y = n.height - p.height / 2.0;
        }
        PortSide::East => {
            p.x = n.width - p.width / 2.0;
            p.y = (n.height - p.height).max(0.0) / 2.0;
        }
        PortSide::West => {
            p.x = -p.width / 2.0;
            p.y = (n.height - p.height).max(0.0) / 2.0;
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
        let Some(effective_source) =
            elk_alg_common::graph::nearest_ancestor_in_set(graph, original_source.node, local_nodes)
        else {
            continue;
        };
        let Some(effective_target) =
            elk_alg_common::graph::nearest_ancestor_in_set(graph, original_target.node, local_nodes)
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
        let source_center = endpoint_abs_center(graph, original_source);
        let target_center = endpoint_abs_center(graph, original_target);
        let hp_source_side = hierarchical_port_side_for_edge(direction, source_center, target_center, true);
        let hp_target_side = hierarchical_port_side_for_edge(direction, source_center, target_center, false);

        let hp_source = graph.add_port(effective_source, hp_source_side, port_geom);
        let hp_target = graph.add_port(effective_target, hp_target_side, port_geom);
        place_hierarchical_port_on_boundary(graph, effective_source, hp_source, hp_source_side);
        place_hierarchical_port_on_boundary(graph, effective_target, hp_target, hp_target_side);

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
        if let Some(first_id) = edge.sections.first().copied() {
            set_section_start_preserve_orthogonality(
                &mut graph.edge_sections[first_id.index()],
                start_center,
            );
        }
        if let Some(last_id) = edge.sections.last().copied() {
            set_section_end_preserve_orthogonality(
                &mut graph.edge_sections[last_id.index()],
                end_center,
            );
        }
    }
}

fn set_section_start_preserve_orthogonality(section: &mut elk_graph::EdgeSection, start: elk_core::Point) {
    section.start = start;
    if section.bend_points.is_empty() {
        return;
    }
    let first = section.bend_points[0];
    if (first.x - start.x).abs() > f32::EPSILON && (first.y - start.y).abs() > f32::EPSILON {
        let dx = (first.x - start.x).abs();
        let dy = (first.y - start.y).abs();
        section.bend_points[0] = if dx <= dy {
            elk_core::Point::new(start.x, first.y)
        } else {
            elk_core::Point::new(first.x, start.y)
        };
    }
}

fn set_section_end_preserve_orthogonality(section: &mut elk_graph::EdgeSection, end: elk_core::Point) {
    section.end = end;
    if section.bend_points.is_empty() {
        return;
    }
    let last_idx = section.bend_points.len() - 1;
    let last = section.bend_points[last_idx];
    if (last.x - end.x).abs() > f32::EPSILON && (last.y - end.y).abs() > f32::EPSILON {
        let dx = (last.x - end.x).abs();
        let dy = (last.y - end.y).abs();
        section.bend_points[last_idx] = if dx <= dy {
            elk_core::Point::new(end.x, last.y)
        } else {
            elk_core::Point::new(last.x, end.y)
        };
    }
}

// moved to `elk-alg-common`
