//! Compound graph preprocessing and postprocessing for cross-hierarchy edges.
//!
//! Cross-hierarchy edges connect ports in different subtrees. We temporarily replace
//! their endpoints with hierarchical ports on boundary nodes so routing produces
//! boundary-to-boundary paths. During postprocessing we rebuild the original edge by
//! concatenating the routed boundary trunk with explicit source/target boundary branches
//! instead of simply snapping one section back to the deep endpoints.

use std::collections::{BTreeMap, BTreeSet};

use elk_core::{LayoutDirection, LayoutOptions, Point, PortSide, Rect, Size};
use elk_graph::{EdgeEndpoint, EdgeId, ElkGraph, NodeId, ShapeGeometry};

use crate::pipeline::routing::restore_declared_port_terminals;
use crate::pipeline::util::{
    dedup_points, endpoint_abs_center, endpoint_declared_abs_center, point_along_outward_normal,
};

pub(crate) const TEMP_HIERARCHICAL_PORT_KEY: &str = "spec42.compound.tempHierarchicalPort";
pub(crate) const TEMP_HIERARCHICAL_DUMMY_NODE_KEY: &str = "spec42.compound.tempHierarchicalDummyNode";
pub(crate) const TEMP_HIERARCHICAL_DUMMY_PORT_KEY: &str = "spec42.compound.tempHierarchicalDummyPort";
pub(crate) const TEMP_HIERARCHICAL_DUMMY_PARENT_PORT_KEY: &str =
    "spec42.compound.tempHierarchicalDummyParentPort";

#[derive(Clone, Copy, Debug)]
pub struct CompoundRouteRecord {
    pub original_source: EdgeEndpoint,
    pub original_target: EdgeEndpoint,
    pub effective_source: NodeId,
    pub effective_target: NodeId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossHierarchySegmentKind {
    Output,
    Input,
}

#[derive(Clone, Copy, Debug)]
pub struct CrossHierarchySegmentRecord {
    pub original_edge: EdgeId,
    pub segment_edge: EdgeId,
    pub container: NodeId,
    pub routed_source: EdgeEndpoint,
    pub routed_target: EdgeEndpoint,
    pub kind: CrossHierarchySegmentKind,
}

/// Map from edge id to cross-hierarchy routing metadata.
#[derive(Clone, Debug, Default)]
pub struct CompoundRoutingMap {
    pub original_edges: BTreeMap<EdgeId, CompoundRouteRecord>,
    pub segments_by_original: BTreeMap<EdgeId, Vec<CrossHierarchySegmentRecord>>,
    pub segment_plan_by_original: BTreeMap<EdgeId, Vec<(NodeId, CrossHierarchySegmentKind)>>,
    pub temporary_ports: Vec<(NodeId, elk_graph::PortId)>,
    pub temporary_dummy_nodes: Vec<NodeId>,
    pub temporary_dummy_ports: Vec<elk_graph::PortId>,
}

impl CompoundRoutingMap {
    pub fn is_empty(&self) -> bool {
        self.original_edges.is_empty()
    }

    pub fn original_edge_ids(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.original_edges.keys().copied()
    }

    pub fn original_record(&self, edge_id: EdgeId) -> Option<&CompoundRouteRecord> {
        self.original_edges.get(&edge_id)
    }

    pub fn routed_segments(&self, edge_id: EdgeId) -> &[CrossHierarchySegmentRecord] {
        self.segments_by_original
            .get(&edge_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn planned_segments(&self, edge_id: EdgeId) -> &[(NodeId, CrossHierarchySegmentKind)] {
        self.segment_plan_by_original
            .get(&edge_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn sorted_routed_segments(
        &self,
        graph: &ElkGraph,
        top_container: NodeId,
        edge_id: EdgeId,
    ) -> Vec<CrossHierarchySegmentRecord> {
        let mut segments = self.routed_segments(edge_id).to_vec();
        segments.sort_by(|left, right| {
            compare_cross_hierarchy_segments(graph, top_container, left, right)
        });
        segments
    }
}

#[derive(Clone, Copy, Debug)]
struct RebuildContext {
    edge_id: EdgeId,
    record: CompoundRouteRecord,
    source_center: Point,
    target_center: Point,
    routed_source_center: Point,
    routed_target_center: Point,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
    source_boundary_anchor: Point,
    target_boundary_anchor: Point,
}

#[derive(Clone, Debug)]
struct CompoundEdgeRoute {
    trunk_points: Vec<Point>,
    source_branch: Vec<Point>,
    target_branch: Vec<Point>,
}

impl CompoundEdgeRoute {
    fn into_polyline(self) -> Vec<Point> {
        let mut points = self.source_branch;
        append_points(&mut points, self.trunk_points.into_iter().skip(1));
        let target_branch_reversed = self.target_branch.into_iter().rev().skip(1).collect::<Vec<_>>();
        append_points(&mut points, target_branch_reversed);
        points
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BranchGroupKey {
    endpoint_kind: u8,
    endpoint_index: usize,
    outer_node: NodeId,
    boundary_side: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ClusterBranchGroupKey {
    cluster_node: NodeId,
    outer_node: NodeId,
    boundary_side: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EndpointIdentity {
    kind: u8,
    index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HyperedgeGroupId {
    Source(EndpointIdentity),
    Target(EndpointIdentity),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SharedHierarchicalPortKey {
    node: NodeId,
    side: u8,
    bundle_key: Option<u32>,
    group_id: HyperedgeGroupId,
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

fn mark_temporary_hierarchical_port(graph: &mut ElkGraph, port_id: elk_graph::PortId) {
    graph.ports[port_id.index()].properties.insert(
        elk_graph::PropertyKey::from(TEMP_HIERARCHICAL_PORT_KEY),
        elk_graph::PropertyValue::Bool(true),
    );
}

pub(crate) fn place_hierarchical_port_on_boundary_at_tangent(
    graph: &mut ElkGraph,
    node_id: NodeId,
    port_id: elk_graph::PortId,
    side: PortSide,
    tangent: f32,
) {
    let n = graph.nodes[node_id.index()].geometry;
    let p = &mut graph.ports[port_id.index()].geometry;
    match side {
        PortSide::North => {
            p.x = (tangent - p.width / 2.0).clamp(0.0, (n.width - p.width).max(0.0));
            p.y = -p.height / 2.0;
        }
        PortSide::South => {
            p.x = (tangent - p.width / 2.0).clamp(0.0, (n.width - p.width).max(0.0));
            p.y = n.height - p.height / 2.0;
        }
        PortSide::East => {
            p.x = n.width - p.width / 2.0;
            p.y = (tangent - p.height / 2.0).clamp(0.0, (n.height - p.height).max(0.0));
        }
        PortSide::West => {
            p.x = -p.width / 2.0;
            p.y = (tangent - p.height / 2.0).clamp(0.0, (n.height - p.height).max(0.0));
        }
    }
}

fn endpoint_identity(endpoint: EdgeEndpoint) -> EndpointIdentity {
    EndpointIdentity {
        kind: if endpoint.port.is_some() { 1 } else { 0 },
        index: endpoint
            .port
            .map(|port| port.index())
            .unwrap_or(endpoint.node.index()),
    }
}

fn edge_bundle_key(graph: &ElkGraph, edge_id: EdgeId) -> Option<u32> {
    graph.edges[edge_id.index()]
        .properties
        .get(&elk_graph::PropertyKey::from("elk.edge.bundle"))
        .and_then(|value| value.as_i64())
        .and_then(|value| u32::try_from(value).ok())
}

/// Preprocess cross-hierarchy edges: replace endpoints with hierarchical ports on boundaries.
/// Returns a map of original endpoints for postprocessing.
pub fn preprocess_cross_hierarchy_edges(
    graph: &mut ElkGraph,
    scope_container: NodeId,
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
    let mut to_process: Vec<(EdgeId, NodeId, NodeId, EdgeEndpoint, EdgeEndpoint, Option<u32>)> =
        Vec::new();
    for edge_id in graph.nodes[scope_container.index()].edges.clone() {
        let edge = &graph.edges[edge_id.index()];
        let Some(original_source) = edge.sources.first().copied() else {
            continue;
        };
        let Some(original_target) = edge.targets.first().copied() else {
            continue;
        };
        if graph.nearest_common_ancestor(original_source.node, original_target.node) != Some(scope_container) {
            continue;
        }
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
                edge_bundle_key(graph, edge.id),
            ));
        }
    }

    let mut source_group_counts: BTreeMap<(EndpointIdentity, Option<u32>), usize> = BTreeMap::new();
    let mut target_group_counts: BTreeMap<(EndpointIdentity, Option<u32>), usize> = BTreeMap::new();
    for (_, _, _, original_source, original_target, bundle_key) in &to_process {
        *source_group_counts
            .entry((endpoint_identity(*original_source), *bundle_key))
            .or_default() += 1;
        *target_group_counts
            .entry((endpoint_identity(*original_target), *bundle_key))
            .or_default() += 1;
    }

    let mut shared_ports: BTreeMap<SharedHierarchicalPortKey, elk_graph::PortId> = BTreeMap::new();
    let mut temporary_ports = Vec::new();
    let mut temporary_dummy_nodes = Vec::new();
    let mut temporary_dummy_ports = Vec::new();

    for (edge_id, effective_source, effective_target, original_source, original_target, bundle_key) in
        to_process
    {
        let source_center = endpoint_abs_center(graph, original_source);
        let target_center = endpoint_abs_center(graph, original_target);
        let hp_source_side = hierarchical_port_side_for_edge(direction, source_center, target_center, true);
        let hp_target_side = hierarchical_port_side_for_edge(direction, source_center, target_center, false);

        let source_identity = endpoint_identity(original_source);
        let target_identity = endpoint_identity(original_target);
        let source_group_size = source_group_counts
            .get(&(source_identity, bundle_key))
            .copied()
            .unwrap_or(0);
        let target_group_size = target_group_counts
            .get(&(target_identity, bundle_key))
            .copied()
            .unwrap_or(0);
        let hyperedge_group = if source_group_size > 1 && source_group_size >= target_group_size {
            Some(HyperedgeGroupId::Source(source_identity))
        } else if target_group_size > 1 {
            Some(HyperedgeGroupId::Target(target_identity))
        } else {
            None
        };

        let hp_source = if let Some(group_id) = hyperedge_group {
            let key = SharedHierarchicalPortKey {
                node: effective_source,
                side: side_ordinal(hp_source_side),
                bundle_key,
                group_id,
            };
            *shared_ports.entry(key).or_insert_with(|| {
                let port_id = graph.add_port(effective_source, hp_source_side, port_geom);
                mark_temporary_hierarchical_port(graph, port_id);
                place_hierarchical_port_on_boundary(graph, effective_source, port_id, hp_source_side);
                temporary_ports.push((effective_source, port_id));
                port_id
            })
        } else {
            let port_id = graph.add_port(effective_source, hp_source_side, port_geom);
            mark_temporary_hierarchical_port(graph, port_id);
            place_hierarchical_port_on_boundary(graph, effective_source, port_id, hp_source_side);
            temporary_ports.push((effective_source, port_id));
            port_id
        };
        let hp_target = if let Some(group_id) = hyperedge_group {
            let key = SharedHierarchicalPortKey {
                node: effective_target,
                side: side_ordinal(hp_target_side),
                bundle_key,
                group_id,
            };
            *shared_ports.entry(key).or_insert_with(|| {
                let port_id = graph.add_port(effective_target, hp_target_side, port_geom);
                mark_temporary_hierarchical_port(graph, port_id);
                place_hierarchical_port_on_boundary(graph, effective_target, port_id, hp_target_side);
                temporary_ports.push((effective_target, port_id));
                port_id
            })
        } else {
            let port_id = graph.add_port(effective_target, hp_target_side, port_geom);
            mark_temporary_hierarchical_port(graph, port_id);
            place_hierarchical_port_on_boundary(graph, effective_target, port_id, hp_target_side);
            temporary_ports.push((effective_target, port_id));
            port_id
        };

        let edge = &mut graph.edges[edge_id.index()];
        if let Some(first) = edge.sources.first_mut() {
            *first = EdgeEndpoint::port(effective_source, hp_source);
        }
        if let Some(first) = edge.targets.first_mut() {
            *first = EdgeEndpoint::port(effective_target, hp_target);
        }

        let mut materialized_segments = Vec::new();
        if original_source.node != effective_source {
            let (source_dummy_node, source_dummy_port) =
                create_hierarchical_port_dummy(graph, effective_source, hp_source, hp_source_side);
            temporary_dummy_nodes.push(source_dummy_node);
            temporary_dummy_ports.push(source_dummy_port);
            let source_segment_edge = create_segment_edge(
                graph,
                effective_source,
                edge_id,
                original_source,
                EdgeEndpoint::port(source_dummy_node, source_dummy_port),
            );
            materialized_segments.push(CrossHierarchySegmentRecord {
                original_edge: edge_id,
                segment_edge: source_segment_edge,
                container: effective_source,
                routed_source: original_source,
                routed_target: EdgeEndpoint::port(source_dummy_node, source_dummy_port),
                kind: CrossHierarchySegmentKind::Output,
            });
        }

        map.original_edges.insert(
            edge_id,
            CompoundRouteRecord {
                original_source,
                original_target,
                effective_source,
                effective_target,
            },
        );
        map.segment_plan_by_original.insert(
            edge_id,
            planned_cross_hierarchy_segments(graph, scope_container, original_source.node, original_target.node)
                .into_iter()
                .map(|segment| (segment.container, segment.kind))
                .collect(),
        );
        map.segments_by_original
            .entry(edge_id)
            .or_default()
            .extend(materialized_segments);
        map.segments_by_original
            .entry(edge_id)
            .or_default()
            .push(CrossHierarchySegmentRecord {
                original_edge: edge_id,
                segment_edge: edge_id,
                container: scope_container,
                routed_source: EdgeEndpoint::port(effective_source, hp_source),
                routed_target: EdgeEndpoint::port(effective_target, hp_target),
                kind: CrossHierarchySegmentKind::Output,
            });
        if original_target.node != effective_target {
            let (target_dummy_node, target_dummy_port) =
                create_hierarchical_port_dummy(graph, effective_target, hp_target, hp_target_side);
            temporary_dummy_nodes.push(target_dummy_node);
            temporary_dummy_ports.push(target_dummy_port);
            let target_segment_edge = create_segment_edge(
                graph,
                effective_target,
                edge_id,
                EdgeEndpoint::port(target_dummy_node, target_dummy_port),
                original_target,
            );
            map.segments_by_original
                .entry(edge_id)
                .or_default()
                .push(CrossHierarchySegmentRecord {
                    original_edge: edge_id,
                    segment_edge: target_segment_edge,
                    container: effective_target,
                    routed_source: EdgeEndpoint::port(target_dummy_node, target_dummy_port),
                    routed_target: original_target,
                    kind: CrossHierarchySegmentKind::Input,
                });
        }
    }

    map.temporary_ports = temporary_ports;
    map.temporary_dummy_nodes = temporary_dummy_nodes;
    map.temporary_dummy_ports = temporary_dummy_ports;
    map
}

fn mark_temporary_hierarchical_dummy_node(graph: &mut ElkGraph, node_id: NodeId) {
    graph.nodes[node_id.index()].properties.insert(
        elk_graph::PropertyKey::from(TEMP_HIERARCHICAL_DUMMY_NODE_KEY),
        elk_graph::PropertyValue::Bool(true),
    );
}

fn mark_temporary_hierarchical_dummy_port(
    graph: &mut ElkGraph,
    port_id: elk_graph::PortId,
    parent_port_id: elk_graph::PortId,
) {
    let properties = &mut graph.ports[port_id.index()].properties;
    properties.insert(
        elk_graph::PropertyKey::from(TEMP_HIERARCHICAL_DUMMY_PORT_KEY),
        elk_graph::PropertyValue::Bool(true),
    );
    properties.insert(
        elk_graph::PropertyKey::from(TEMP_HIERARCHICAL_DUMMY_PARENT_PORT_KEY),
        elk_graph::PropertyValue::Int(parent_port_id.index() as i64),
    );
}

fn create_hierarchical_port_dummy(
    graph: &mut ElkGraph,
    container: NodeId,
    parent_port_id: elk_graph::PortId,
    side: PortSide,
) -> (NodeId, elk_graph::PortId) {
    let dummy_node = graph.add_node(
        container,
        ShapeGeometry {
            x: 0.0,
            y: 0.0,
            width: 8.0,
            height: 8.0,
        },
    );
    mark_temporary_hierarchical_dummy_node(graph, dummy_node);
    let dummy_port = graph.add_port(
        dummy_node,
        side,
        ShapeGeometry {
            x: 0.0,
            y: 0.0,
            width: 8.0,
            height: 8.0,
        },
    );
    mark_temporary_hierarchical_dummy_port(graph, dummy_port, parent_port_id);
    set_dummy_node_boundary_geometry_from_parent_port(graph, dummy_node, dummy_port, parent_port_id);
    let side_constraint = match side {
        PortSide::East | PortSide::West => {
            if side == PortSide::West { "FIRST" } else { "LAST" }
        }
        PortSide::North | PortSide::South => {
            if side == PortSide::North { "FIRST" } else { "LAST" }
        }
    };
    graph.nodes[dummy_node.index()]
        .properties
        .insert("elk.layerconstraint", elk_graph::PropertyValue::String(side_constraint.to_string()));
    graph.nodes[dummy_node.index()]
        .properties
        .insert("elk.portconstraints", elk_graph::PropertyValue::String("FIXED_POS".to_string()));
    (dummy_node, dummy_port)
}

pub(crate) fn set_dummy_node_boundary_geometry_from_parent_port(
    graph: &mut ElkGraph,
    dummy_node: NodeId,
    dummy_port: elk_graph::PortId,
    parent_port_id: elk_graph::PortId,
) {
    let parent_port = &graph.ports[parent_port_id.index()];
    let side = parent_port.side;
    let parent_node = parent_port.node;
    let parent_abs = endpoint_abs_center(graph, EdgeEndpoint::port(parent_node, parent_port_id));
    let container_origin = graph.nodes[dummy_node.index()]
        .parent
        .map(|parent| abs_node_origin(graph, parent))
        .unwrap_or_default();
    let local_center = Point::new(parent_abs.x - container_origin.x, parent_abs.y - container_origin.y);
    let node = &mut graph.nodes[dummy_node.index()].geometry;
    node.x = local_center.x - node.width / 2.0;
    node.y = local_center.y - node.height / 2.0;
    let port = &mut graph.ports[dummy_port.index()].geometry;
    match side {
        PortSide::North => {
            port.x = (node.width - port.width).max(0.0) / 2.0;
            port.y = -port.height / 2.0;
        }
        PortSide::South => {
            port.x = (node.width - port.width).max(0.0) / 2.0;
            port.y = node.height - port.height / 2.0;
        }
        PortSide::East => {
            port.x = node.width - port.width / 2.0;
            port.y = (node.height - port.height).max(0.0) / 2.0;
        }
        PortSide::West => {
            port.x = -port.width / 2.0;
            port.y = (node.height - port.height).max(0.0) / 2.0;
        }
    }
}

fn create_segment_edge(
    graph: &mut ElkGraph,
    container: NodeId,
    original_edge: EdgeId,
    source: EdgeEndpoint,
    target: EdgeEndpoint,
) -> EdgeId {
    let edge_id = graph.add_edge(container, vec![source], vec![target]);
    graph.edges[edge_id.index()].properties = graph.edges[original_edge.index()].properties.clone();
    graph.edges[edge_id.index()].labels.clear();
    edge_id
}

/// Postprocess: rebuild cross-hierarchy geometry from preserved boundary anchors.
pub fn postprocess_cross_hierarchy_edges(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
    warnings: &mut Vec<String>,
) {
    let debug_enabled = std::env::var_os("SPEC42_ELK_DEBUG").is_some();
    let mut temporary_segment_edges = BTreeSet::new();
    for edge_id in map.original_edge_ids() {
        let Some(record) = map.original_record(edge_id).copied() else {
            continue;
        };
        let sorted_segments = map.sorted_routed_segments(graph, graph.root, edge_id);
        for segment in &sorted_segments {
            if segment.segment_edge != edge_id {
                temporary_segment_edges.insert(segment.segment_edge);
            }
        }
        let Some(points) = concat_cross_hierarchy_segments_java_style(graph, map, graph.root, edge_id, &record)
        else {
            let edge = &mut graph.edges[edge_id.index()];
            if let Some(first) = edge.sources.first_mut() {
                *first = record.original_source;
            }
            if let Some(first) = edge.targets.first_mut() {
                *first = record.original_target;
            }
            continue;
        };

        if points.len() < 2 {
            continue;
        }

        if debug_enabled {
            warnings.push(format!(
                "elk-layered compound: edge={:?} java_segment_route={}",
                edge_id,
                format_polyline(&points),
            ));
            warnings.push(format!(
                "elk-layered compound: edge={:?} route_bends={} route_inside_start={} route_inside_end={} route_endpoint_intrusions={} route_sibling_intrusions={} route_global_intrusions={}",
                edge_id,
                points.len().saturating_sub(2),
                terminal_approaches_from_inside(graph, &points, record.original_source, true),
                terminal_approaches_from_inside(graph, &points, record.original_target, false),
                count_endpoint_node_intrusions(graph, &points, &record),
                count_crossed_sibling_obstacles(graph, &points, &record),
                count_global_node_intrusions(graph, &points, &record),
            ));
        }

        let edge = &mut graph.edges[edge_id.index()];
        if let Some(first) = edge.sources.first_mut() {
            *first = record.original_source;
        }
        if let Some(first) = edge.targets.first_mut() {
            *first = record.original_target;
        }
        edge.sections.clear();
        let _ = graph.add_edge_section(
            edge_id,
            points[0],
            points[1..points.len() - 1].to_vec(),
            points[points.len() - 1],
        );
        restore_declared_port_terminals(graph, edge_id);
    }

    for edge_id in temporary_segment_edges {
        let edge = &mut graph.edges[edge_id.index()];
        edge.sources.clear();
        edge.targets.clear();
        edge.sections.clear();
        edge.labels.clear();
    }
}

fn concat_cross_hierarchy_segments_java_style(
    graph: &ElkGraph,
    map: &CompoundRoutingMap,
    top_container: NodeId,
    edge_id: EdgeId,
    record: &CompoundRouteRecord,
) -> Option<Vec<Point>> {
    let mut points = flatten_cross_hierarchy_segment_points(graph, map, top_container, edge_id)?;
    let source_anchor = endpoint_declared_abs_center(graph, record.original_source);
    let target_anchor = endpoint_declared_abs_center(graph, record.original_target);
    if should_reverse_points(&points, source_anchor, target_anchor) {
        points.reverse();
    }
    if let Some(first) = points.first_mut() {
        *first = source_anchor;
    }
    if let Some(last) = points.last_mut() {
        *last = target_anchor;
    }
    Some(dedup_points(points))
}

fn orthogonalize_compound_route(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    mut route: CompoundEdgeRoute,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
) -> CompoundEdgeRoute {
    route.trunk_points = orthogonalize_compound_trunk(
        graph,
        record,
        &route.trunk_points,
        source_side,
        target_side,
    );
    route
}

fn orthogonalize_compound_trunk(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    points: &[Point],
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
) -> Vec<Point> {
    const CLEARANCE: f32 = 24.0;

    let simplified = simplify_polyline(points.to_vec());
    if simplified.len() < 2 {
        return simplified;
    }

    let start = simplified[0];
    let end = *simplified.last().unwrap_or(&start);
    match (source_side, target_side) {
        (Some(PortSide::East), Some(PortSide::West))
        | (Some(PortSide::West), Some(PortSide::East)) => {
            let preferred_x = preferred_vertical_trunk_axis(&simplified).unwrap_or((start.x + end.x) * 0.5);
            if let Some(corridor_x) =
                choose_clear_common_ancestor_vertical_corridor(graph, record, start, end, preferred_x, CLEARANCE)
            {
                return simplify_polyline(vec![
                    start,
                    Point::new(corridor_x, start.y),
                    Point::new(corridor_x, end.y),
                    end,
                ]);
            }
        }
        (Some(PortSide::North), Some(PortSide::South))
        | (Some(PortSide::South), Some(PortSide::North)) => {
            let preferred_y =
                preferred_horizontal_trunk_axis(&simplified).unwrap_or((start.y + end.y) * 0.5);
            if let Some(corridor_y) =
                choose_clear_common_ancestor_horizontal_corridor(graph, record, start, end, preferred_y, CLEARANCE)
            {
                return simplify_polyline(vec![
                    start,
                    Point::new(start.x, corridor_y),
                    Point::new(end.x, corridor_y),
                    end,
                ]);
            }
        }
        _ => {}
    }

    if polyline_is_orthogonal(&simplified) {
        simplified
    } else {
        orthogonalize_polyline(simplified, source_side, target_side)
    }
}

fn normalize_candidate_polyline(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    points: Vec<Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<Point> {
    let baseline_points = points;
    if baseline_points.len() < 2 {
        return baseline_points;
    }

    let mut points = simplify_polyline(baseline_points.clone());
    if let Some(side) = start_side {
        points = rebuild_candidate_terminal_branch(points, side, true);
    }
    if let Some(side) = end_side {
        points = rebuild_candidate_terminal_branch(points, side, false);
    }
    if !polyline_is_orthogonal(&points) {
        points = orthogonalize_polyline(points, start_side, end_side);
    }
    let normalized = ensure_terminal_normals(points, start_side, end_side);
    if normalized_candidate_is_worse(
        graph,
        record,
        &baseline_points,
        &normalized,
    ) {
        baseline_points
    } else {
        normalized
    }
}

fn normalized_candidate_is_worse(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    baseline: &[Point],
    normalized: &[Point],
) -> bool {
    if normalized.len() < 2 {
        return true;
    }

    let baseline_inside_start =
        terminal_approaches_from_inside(graph, baseline, record.original_source, true);
    let normalized_inside_start =
        terminal_approaches_from_inside(graph, normalized, record.original_source, true);
    if !baseline_inside_start && normalized_inside_start {
        return true;
    }

    let baseline_inside_end =
        terminal_approaches_from_inside(graph, baseline, record.original_target, false);
    let normalized_inside_end =
        terminal_approaches_from_inside(graph, normalized, record.original_target, false);
    if !baseline_inside_end && normalized_inside_end {
        return true;
    }

    if count_endpoint_node_intrusions(graph, normalized, record)
        > count_endpoint_node_intrusions(graph, baseline, record)
    {
        return true;
    }
    if count_crossed_sibling_obstacles(graph, normalized, record)
        > count_crossed_sibling_obstacles(graph, baseline, record)
    {
        return true;
    }
    count_global_node_intrusions(graph, normalized, record)
        > count_global_node_intrusions(graph, baseline, record)
}

struct CandidateDecision {
    reason: &'static str,
    legacy_points: Vec<Point>,
    rebuilt_points: Vec<Point>,
    legacy_bends: usize,
    rebuilt_bends: usize,
    legacy_inside_start: bool,
    rebuilt_inside_start: bool,
    legacy_inside_end: bool,
    rebuilt_inside_end: bool,
    legacy_endpoint_intrusions: usize,
    rebuilt_endpoint_intrusions: usize,
    legacy_sibling_intrusions: usize,
    rebuilt_sibling_intrusions: usize,
    legacy_global_intrusions: usize,
    rebuilt_global_intrusions: usize,
}

fn choose_best_candidate(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    legacy_points: Vec<Point>,
    rebuilt_points: Vec<Point>,
    _prefers_shared_rebuilt: bool,
) -> (Vec<Point>, CandidateDecision) {
    let legacy_bends = legacy_points.len().saturating_sub(2);
    let rebuilt_bends = rebuilt_points.len().saturating_sub(2);
    let legacy_inside_start =
        terminal_approaches_from_inside(graph, &legacy_points, record.original_source, true);
    let rebuilt_inside_start =
        terminal_approaches_from_inside(graph, &rebuilt_points, record.original_source, true);
    let legacy_inside_end =
        terminal_approaches_from_inside(graph, &legacy_points, record.original_target, false);
    let rebuilt_inside_end =
        terminal_approaches_from_inside(graph, &rebuilt_points, record.original_target, false);
    let legacy_endpoint_intrusions = count_endpoint_node_intrusions(graph, &legacy_points, record);
    let rebuilt_endpoint_intrusions = count_endpoint_node_intrusions(graph, &rebuilt_points, record);
    let legacy_sibling_intrusions = count_crossed_sibling_obstacles(graph, &legacy_points, record);
    let rebuilt_sibling_intrusions = count_crossed_sibling_obstacles(graph, &rebuilt_points, record);
    let legacy_global_intrusions = count_global_node_intrusions(graph, &legacy_points, record);
    let rebuilt_global_intrusions = count_global_node_intrusions(graph, &rebuilt_points, record);
    let decision = |reason: &'static str,
                    chosen: Vec<Point>,
                    legacy_points: Vec<Point>,
                    rebuilt_points: Vec<Point>| {
        (
            chosen,
            CandidateDecision {
                reason,
                legacy_points,
                rebuilt_points,
                legacy_bends,
                rebuilt_bends,
                legacy_inside_start,
                rebuilt_inside_start,
                legacy_inside_end,
                rebuilt_inside_end,
                legacy_endpoint_intrusions,
                rebuilt_endpoint_intrusions,
                legacy_sibling_intrusions,
                rebuilt_sibling_intrusions,
                legacy_global_intrusions,
                rebuilt_global_intrusions,
            },
        )
    };
    if rebuilt_points.len() < 2 {
        return decision("rebuilt-invalid", legacy_points.clone(), legacy_points, rebuilt_points);
    }
    if legacy_points.len() < 2 {
        return decision("legacy-invalid", rebuilt_points.clone(), legacy_points, rebuilt_points);
    }
    if (legacy_inside_start && !rebuilt_inside_start) || (legacy_inside_end && !rebuilt_inside_end) {
        return decision(
            "rebuilt-fixes-inside-approach",
            rebuilt_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_endpoint_intrusions > legacy_endpoint_intrusions {
        return decision(
            "legacy-avoids-endpoint-node-intrusions",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_sibling_intrusions > legacy_sibling_intrusions {
        return decision(
            "legacy-avoids-sibling-obstacles",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_global_intrusions > legacy_global_intrusions {
        return decision(
            "legacy-avoids-global-node-intrusions",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    decision(
        "rebuilt-preferred",
        rebuilt_points.clone(),
        legacy_points,
        rebuilt_points,
    )
}

fn terminal_approaches_from_inside(
    graph: &ElkGraph,
    points: &[Point],
    endpoint: EdgeEndpoint,
    is_start: bool,
) -> bool {
    let Some(port_id) = endpoint.port else {
        return false;
    };
    if points.len() < 2 {
        return false;
    }
    let side = graph.ports[port_id.index()].side;
    if is_start {
        !terminal_matches_side(points[0], points[1], side)
    } else {
        !terminal_matches_side(points[points.len() - 1], points[points.len() - 2], side)
    }
}

fn format_polyline(points: &[Point]) -> String {
    points
        .iter()
        .map(|point| format!("({:.1},{:.1})", point.x, point.y))
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn flatten_edge_points(graph: &ElkGraph, edge_id: EdgeId) -> Option<Vec<Point>> {
    let edge = &graph.edges[edge_id.index()];
    let first_section = edge.sections.first().copied()?;
    let mut points = Vec::new();
    for (index, section_id) in edge.sections.iter().copied().enumerate() {
        let section = &graph.edge_sections[section_id.index()];
        if index == 0 {
            points.push(section.start);
        }
        points.extend(section.bend_points.iter().copied());
        points.push(section.end);
    }
    if points.is_empty() {
        let section = &graph.edge_sections[first_section.index()];
        points.push(section.start);
        points.push(section.end);
    }
    Some(dedup_points(points))
}

fn should_reverse_points(points: &[Point], source_boundary: Point, target_boundary: Point) -> bool {
    let Some(first) = points.first().copied() else {
        return false;
    };
    let Some(last) = points.last().copied() else {
        return false;
    };
    let forward = distance_squared(first, source_boundary) + distance_squared(last, target_boundary);
    let backward = distance_squared(first, target_boundary) + distance_squared(last, source_boundary);
    backward < forward
}

fn endpoint_group_key(
    endpoint: EdgeEndpoint,
    outer_node: NodeId,
    boundary_side: Option<PortSide>,
) -> Option<BranchGroupKey> {
    let side = boundary_side?;
    Some(BranchGroupKey {
        endpoint_kind: if endpoint.port.is_some() { 1 } else { 0 },
        endpoint_index: endpoint
            .port
            .map(|port| port.index())
            .unwrap_or(endpoint.node.index()),
        outer_node,
        boundary_side: side_ordinal(side),
    })
}

fn side_ordinal(side: PortSide) -> u8 {
    match side {
        PortSide::North => 0,
        PortSide::East => 1,
        PortSide::South => 2,
        PortSide::West => 3,
    }
}

fn branch_axis_value(point: Point, side: PortSide) -> f32 {
    match side {
        PortSide::East | PortSide::West => point.y,
        PortSide::North | PortSide::South => point.x,
    }
}

fn branch_outward_point(graph: &ElkGraph, endpoint: EdgeEndpoint) -> Option<(PortSide, Point)> {
    let port_id = endpoint.port?;
    let port_side = graph.ports[port_id.index()].side;
    let center = endpoint_abs_center(graph, endpoint);
    Some((port_side, point_along_outward_normal(center, port_side, 24.0)))
}

fn flatten_cross_hierarchy_segment_points(
    graph: &ElkGraph,
    map: &CompoundRoutingMap,
    top_container: NodeId,
    original_edge_id: EdgeId,
) -> Option<Vec<Point>> {
    let segments = map.sorted_routed_segments(graph, top_container, original_edge_id);
    if segments.is_empty() {
        return flatten_edge_points(graph, original_edge_id);
    }

    let mut points = Vec::new();
    for (index, segment) in segments.iter().enumerate() {
        let mut segment_points = flatten_segment_points_with_routed_terminals(graph, segment)?;
        if index > 0 {
            if points.last().copied() == segment_points.first().copied() {
                segment_points.remove(0);
            } else if let (Some(current), Some(target)) =
                (points.last().copied(), segment_points.first().copied())
            {
                if (current.x - target.x).abs() > f32::EPSILON
                    && (current.y - target.y).abs() > f32::EPSILON
                {
                    append_points(&mut points, [Point::new(current.x, target.y)]);
                }
            }
        }
        append_points(&mut points, segment_points);
    }

    if points.is_empty() {
        None
    } else {
        Some(dedup_points(points))
    }
}

fn flatten_segment_points_with_routed_terminals(
    graph: &ElkGraph,
    segment: &CrossHierarchySegmentRecord,
) -> Option<Vec<Point>> {
    let mut points = flatten_edge_points(graph, segment.segment_edge)?;
    if should_reverse_points(
        &points,
        endpoint_abs_center(graph, segment.routed_source),
        endpoint_abs_center(graph, segment.routed_target),
    ) {
        points.reverse();
    }
    if let Some(first) = points.first_mut() {
        *first = endpoint_abs_center(graph, segment.routed_source);
    }
    if let Some(last) = points.last_mut() {
        *last = endpoint_abs_center(graph, segment.routed_target);
    }
    Some(dedup_points(points))
}

fn compare_cross_hierarchy_segments(
    graph: &ElkGraph,
    top_container: NodeId,
    left: &CrossHierarchySegmentRecord,
    right: &CrossHierarchySegmentRecord,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match (left.kind, right.kind) {
        (CrossHierarchySegmentKind::Output, CrossHierarchySegmentKind::Input) => Ordering::Less,
        (CrossHierarchySegmentKind::Input, CrossHierarchySegmentKind::Output) => Ordering::Greater,
        _ => {
            let left_level = hierarchy_level(graph, left.container, top_container);
            let right_level = hierarchy_level(graph, right.container, top_container);
            match left.kind {
                CrossHierarchySegmentKind::Output => right_level.cmp(&left_level),
                CrossHierarchySegmentKind::Input => left_level.cmp(&right_level),
            }
            .then_with(|| left.segment_edge.index().cmp(&right.segment_edge.index()))
        }
    }
}

fn hierarchy_level(graph: &ElkGraph, node_id: NodeId, top_container: NodeId) -> usize {
    let mut current = Some(node_id);
    let mut level = 0usize;
    while let Some(node_id) = current {
        if node_id == top_container {
            return level;
        }
        current = graph.nodes[node_id.index()].parent;
        level += 1;
    }
    level
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PlannedCrossHierarchySegment {
    container: NodeId,
    kind: CrossHierarchySegmentKind,
}

fn planned_cross_hierarchy_segments(
    graph: &ElkGraph,
    top_container: NodeId,
    source_node: NodeId,
    target_node: NodeId,
) -> Vec<PlannedCrossHierarchySegment> {
    let Some(common_ancestor) = graph.nearest_common_ancestor(source_node, target_node) else {
        return Vec::new();
    };
    if !graph.is_ancestor(top_container, common_ancestor) {
        return Vec::new();
    }

    let mut segments = Vec::new();

    let mut current = source_node;
    while current != common_ancestor {
        let Some(parent) = graph.nodes[current.index()].parent else {
            break;
        };
        if parent == common_ancestor {
            segments.push(PlannedCrossHierarchySegment {
                container: common_ancestor,
                kind: CrossHierarchySegmentKind::Output,
            });
            break;
        }
        segments.push(PlannedCrossHierarchySegment {
            container: parent,
            kind: CrossHierarchySegmentKind::Output,
        });
        current = parent;
    }

    let mut inward = Vec::new();
    let mut current = target_node;
    while current != common_ancestor {
        let Some(parent) = graph.nodes[current.index()].parent else {
            break;
        };
        if parent == common_ancestor {
            inward.push(PlannedCrossHierarchySegment {
                container: common_ancestor,
                kind: CrossHierarchySegmentKind::Input,
            });
            break;
        }
        inward.push(PlannedCrossHierarchySegment {
            container: parent,
            kind: CrossHierarchySegmentKind::Input,
        });
        current = parent;
    }

    inward.reverse();
    segments.extend(inward);
    segments
}

#[derive(Clone, Copy)]
struct BranchSplitEntry {
    edge_id: EdgeId,
    endpoint: EdgeEndpoint,
    outer_node: NodeId,
    boundary_point: Point,
}

fn assign_shared_branch_splits<I>(
    graph: &ElkGraph,
    entries: I,
) -> BTreeMap<EdgeId, Option<f32>>
where
    I: IntoIterator<Item = (EdgeId, EdgeEndpoint, NodeId, Option<PortSide>, Point)>,
{
    let mut grouped: BTreeMap<BranchGroupKey, Vec<BranchSplitEntry>> = BTreeMap::new();
    let mut cluster_grouped: BTreeMap<ClusterBranchGroupKey, Vec<BranchSplitEntry>> =
        BTreeMap::new();
    for (edge_id, endpoint, outer_node, boundary_side, boundary_point) in entries {
        let Some(key) = endpoint_group_key(endpoint, outer_node, boundary_side) else {
            continue;
        };
        let entry = BranchSplitEntry {
            edge_id,
            endpoint,
            outer_node,
            boundary_point,
        };
        grouped.entry(key).or_default().push(entry);
        if let Some(cluster_key) = endpoint_cluster_group_key(graph, endpoint, outer_node, boundary_side)
        {
            cluster_grouped.entry(cluster_key).or_default().push(entry);
        }
    }

    let mut result = BTreeMap::new();
    for (_key, entries) in grouped {
        if entries.len() < 2 {
            continue;
        }
        let split = choose_shared_branch_split(graph, &entries);
        for entry in entries {
            result.insert(entry.edge_id, split);
        }
    }
    for (_key, entries) in cluster_grouped {
        if entries.len() < 2 {
            continue;
        }
        let split = choose_shared_branch_split(graph, &entries);
        for entry in entries {
            result.insert(entry.edge_id, split);
        }
    }
    result
}

fn choose_shared_branch_split(
    graph: &ElkGraph,
    entries: &[impl Copy + BranchSplitEntryLike],
) -> Option<f32> {
    const BRANCH_CLEARANCE: f32 = 24.0;

    let sample = *entries.first()?;
    let (port_side, sample_outward) = branch_outward_point(graph, sample.endpoint())?;
    let outer_rect = node_abs_rect(graph, sample.outer_node());
    let min_value = match port_side {
        PortSide::East | PortSide::West => outer_rect.origin.y + 1.0,
        PortSide::North | PortSide::South => outer_rect.origin.x + 1.0,
    };
    let max_value = match port_side {
        PortSide::East | PortSide::West => outer_rect.max_y() - 1.0,
        PortSide::North | PortSide::South => outer_rect.max_x() - 1.0,
    };

    let mut candidates = Vec::new();
    let mut saw_non_negative = false;
    let mut saw_non_positive = false;

    for entry in entries {
        let (entry_side, outward) = branch_outward_point(graph, entry.endpoint())?;
        if entry_side != port_side {
            return None;
        }
        let boundary_axis = branch_axis_value(entry.boundary_point(), port_side);
        let origin_axis = branch_axis_value(outward, port_side);
        if boundary_axis >= origin_axis + 1e-3 {
            saw_non_negative = true;
        }
        if boundary_axis <= origin_axis - 1e-3 {
            saw_non_positive = true;
        }
        candidates.push(boundary_axis);

        let endpoint_rect = node_abs_rect(graph, entry.endpoint().node);
        match port_side {
            PortSide::East | PortSide::West => {
                candidates.push(endpoint_rect.origin.y - BRANCH_CLEARANCE);
                candidates.push(endpoint_rect.max_y() + BRANCH_CLEARANCE);
                for rect in sibling_obstacle_rects(graph, entry.outer_node(), entry.endpoint().node) {
                    candidates.push(rect.origin.y - BRANCH_CLEARANCE);
                    candidates.push(rect.max_y() + BRANCH_CLEARANCE);
                }
            }
            PortSide::North | PortSide::South => {
                candidates.push(endpoint_rect.origin.x - BRANCH_CLEARANCE);
                candidates.push(endpoint_rect.max_x() + BRANCH_CLEARANCE);
                for rect in sibling_obstacle_rects(graph, entry.outer_node(), entry.endpoint().node) {
                    candidates.push(rect.origin.x - BRANCH_CLEARANCE);
                    candidates.push(rect.max_x() + BRANCH_CLEARANCE);
                }
            }
        }
    }

    if saw_non_negative && saw_non_positive {
        return None;
    }

    let direction_sign = if saw_non_positive { -1.0 } else { 1.0 };
    let reference_axis = branch_axis_value(sample_outward, port_side);

    choose_best_axis_candidate(
        candidates,
        min_value,
        max_value,
        |candidate| {
            entries.iter().all(|entry| {
                let Some((_, outward)) = branch_outward_point(graph, entry.endpoint()) else {
                    return false;
                };
                let candidate_axis = candidate - branch_axis_value(outward, port_side);
                if candidate_axis * direction_sign < -1e-3 {
                    return false;
                }
                shared_branch_intrusions(graph, *entry, port_side, outward, candidate) == 0
            })
        },
        |candidate| {
            entries
                .iter()
                .map(|entry| {
                    let Some((_, outward)) = branch_outward_point(graph, entry.endpoint()) else {
                        return usize::MAX / 4;
                    };
                    let candidate_axis = candidate - branch_axis_value(outward, port_side);
                    let direction_penalty = if candidate_axis * direction_sign < -1e-3 { 1000 } else { 0 };
                    direction_penalty
                        + shared_branch_intrusions(graph, *entry, port_side, outward, candidate)
                })
                .sum()
        },
        |candidate| {
            let branch_cost: f32 = entries
                .iter()
                .filter_map(|entry| {
                    let (_, outward) = branch_outward_point(graph, entry.endpoint())?;
                    Some(
                        (candidate - branch_axis_value(entry.boundary_point(), port_side)).abs()
                            + 0.25 * (candidate - branch_axis_value(outward, port_side)).abs(),
                    )
                })
                .sum();
            branch_cost + 0.05 * (candidate - reference_axis).abs()
        },
    )
}

trait BranchSplitEntryLike {
    fn endpoint(self) -> EdgeEndpoint;
    fn outer_node(self) -> NodeId;
    fn boundary_point(self) -> Point;
}

impl BranchSplitEntryLike for (EdgeId, EdgeEndpoint, NodeId, Point) {
    fn endpoint(self) -> EdgeEndpoint {
        self.1
    }

    fn outer_node(self) -> NodeId {
        self.2
    }

    fn boundary_point(self) -> Point {
        self.3
    }
}

impl BranchSplitEntryLike for BranchSplitEntry {
    fn endpoint(self) -> EdgeEndpoint {
        self.endpoint
    }

    fn outer_node(self) -> NodeId {
        self.outer_node
    }

    fn boundary_point(self) -> Point {
        self.boundary_point
    }
}

fn shared_branch_intrusions(
    graph: &ElkGraph,
    entry: impl Copy + BranchSplitEntryLike,
    port_side: PortSide,
    outward: Point,
    candidate: f32,
) -> usize {
    let endpoint_rect = node_abs_rect(graph, entry.endpoint().node);
    let obstacles = sibling_obstacle_rects(graph, entry.outer_node(), entry.endpoint().node);
    match port_side {
        PortSide::East | PortSide::West => horizontal_branch_intrusions(
            outward,
            entry.boundary_point(),
            candidate,
            endpoint_rect,
            &obstacles,
        ),
        PortSide::North | PortSide::South => vertical_branch_intrusions(
            outward,
            entry.boundary_point(),
            candidate,
            endpoint_rect,
            &obstacles,
        ),
    }
}

fn endpoint_cluster_group_key(
    graph: &ElkGraph,
    endpoint: EdgeEndpoint,
    outer_node: NodeId,
    boundary_side: Option<PortSide>,
) -> Option<ClusterBranchGroupKey> {
    let side = boundary_side?;
    let cluster_node = graph.nodes[endpoint.node.index()].parent?;
    if cluster_node != outer_node {
        return None;
    }
    Some(ClusterBranchGroupKey {
        cluster_node,
        outer_node,
        boundary_side: side_ordinal(side),
    })
}

fn build_endpoint_branch(
    graph: &ElkGraph,
    endpoint: EdgeEndpoint,
    outer_node: NodeId,
    boundary_point: Point,
    boundary_side: Option<PortSide>,
    shared_split: Option<f32>,
) -> Vec<Point> {
    const BRANCH_CLEARANCE: f32 = 24.0;
    let mut points = vec![endpoint_declared_abs_center(graph, endpoint)];
    if let Some(port_id) = endpoint.port {
        let port_side = graph.ports[port_id.index()].side;
        let outward = point_along_outward_normal(points[0], port_side, BRANCH_CLEARANCE);
        let node_rect = node_abs_rect(graph, endpoint.node);
        let outer_rect = node_abs_rect(graph, outer_node);
        let sibling_obstacles = sibling_obstacle_rects(graph, outer_node, endpoint.node);
        append_orthogonal_connection(&mut points, outward, Some(port_side));
        if let Some(split) = shared_split {
            let shared_point = match port_side {
                PortSide::East | PortSide::West => Point::new(outward.x, split),
                PortSide::North | PortSide::South => Point::new(split, outward.y),
            };
            append_orthogonal_connection(&mut points, shared_point, Some(port_side));
        }
        let branch_start = points.last().copied().unwrap_or(outward);
        match port_side {
            PortSide::East | PortSide::West => {
                if shared_split.is_some()
                    && horizontal_connection_intrusions(
                        branch_start,
                        boundary_point,
                        branch_start.y,
                        node_rect,
                        &sibling_obstacles,
                    ) == 0
                {
                    append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::West));
                } else {
                    let detour_y = choose_clear_horizontal_corridor(
                        branch_start,
                        boundary_point,
                        boundary_point.y,
                        outer_rect,
                        node_rect,
                        &sibling_obstacles,
                        BRANCH_CLEARANCE,
                    );
                    if let Some(detour_y) = detour_y {
                        append_orthogonal_connection(
                            &mut points,
                            Point::new(branch_start.x, detour_y),
                            Some(PortSide::North),
                        );
                        append_orthogonal_connection(
                            &mut points,
                            Point::new(boundary_point.x, detour_y),
                            Some(PortSide::West),
                        );
                        append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::North));
                    } else {
                        append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::North));
                    }
                }
            }
            PortSide::North | PortSide::South => {
                if shared_split.is_some()
                    && vertical_connection_intrusions(
                        branch_start,
                        boundary_point,
                        branch_start.x,
                        node_rect,
                        &sibling_obstacles,
                    ) == 0
                {
                    append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::North));
                } else {
                    let detour_x = choose_clear_vertical_corridor(
                        branch_start,
                        boundary_point,
                        boundary_point.x,
                        outer_rect,
                        node_rect,
                        &sibling_obstacles,
                        BRANCH_CLEARANCE,
                    );
                    if let Some(detour_x) = detour_x {
                        append_orthogonal_connection(
                            &mut points,
                            Point::new(detour_x, branch_start.y),
                            Some(PortSide::West),
                        );
                        append_orthogonal_connection(
                            &mut points,
                            Point::new(detour_x, boundary_point.y),
                            Some(PortSide::North),
                        );
                        append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::West));
                    } else {
                        append_orthogonal_connection(&mut points, boundary_point, Some(PortSide::West));
                    }
                }
            }
        }
        return simplify_polyline(points);
    }
    let Some(side) = boundary_side else {
        append_orthogonal_connection(&mut points, boundary_point, None);
        return simplify_polyline(points);
    };

    if endpoint.port.is_none() {
        let exit = boundary_crossing_for_side(
            graph,
            endpoint.node,
            side,
            points[0],
            boundary_point,
        );
        append_orthogonal_connection(&mut points, exit, Some(side));
    }
    append_orthogonal_connection(&mut points, boundary_point, Some(side));
    simplify_polyline(points)
}

fn boundary_crossing_for_side(
    graph: &ElkGraph,
    node_id: NodeId,
    side: PortSide,
    current: Point,
    _toward: Point,
) -> Point {
    let rect = node_abs_rect(graph, node_id);
    match side {
        PortSide::East => Point::new(rect.max_x(), current.y.clamp(rect.origin.y, rect.max_y())),
        PortSide::West => Point::new(rect.origin.x, current.y.clamp(rect.origin.y, rect.max_y())),
        PortSide::North => Point::new(current.x.clamp(rect.origin.x, rect.max_x()), rect.origin.y),
        PortSide::South => Point::new(current.x.clamp(rect.origin.x, rect.max_x()), rect.max_y()),
    }
}

fn node_abs_rect(graph: &ElkGraph, node_id: NodeId) -> Rect {
    let node = &graph.nodes[node_id.index()];
    let origin = abs_node_origin(graph, node_id);
    Rect::new(origin, Size::new(node.geometry.width, node.geometry.height))
}

fn abs_node_origin(graph: &ElkGraph, node_id: NodeId) -> Point {
    let node = &graph.nodes[node_id.index()];
    match node.parent {
        Some(parent) if parent != graph.root => {
            let parent_origin = abs_node_origin(graph, parent);
            Point::new(parent_origin.x + node.geometry.x, parent_origin.y + node.geometry.y)
        }
        _ => Point::new(node.geometry.x, node.geometry.y),
    }
}

fn dominant_side_toward(graph: &ElkGraph, node_id: NodeId, toward: Point) -> Option<PortSide> {
    let rect = node_abs_rect(graph, node_id);
    let center = Point::new(
        rect.origin.x + rect.size.width / 2.0,
        rect.origin.y + rect.size.height / 2.0,
    );
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;
    Some(if dx.abs() >= dy.abs() {
        if dx >= 0.0 { PortSide::East } else { PortSide::West }
    } else if dy >= 0.0 {
        PortSide::South
    } else {
        PortSide::North
    })
}

fn append_orthogonal_connection(points: &mut Vec<Point>, target: Point, preferred_side: Option<PortSide>) {
    let Some(current) = points.last().copied() else {
        points.push(target);
        return;
    };
    if current == target {
        return;
    }
    if (current.x - target.x).abs() > f32::EPSILON && (current.y - target.y).abs() > f32::EPSILON {
        let elbow = match preferred_side {
            Some(PortSide::East | PortSide::West) => Point::new(target.x, current.y),
            Some(PortSide::North | PortSide::South) => Point::new(current.x, target.y),
            None => Point::new(target.x, current.y),
        };
        if elbow != current && elbow != target {
            points.push(elbow);
        }
    }
    if points.last().copied() != Some(target) {
        points.push(target);
    }
}

fn append_points<I>(points: &mut Vec<Point>, more: I)
where
    I: IntoIterator<Item = Point>,
{
    for point in more {
        if points.last().copied() != Some(point) {
            points.push(point);
        }
    }
}

fn distance_squared(a: Point, b: Point) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn count_crossed_sibling_obstacles(
    graph: &ElkGraph,
    points: &[Point],
    record: &CompoundRouteRecord,
) -> usize {
    count_endpoint_sibling_intrusions(graph, points, record.effective_source, record.original_source.node)
        + count_endpoint_sibling_intrusions(graph, points, record.effective_target, record.original_target.node)
}

fn count_endpoint_node_intrusions(
    graph: &ElkGraph,
    points: &[Point],
    record: &CompoundRouteRecord,
) -> usize {
    count_specific_node_intrusions(graph, points, record.original_source, true)
        + count_specific_node_intrusions(graph, points, record.original_target, false)
}

fn count_specific_node_intrusions(
    graph: &ElkGraph,
    points: &[Point],
    endpoint: EdgeEndpoint,
    is_start: bool,
) -> usize {
    let rect = node_abs_rect(graph, endpoint.node);
    points
        .windows(2)
        .enumerate()
        .filter(|(index, segment)| {
            let is_terminal_segment = if is_start {
                *index == 0
            } else {
                *index + 1 == points.len() - 1
            };
            !is_terminal_segment
                && orthogonal_segment_intersects_rect_interior(segment[0], segment[1], rect)
        })
        .count()
}

fn count_endpoint_sibling_intrusions(
    graph: &ElkGraph,
    points: &[Point],
    outer_node: NodeId,
    endpoint_node: NodeId,
) -> usize {
    let obstacles = sibling_obstacle_rects(graph, outer_node, endpoint_node);
    points
        .windows(2)
        .map(|segment| {
            obstacles
                .iter()
                .filter(|rect| orthogonal_segment_intersects_rect_interior(segment[0], segment[1], **rect))
                .count()
        })
        .sum()
}

fn count_global_node_intrusions(
    graph: &ElkGraph,
    points: &[Point],
    record: &CompoundRouteRecord,
) -> usize {
    let source_node = record.original_source.node;
    let target_node = record.original_target.node;
    let source_ancestors = ancestor_node_chain(graph, source_node);
    let target_ancestors = ancestor_node_chain(graph, target_node);
    points
        .windows(2)
        .map(|segment| {
            all_node_ids(graph)
                .into_iter()
                .filter(|node_id| {
                    *node_id != source_node
                        && *node_id != target_node
                        && !source_ancestors.contains(node_id)
                        && !target_ancestors.contains(node_id)
                        && segment_hits_rect(segment[0], segment[1], node_abs_rect(graph, *node_id))
                })
                .count()
        })
        .sum()
}

fn ancestor_node_chain(graph: &ElkGraph, node_id: NodeId) -> BTreeSet<NodeId> {
    let mut ancestors = BTreeSet::new();
    let mut current = graph.nodes[node_id.index()].parent;
    while let Some(parent) = current {
        if !ancestors.insert(parent) {
            break;
        }
        current = graph.nodes[parent.index()].parent;
    }
    ancestors
}

fn all_node_ids(graph: &ElkGraph) -> Vec<NodeId> {
    fn visit(graph: &ElkGraph, node_id: NodeId, out: &mut Vec<NodeId>) {
        out.push(node_id);
        for &child in graph.children_of(node_id) {
            visit(graph, child, out);
        }
    }

    let mut ids = Vec::new();
    visit(graph, graph.root, &mut ids);
    ids
}

fn sibling_obstacle_rects(graph: &ElkGraph, outer_node: NodeId, endpoint_node: NodeId) -> Vec<Rect> {
    let Some(self_child) = direct_child_on_path(graph, outer_node, endpoint_node) else {
        return Vec::new();
    };
    graph.nodes[outer_node.index()]
        .children
        .iter()
        .copied()
        .filter(|child| *child != self_child)
        .map(|child| node_abs_rect(graph, child))
        .collect()
}

fn direct_child_on_path(graph: &ElkGraph, ancestor: NodeId, node_id: NodeId) -> Option<NodeId> {
    if ancestor == node_id {
        return Some(node_id);
    }
    let mut current = node_id;
    loop {
        let parent = graph.nodes[current.index()].parent?;
        if parent == ancestor {
            return Some(current);
        }
        current = parent;
    }
}

fn choose_clear_horizontal_corridor(
    outward: Point,
    boundary_point: Point,
    preferred_y: f32,
    outer_rect: Rect,
    endpoint_rect: Rect,
    sibling_obstacles: &[Rect],
    clearance: f32,
) -> Option<f32> {
    let mut candidates = vec![
        preferred_y,
        endpoint_rect.origin.y - clearance,
        endpoint_rect.max_y() + clearance,
    ];
    for rect in sibling_obstacles {
        candidates.push(rect.origin.y - clearance);
        candidates.push(rect.max_y() + clearance);
    }
    choose_best_axis_candidate(
        candidates,
        outer_rect.origin.y + 1.0,
        outer_rect.max_y() - 1.0,
        |candidate| {
            horizontal_branch_intrusions(
                outward,
                boundary_point,
                candidate,
                endpoint_rect,
                sibling_obstacles,
            ) == 0
        },
        |candidate| {
            horizontal_branch_intrusions(
                outward,
                boundary_point,
                candidate,
                endpoint_rect,
                sibling_obstacles,
            )
        },
        |candidate| (candidate - preferred_y).abs() + 0.25 * (candidate - outward.y).abs(),
    )
}

fn choose_clear_vertical_corridor(
    outward: Point,
    boundary_point: Point,
    preferred_x: f32,
    outer_rect: Rect,
    endpoint_rect: Rect,
    sibling_obstacles: &[Rect],
    clearance: f32,
) -> Option<f32> {
    let mut candidates = vec![
        preferred_x,
        endpoint_rect.origin.x - clearance,
        endpoint_rect.max_x() + clearance,
    ];
    for rect in sibling_obstacles {
        candidates.push(rect.origin.x - clearance);
        candidates.push(rect.max_x() + clearance);
    }
    choose_best_axis_candidate(
        candidates,
        outer_rect.origin.x + 1.0,
        outer_rect.max_x() - 1.0,
        |candidate| {
            vertical_branch_intrusions(
                outward,
                boundary_point,
                candidate,
                endpoint_rect,
                sibling_obstacles,
            ) == 0
        },
        |candidate| {
            vertical_branch_intrusions(
                outward,
                boundary_point,
                candidate,
                endpoint_rect,
                sibling_obstacles,
            )
        },
        |candidate| (candidate - preferred_x).abs() + 0.25 * (candidate - outward.x).abs(),
    )
}

fn choose_best_axis_candidate<F, G, H>(
    candidates: Vec<f32>,
    min_value: f32,
    max_value: f32,
    is_clear: F,
    intrusion_count: G,
    cost: H,
) -> Option<f32>
where
    F: Fn(f32) -> bool,
    G: Fn(f32) -> usize,
    H: Fn(f32) -> f32,
{
    let mut best_clear: Option<(f32, f32)> = None;
    let mut best_fallback: Option<(usize, f32, f32)> = None;
    let mut seen = Vec::new();
    for mut candidate in candidates {
        if !candidate.is_finite() {
            continue;
        }
        candidate = candidate.clamp(min_value, max_value);
        if seen.iter().any(|seen_value: &f32| (seen_value - candidate).abs() <= 1e-3) {
            continue;
        }
        seen.push(candidate);
        let candidate_cost = cost(candidate);
        if is_clear(candidate) {
            match best_clear {
                Some((_, best_cost)) if best_cost <= candidate_cost => {}
                _ => best_clear = Some((candidate, candidate_cost)),
            }
            continue;
        }
        let candidate_intrusions = intrusion_count(candidate);
        match best_fallback {
            Some((best_intrusions, _, best_cost))
                if best_intrusions < candidate_intrusions
                    || (best_intrusions == candidate_intrusions && best_cost <= candidate_cost) => {}
            _ => best_fallback = Some((candidate_intrusions, candidate, candidate_cost)),
        }
    }
    best_clear
        .map(|(candidate, _)| candidate)
        .or_else(|| best_fallback.map(|(_, candidate, _)| candidate))
}

fn horizontal_branch_intrusions(
    outward: Point,
    boundary_point: Point,
    corridor_y: f32,
    endpoint_rect: Rect,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (outward, Point::new(outward.x, corridor_y)),
        (
            Point::new(outward.x, corridor_y),
            Point::new(boundary_point.x, corridor_y),
        ),
        (Point::new(boundary_point.x, corridor_y), boundary_point),
    ];
    count_branch_intrusions(&segments, obstacles)
        + count_endpoint_rect_intrusions(&segments, endpoint_rect)
}

fn choose_clear_common_ancestor_vertical_corridor(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    start: Point,
    end: Point,
    preferred_x: f32,
    clearance: f32,
) -> Option<f32> {
    let obstacles = common_ancestor_obstacle_rects(graph, record)?;
    let mut candidates = vec![preferred_x, start.x, end.x];
    for rect in &obstacles {
        candidates.push(rect.origin.x - clearance);
        candidates.push(rect.max_x() + clearance);
    }
    let min_value = start.x.min(end.x) + 1.0;
    let max_value = start.x.max(end.x) - 1.0;
    choose_best_axis_candidate(
        candidates,
        min_value.min(max_value),
        min_value.max(max_value),
        |candidate| common_ancestor_vertical_intrusions(start, end, candidate, &obstacles) == 0,
        |candidate| common_ancestor_vertical_intrusions(start, end, candidate, &obstacles),
        |candidate| {
            (candidate - preferred_x).abs()
                + 0.1 * (candidate - start.x).abs()
                + 0.1 * (candidate - end.x).abs()
        },
    )
}

fn choose_clear_common_ancestor_horizontal_corridor(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    start: Point,
    end: Point,
    preferred_y: f32,
    clearance: f32,
) -> Option<f32> {
    let obstacles = common_ancestor_obstacle_rects(graph, record)?;
    let mut candidates = vec![preferred_y, start.y, end.y];
    for rect in &obstacles {
        candidates.push(rect.origin.y - clearance);
        candidates.push(rect.max_y() + clearance);
    }
    let min_value = start.y.min(end.y) + 1.0;
    let max_value = start.y.max(end.y) - 1.0;
    choose_best_axis_candidate(
        candidates,
        min_value.min(max_value),
        min_value.max(max_value),
        |candidate| common_ancestor_horizontal_intrusions(start, end, candidate, &obstacles) == 0,
        |candidate| common_ancestor_horizontal_intrusions(start, end, candidate, &obstacles),
        |candidate| {
            (candidate - preferred_y).abs()
                + 0.1 * (candidate - start.y).abs()
                + 0.1 * (candidate - end.y).abs()
        },
    )
}

fn common_ancestor_obstacle_rects(graph: &ElkGraph, record: &CompoundRouteRecord) -> Option<Vec<Rect>> {
    let ancestor = graph.nearest_common_ancestor(record.original_source.node, record.original_target.node)?;
    let source_child = direct_child_on_path(graph, ancestor, record.original_source.node)?;
    let target_child = direct_child_on_path(graph, ancestor, record.original_target.node)?;
    Some(
        graph.nodes[ancestor.index()]
            .children
            .iter()
            .copied()
            .filter(|child| *child != source_child && *child != target_child)
            .map(|child| node_abs_rect(graph, child))
            .collect(),
    )
}

fn common_ancestor_vertical_intrusions(
    start: Point,
    end: Point,
    corridor_x: f32,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (start, Point::new(corridor_x, start.y)),
        (Point::new(corridor_x, start.y), Point::new(corridor_x, end.y)),
        (Point::new(corridor_x, end.y), end),
    ];
    count_branch_intrusions(&segments, obstacles)
}

fn common_ancestor_horizontal_intrusions(
    start: Point,
    end: Point,
    corridor_y: f32,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (start, Point::new(start.x, corridor_y)),
        (Point::new(start.x, corridor_y), Point::new(end.x, corridor_y)),
        (Point::new(end.x, corridor_y), end),
    ];
    count_branch_intrusions(&segments, obstacles)
}

fn preferred_vertical_trunk_axis(points: &[Point]) -> Option<f32> {
    points
        .windows(2)
        .find(|segment| (segment[0].x - segment[1].x).abs() <= f32::EPSILON)
        .map(|segment| segment[0].x)
}

fn preferred_horizontal_trunk_axis(points: &[Point]) -> Option<f32> {
    points
        .windows(2)
        .find(|segment| (segment[0].y - segment[1].y).abs() <= f32::EPSILON)
        .map(|segment| segment[0].y)
}

fn horizontal_connection_intrusions(
    start: Point,
    boundary_point: Point,
    corridor_y: f32,
    endpoint_rect: Rect,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (start, Point::new(boundary_point.x, corridor_y)),
        (Point::new(boundary_point.x, corridor_y), boundary_point),
    ];
    count_branch_intrusions(&segments, obstacles)
        + count_endpoint_rect_intrusions(&segments, endpoint_rect)
}

fn vertical_branch_intrusions(
    outward: Point,
    boundary_point: Point,
    corridor_x: f32,
    endpoint_rect: Rect,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (outward, Point::new(corridor_x, outward.y)),
        (
            Point::new(corridor_x, outward.y),
            Point::new(corridor_x, boundary_point.y),
        ),
        (Point::new(corridor_x, boundary_point.y), boundary_point),
    ];
    count_branch_intrusions(&segments, obstacles)
        + count_endpoint_rect_intrusions(&segments, endpoint_rect)
}

fn vertical_connection_intrusions(
    start: Point,
    boundary_point: Point,
    corridor_x: f32,
    endpoint_rect: Rect,
    obstacles: &[Rect],
) -> usize {
    let segments = [
        (start, Point::new(corridor_x, boundary_point.y)),
        (Point::new(corridor_x, boundary_point.y), boundary_point),
    ];
    count_branch_intrusions(&segments, obstacles)
        + count_endpoint_rect_intrusions(&segments, endpoint_rect)
}

fn count_branch_intrusions(segments: &[(Point, Point)], obstacles: &[Rect]) -> usize {
    segments
        .iter()
        .map(|(a, b)| {
            obstacles
                .iter()
                .filter(|rect| orthogonal_segment_intersects_rect_interior(*a, *b, **rect))
                .count()
        })
        .sum()
}

fn count_endpoint_rect_intrusions(segments: &[(Point, Point)], endpoint_rect: Rect) -> usize {
    segments
        .iter()
        .enumerate()
        .filter(|(index, (a, b))| {
            // The first segment is the perpendicular terminal stub from the port to the outside.
            // That segment is allowed to touch the endpoint node because it is the actual terminal approach.
            *index > 0 && orthogonal_segment_intersects_rect_interior(*a, *b, endpoint_rect)
        })
        .count()
}

fn segment_hits_rect(a: Point, b: Point, rect: Rect) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    !(max_x < rect.origin.x
        || min_x > rect.max_x()
        || max_y < rect.origin.y
        || min_y > rect.max_y())
}

fn orthogonal_segment_intersects_rect_interior(a: Point, b: Point, rect: Rect) -> bool {
    const EPS: f32 = 1e-4;
    if (a.x - b.x).abs() <= EPS {
        let x = a.x;
        if x <= rect.origin.x + EPS || x >= rect.max_x() - EPS {
            return false;
        }
        let seg_min = a.y.min(b.y);
        let seg_max = a.y.max(b.y);
        seg_max > rect.origin.y + EPS && seg_min < rect.max_y() - EPS
    } else if (a.y - b.y).abs() <= EPS {
        let y = a.y;
        if y <= rect.origin.y + EPS || y >= rect.max_y() - EPS {
            return false;
        }
        let seg_min = a.x.min(b.x);
        let seg_max = a.x.max(b.x);
        seg_max > rect.origin.x + EPS && seg_min < rect.max_x() - EPS
    } else {
        false
    }
}

fn orthogonalize_edge_sections_with_sides(
    graph: &mut ElkGraph,
    edge_id: EdgeId,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) {
    let section_ids = graph.edges[edge_id.index()].sections.clone();
    for section_id in section_ids {
        let section = &graph.edge_sections[section_id.index()];
        let points: Vec<elk_core::Point> = std::iter::once(section.start)
            .chain(section.bend_points.iter().copied())
            .chain(std::iter::once(section.end))
            .collect();
        let orthogonal = ensure_terminal_normals(
            orthogonalize_polyline(points, start_side, end_side),
            start_side,
            end_side,
        );
        if orthogonal.len() < 2 {
            continue;
        }
        let section_mut = &mut graph.edge_sections[section_id.index()];
        section_mut.start = orthogonal[0];
        section_mut.end = orthogonal[orthogonal.len() - 1];
        section_mut.bend_points = orthogonal[1..orthogonal.len() - 1].to_vec();
    }
}

fn rebuild_candidate_terminal_branch(
    mut points: Vec<Point>,
    side: PortSide,
    is_start: bool,
) -> Vec<Point> {
    if points.len() < 2 {
        return points;
    }
    let anchor = if is_start {
        points[0]
    } else {
        *points.last().unwrap_or(&points[0])
    };
    if is_start {
        let run_coordinate = terminal_run_coordinate(&points, side, true);
        let Some(run_coordinate) = run_coordinate else {
            return simplify_polyline(points);
        };
        let mut run_end = 1usize;
        while run_end + 1 < points.len()
            && same_terminal_run(points[run_end + 1], run_coordinate, side)
        {
            run_end += 1;
        }

        let mut rebuilt = Vec::with_capacity(points.len());
        rebuilt.push(anchor);
        let bridge = terminal_bridge_point(anchor, run_coordinate, side);
        if rebuilt.last().copied() != Some(bridge) && bridge != anchor {
            rebuilt.push(bridge);
        }
        rebuilt.extend(points.into_iter().skip(run_end + 1));
        simplify_polyline(rebuilt)
    } else {
        let last = points.len() - 1;
        let run_coordinate = terminal_run_coordinate(&points, side, false);
        let Some(run_coordinate) = run_coordinate else {
            return simplify_polyline(points);
        };
        let mut run_start = last.saturating_sub(1);
        while run_start > 0 && same_terminal_run(points[run_start - 1], run_coordinate, side) {
            run_start -= 1;
        }

        let mut rebuilt = Vec::with_capacity(points.len());
        rebuilt.extend(points.iter().copied().take(run_start));
        let bridge = terminal_bridge_point(anchor, run_coordinate, side);
        if rebuilt.last().copied() != Some(bridge) && bridge != anchor {
            rebuilt.push(bridge);
        }
        rebuilt.push(anchor);
        simplify_polyline(rebuilt)
    }
}

fn terminal_run_coordinate(points: &[Point], side: PortSide, is_start: bool) -> Option<f32> {
    if points.len() < 2 {
        return None;
    }
    let run_point = if is_start {
        points[1]
    } else {
        points[points.len() - 2]
    };
    Some(match side {
        PortSide::East | PortSide::West => run_point.x,
        PortSide::North | PortSide::South => run_point.y,
    })
}

fn same_terminal_run(point: Point, coordinate: f32, side: PortSide) -> bool {
    match side {
        PortSide::East | PortSide::West => (point.x - coordinate).abs() <= 1e-5,
        PortSide::North | PortSide::South => (point.y - coordinate).abs() <= 1e-5,
    }
}

fn terminal_bridge_point(anchor: Point, run_coordinate: f32, side: PortSide) -> Point {
    match side {
        PortSide::East | PortSide::West => Point::new(run_coordinate, anchor.y),
        PortSide::North | PortSide::South => Point::new(anchor.x, run_coordinate),
    }
}

fn ensure_terminal_normals(
    mut points: Vec<elk_core::Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<elk_core::Point> {
    if let Some(side) = start_side {
        points = ensure_start_terminal_normal(points, side);
    }
    if let Some(side) = end_side {
        points = ensure_end_terminal_normal(points, side);
    }
    simplify_polyline(points)
}

fn ensure_start_terminal_normal(points: Vec<elk_core::Point>, side: PortSide) -> Vec<elk_core::Point> {
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    if points.len() < 2 {
        return points;
    }
    let start = points[0];
    let next = points[1];
    if terminal_matches_side(start, next, side) {
        return points;
    }
    let route = point_along_outward_normal(start, side, PORT_NORMAL_OFFSET);
    let mut rebuilt = Vec::with_capacity(points.len() + 2);
    rebuilt.push(start);
    if rebuilt.last().copied() != Some(route) {
        rebuilt.push(route);
    }
    if route != next {
        let bridge = match side {
            PortSide::East | PortSide::West => elk_core::Point::new(route.x, next.y),
            PortSide::North | PortSide::South => elk_core::Point::new(next.x, route.y),
        };
        if bridge != route && bridge != next {
            rebuilt.push(bridge);
        }
    }
    rebuilt.extend(points.into_iter().skip(1));
    rebuilt
}

fn ensure_end_terminal_normal(points: Vec<elk_core::Point>, side: PortSide) -> Vec<elk_core::Point> {
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    if points.len() < 2 {
        return points;
    }
    let end = *points.last().unwrap_or(&elk_core::Point::new(0.0, 0.0));
    let prev = points[points.len() - 2];
    if terminal_matches_side(end, prev, side) {
        return points;
    }
    let route = point_along_outward_normal(end, side, PORT_NORMAL_OFFSET);
    let mut rebuilt = Vec::with_capacity(points.len() + 2);
    rebuilt.extend(points.iter().copied().take(points.len() - 1));
    if rebuilt.last().copied() != Some(route) {
        let bridge = match side {
            PortSide::East | PortSide::West => elk_core::Point::new(route.x, prev.y),
            PortSide::North | PortSide::South => elk_core::Point::new(prev.x, route.y),
        };
        if rebuilt.last().copied() != Some(bridge) && bridge != route && bridge != end {
            rebuilt.push(bridge);
        }
        rebuilt.push(route);
    }
    rebuilt.push(end);
    rebuilt
}

fn terminal_matches_side(endpoint: elk_core::Point, neighbor: elk_core::Point, side: PortSide) -> bool {
    match side {
        PortSide::East => (neighbor.y - endpoint.y).abs() <= 1e-5 && neighbor.x >= endpoint.x - 1e-5,
        PortSide::West => (neighbor.y - endpoint.y).abs() <= 1e-5 && neighbor.x <= endpoint.x + 1e-5,
        PortSide::North => (neighbor.x - endpoint.x).abs() <= 1e-5 && neighbor.y <= endpoint.y + 1e-5,
        PortSide::South => (neighbor.x - endpoint.x).abs() <= 1e-5 && neighbor.y >= endpoint.y - 1e-5,
    }
}

fn orthogonalize_polyline(
    points: Vec<elk_core::Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<elk_core::Point> {
    const EPS: f32 = 1e-5;
    if points.len() < 2 {
        return points;
    }
    let mut out = vec![points[0]];
    for idx in 0..points.len() - 1 {
        let a = *out.last().unwrap_or(&points[idx]);
        let b = points[idx + 1];
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        if dx <= EPS && dy <= EPS {
            continue;
        }
        if dx > EPS && dy > EPS {
            let via = choose_orthogonal_elbow(&points, idx, a, b, start_side, end_side);
            if out.last().copied() != Some(via) {
                out.push(via);
            }
        }
        if out.last().copied() != Some(b) {
            out.push(b);
        }
    }
    simplify_polyline(out)
}

fn choose_orthogonal_elbow(
    points: &[elk_core::Point],
    idx: usize,
    a: elk_core::Point,
    b: elk_core::Point,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> elk_core::Point {
    if idx == 0 {
        if let Some(side) = start_side {
            return match side {
                PortSide::East | PortSide::West => elk_core::Point::new(b.x, a.y),
                PortSide::North | PortSide::South => elk_core::Point::new(a.x, b.y),
            };
        }
    }
    if idx + 1 == points.len() - 1 {
        if let Some(side) = end_side {
            return match side {
                PortSide::East | PortSide::West => elk_core::Point::new(a.x, b.y),
                PortSide::North | PortSide::South => elk_core::Point::new(b.x, a.y),
            };
        }
    }
    if idx > 0 {
        let prev = points[idx - 1];
        if (prev.x - a.x).abs() <= f32::EPSILON {
            return elk_core::Point::new(a.x, b.y);
        }
        if (prev.y - a.y).abs() <= f32::EPSILON {
            return elk_core::Point::new(b.x, a.y);
        }
    }
    if (a.x - b.x).abs() >= (a.y - b.y).abs() {
        elk_core::Point::new(b.x, a.y)
    } else {
        elk_core::Point::new(a.x, b.y)
    }
}

fn simplify_polyline(points: Vec<elk_core::Point>) -> Vec<elk_core::Point> {
    let mut out = Vec::with_capacity(points.len());
    for point in points {
        if out.last().copied() == Some(point) {
            continue;
        }
        out.push(point);
        while out.len() >= 3 {
            let len = out.len();
            let a = out[len - 3];
            let b = out[len - 2];
            let c = out[len - 1];
            let collinear_x = (a.x - b.x).abs() <= 1e-5 && (b.x - c.x).abs() <= 1e-5;
            let collinear_y = (a.y - b.y).abs() <= 1e-5 && (b.y - c.y).abs() <= 1e-5;
            if collinear_x || collinear_y {
                out.remove(len - 2);
            } else {
                break;
            }
        }
    }
    out
}

fn polyline_is_orthogonal(points: &[elk_core::Point]) -> bool {
    const EPS: f32 = 1e-5;
    points.windows(2).all(|segment| {
        let a = segment[0];
        let b = segment[1];
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        dx <= EPS || dy <= EPS
    })
}

// moved to `elk-alg-common`

#[cfg(test)]
mod tests {
    use super::*;
    use elk_core::PortSide;
    use elk_graph::ShapeGeometry;
    use crate::pipeline::hierarchical_ports::{
        correct_hierarchical_port_route_terminals, hide_temporary_hierarchical_ports,
        refresh_hierarchical_port_coordinates,
    };

    #[test]
    fn endpoint_branch_avoids_sibling_obstacle_inside_outer_container() {
        let mut graph = ElkGraph::new();
        let outer = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 100.0,
                y: 100.0,
                width: 420.0,
                height: 420.0,
            },
        );
        let sibling = graph.add_node(
            outer,
            ShapeGeometry {
                x: 72.0,
                y: 120.0,
                width: 260.0,
                height: 124.0,
            },
        );
        let endpoint_node = graph.add_node(
            outer,
            ShapeGeometry {
                x: 72.0,
                y: 272.0,
                width: 260.0,
                height: 124.0,
            },
        );
        let endpoint_port = graph.add_port(
            endpoint_node,
            PortSide::East,
            ShapeGeometry {
                x: 256.0,
                y: 58.0,
                width: 8.0,
                height: 8.0,
            },
        );

        let branch = build_endpoint_branch(
            &graph,
            EdgeEndpoint::port(endpoint_node, endpoint_port),
            outer,
            Point::new(100.0, 282.0),
            Some(PortSide::West),
            None,
        );

        let sibling_rect = node_abs_rect(&graph, sibling);
        assert!(
            branch.windows(2).all(|segment| {
                !orthogonal_segment_intersects_rect_interior(segment[0], segment[1], sibling_rect)
            }),
            "branch should avoid sibling obstacle, got {}",
            format_polyline(&branch)
        );
        assert!(
            branch.iter().any(|point| point.y < sibling_rect.origin.y || point.y > sibling_rect.max_y()),
            "branch should detour outside sibling band, got {}",
            format_polyline(&branch)
        );
    }

    #[test]
    fn shared_branch_split_picks_clear_group_corridor() {
        let mut graph = ElkGraph::new();
        let outer = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 420.0,
                height: 420.0,
            },
        );
        let blocker = graph.add_node(
            outer,
            ShapeGeometry {
                x: 148.0,
                y: 96.0,
                width: 124.0,
                height: 84.0,
            },
        );
        let endpoint_a = graph.add_node(
            outer,
            ShapeGeometry {
                x: 24.0,
                y: 208.0,
                width: 100.0,
                height: 48.0,
            },
        );
        let endpoint_b = graph.add_node(
            outer,
            ShapeGeometry {
                x: 24.0,
                y: 288.0,
                width: 100.0,
                height: 48.0,
            },
        );
        let target = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 520.0,
                y: 120.0,
                width: 100.0,
                height: 48.0,
            },
        );
        let port_a = graph.add_port(
            endpoint_a,
            PortSide::East,
            ShapeGeometry {
                x: 96.0,
                y: 20.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let port_b = graph.add_port(
            endpoint_b,
            PortSide::East,
            ShapeGeometry {
                x: 96.0,
                y: 20.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let target_port = graph.add_port(
            target,
            PortSide::West,
            ShapeGeometry {
                x: -4.0,
                y: 20.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let edge_a = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(endpoint_a, port_a)],
            vec![EdgeEndpoint::port(target, target_port)],
        );
        let edge_b = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(endpoint_b, port_b)],
            vec![EdgeEndpoint::port(target, target_port)],
        );

        let split = choose_shared_branch_split(
            &graph,
            &[
                BranchSplitEntry {
                    edge_id: edge_a,
                    endpoint: EdgeEndpoint::port(endpoint_a, port_a),
                    outer_node: outer,
                    boundary_point: Point::new(320.0, 120.0),
                },
                BranchSplitEntry {
                    edge_id: edge_b,
                    endpoint: EdgeEndpoint::port(endpoint_b, port_b),
                    outer_node: outer,
                    boundary_point: Point::new(320.0, 140.0),
                },
            ],
        )
        .expect("shared split");

        let blocker_rect = node_abs_rect(&graph, blocker);
        assert!(
            split > blocker_rect.max_y() + 1e-3,
            "expected split to move out of the blocked band, got {split}"
        );
        assert!(
            [edge_a, edge_b]
                .into_iter()
                .zip([
                    EdgeEndpoint::port(endpoint_a, port_a),
                    EdgeEndpoint::port(endpoint_b, port_b),
                ])
                .all(|(edge_id, endpoint)| {
                    let outward = branch_outward_point(&graph, endpoint)
                        .expect("outward")
                        .1;
                    shared_branch_intrusions(
                        &graph,
                        BranchSplitEntry {
                            edge_id,
                            endpoint,
                            outer_node: outer,
                            boundary_point: if edge_id == edge_a {
                                Point::new(320.0, 120.0)
                            } else {
                                Point::new(320.0, 140.0)
                            },
                        },
                        PortSide::East,
                        outward,
                        split,
                    ) == 0
                }),
            "expected chosen split to be clear for all grouped branches, got {split}"
        );
    }

    #[test]
    fn refresh_hierarchical_port_coordinates_places_north_port_from_attached_endpoint() {
        let mut graph = ElkGraph::new();
        let outer = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 100.0,
                y: 200.0,
                width: 300.0,
                height: 240.0,
            },
        );
        let inner = graph.add_node(
            outer,
            ShapeGeometry {
                x: 40.0,
                y: 120.0,
                width: 100.0,
                height: 60.0,
            },
        );
        let inner_port = graph.add_port(
            inner,
            PortSide::North,
            ShapeGeometry {
                x: 40.0,
                y: -4.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let routed_port = graph.add_port(
            outer,
            PortSide::North,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 8.0,
                height: 8.0,
            },
        );
        place_hierarchical_port_on_boundary(&mut graph, outer, routed_port, PortSide::North);

        let edge_id = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(inner, inner_port)],
            vec![EdgeEndpoint::port(outer, routed_port)],
        );
        let map = CompoundRoutingMap {
            original_edges: BTreeMap::from([(
                edge_id,
                CompoundRouteRecord {
                    original_source: EdgeEndpoint::port(inner, inner_port),
                    original_target: EdgeEndpoint::port(inner, inner_port),
                    effective_source: outer,
                    effective_target: outer,
                },
            )]),
            segments_by_original: BTreeMap::from([(
                edge_id,
                vec![CrossHierarchySegmentRecord {
                    original_edge: edge_id,
                    segment_edge: edge_id,
                    container: graph.root,
                    routed_source: EdgeEndpoint::port(outer, routed_port),
                    routed_target: EdgeEndpoint::port(outer, routed_port),
                    kind: CrossHierarchySegmentKind::Output,
                }],
            )]),
            segment_plan_by_original: BTreeMap::new(),
            temporary_ports: Vec::new(),
            temporary_dummy_nodes: Vec::new(),
            temporary_dummy_ports: Vec::new(),
        };

        refresh_hierarchical_port_coordinates(&mut graph, &map);

        let refreshed = graph.ports[routed_port.index()].geometry;
        let expected_center_x = endpoint_abs_center(&graph, EdgeEndpoint::port(inner, inner_port)).x;
        let actual_center_x = graph.nodes[outer.index()].geometry.x + refreshed.x + refreshed.width / 2.0;
        assert!(
            (actual_center_x - expected_center_x).abs() <= 1e-3,
            "expected refreshed north port center x {expected_center_x}, got {actual_center_x}"
        );
    }

    #[test]
    fn planned_cross_hierarchy_segments_walks_out_then_in() {
        let mut graph = ElkGraph::new();
        let source_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let source_inner = graph.add_node(source_cluster, ShapeGeometry::default());
        let target_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let target_inner = graph.add_node(target_cluster, ShapeGeometry::default());

        let planned = planned_cross_hierarchy_segments(&graph, graph.root, source_inner, target_inner);
        assert_eq!(
            planned,
            vec![
                PlannedCrossHierarchySegment {
                    container: source_cluster,
                    kind: CrossHierarchySegmentKind::Output,
                },
                PlannedCrossHierarchySegment {
                    container: graph.root,
                    kind: CrossHierarchySegmentKind::Output,
                },
                PlannedCrossHierarchySegment {
                    container: graph.root,
                    kind: CrossHierarchySegmentKind::Input,
                },
                PlannedCrossHierarchySegment {
                    container: target_cluster,
                    kind: CrossHierarchySegmentKind::Input,
                },
            ]
        );
    }

    #[test]
    fn planned_cross_hierarchy_segments_for_siblings_stays_at_common_ancestor() {
        let mut graph = ElkGraph::new();
        let cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let left = graph.add_node(cluster, ShapeGeometry::default());
        let right = graph.add_node(cluster, ShapeGeometry::default());

        let planned = planned_cross_hierarchy_segments(&graph, graph.root, left, right);
        assert_eq!(
            planned,
            vec![
                PlannedCrossHierarchySegment {
                    container: cluster,
                    kind: CrossHierarchySegmentKind::Output,
                },
                PlannedCrossHierarchySegment {
                    container: cluster,
                    kind: CrossHierarchySegmentKind::Input,
                },
            ]
        );
    }

    #[test]
    fn preprocess_records_java_style_segment_plan() {
        let mut graph = ElkGraph::new();
        let left_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let left_inner = graph.add_node(left_cluster, ShapeGeometry::default());
        let left_port = graph.add_port(
            left_inner,
            PortSide::East,
            ShapeGeometry {
                x: 16.0,
                y: 4.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let right_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let right_inner = graph.add_node(right_cluster, ShapeGeometry::default());
        let right_port = graph.add_port(
            right_inner,
            PortSide::West,
            ShapeGeometry {
                x: -4.0,
                y: 4.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let edge_id = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(left_inner, left_port)],
            vec![EdgeEndpoint::port(right_inner, right_port)],
        );
        let local_nodes = BTreeSet::from([left_cluster, right_cluster]);
        let options = LayoutOptions::default();
        let root = graph.root;

        let map = preprocess_cross_hierarchy_edges(&mut graph, root, &local_nodes, &options);

        assert_eq!(
            map.planned_segments(edge_id),
            &[
                (left_cluster, CrossHierarchySegmentKind::Output),
                (root, CrossHierarchySegmentKind::Output),
                (root, CrossHierarchySegmentKind::Input),
                (right_cluster, CrossHierarchySegmentKind::Input),
            ]
        );
    }

    #[test]
    fn preprocess_materializes_child_scope_segment_edges_for_nested_endpoints() {
        let mut graph = ElkGraph::new();
        let left_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let left_inner = graph.add_node(left_cluster, ShapeGeometry::default());
        let left_port = graph.add_port(left_inner, PortSide::East, ShapeGeometry::default());
        let right_cluster = graph.add_node(graph.root, ShapeGeometry::default());
        let right_inner = graph.add_node(right_cluster, ShapeGeometry::default());
        let right_port = graph.add_port(right_inner, PortSide::West, ShapeGeometry::default());
        let edge_id = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(left_inner, left_port)],
            vec![EdgeEndpoint::port(right_inner, right_port)],
        );
        let local_nodes = BTreeSet::from([left_cluster, right_cluster]);
        let options = LayoutOptions::default();
        let root = graph.root;

        let map = preprocess_cross_hierarchy_edges(&mut graph, root, &local_nodes, &options);

        let segments = map.sorted_routed_segments(&graph, root, edge_id);
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].container, left_cluster);
        assert_eq!(segments[0].kind, CrossHierarchySegmentKind::Output);
        assert_eq!(segments[1].segment_edge, edge_id);
        assert_eq!(segments[1].container, root);
        assert_eq!(segments[2].container, right_cluster);
        assert_eq!(segments[2].kind, CrossHierarchySegmentKind::Input);
        assert!(graph.nodes[left_cluster.index()].edges.contains(&segments[0].segment_edge));
        assert!(graph.nodes[right_cluster.index()].edges.contains(&segments[2].segment_edge));
    }

    #[test]
    fn concat_cross_hierarchy_segments_java_style_inserts_boundary_bend_between_segments() {
        let mut graph = ElkGraph::new();
        let source_node = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 40.0,
                height: 40.0,
            },
        );
        let target_node = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 240.0,
                y: 120.0,
                width: 40.0,
                height: 40.0,
            },
        );
        let source_port = graph.add_port(
            source_node,
            PortSide::East,
            ShapeGeometry {
                x: 36.0,
                y: 16.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let target_port = graph.add_port(
            target_node,
            PortSide::West,
            ShapeGeometry {
                x: -4.0,
                y: 16.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let boundary_a = graph.add_port(
            graph.root,
            PortSide::East,
            ShapeGeometry {
                x: 100.0,
                y: 96.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let boundary_b = graph.add_port(
            graph.root,
            PortSide::West,
            ShapeGeometry {
                x: 156.0,
                y: 136.0,
                width: 8.0,
                height: 8.0,
            },
        );

        let original_edge = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(source_node, source_port)],
            vec![EdgeEndpoint::port(target_node, target_port)],
        );
        let source_segment = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(source_node, source_port)],
            vec![EdgeEndpoint::port(graph.root, boundary_a)],
        );
        let target_segment = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(graph.root, boundary_b)],
            vec![EdgeEndpoint::port(target_node, target_port)],
        );

        let _ = graph.add_edge_section(
            source_segment,
            Point::new(40.0, 20.0),
            vec![Point::new(100.0, 20.0)],
            Point::new(100.0, 100.0),
        );
        let _ = graph.add_edge_section(
            target_segment,
            Point::new(100.0, 140.0),
            vec![Point::new(180.0, 140.0)],
            Point::new(240.0, 160.0),
        );

        let map = CompoundRoutingMap {
            original_edges: BTreeMap::from([(
                original_edge,
                CompoundRouteRecord {
                    original_source: EdgeEndpoint::port(source_node, source_port),
                    original_target: EdgeEndpoint::port(target_node, target_port),
                    effective_source: graph.root,
                    effective_target: graph.root,
                },
            )]),
            segments_by_original: BTreeMap::from([(
                original_edge,
                vec![
                    CrossHierarchySegmentRecord {
                        original_edge,
                        segment_edge: source_segment,
                        container: graph.root,
                        routed_source: EdgeEndpoint::port(source_node, source_port),
                        routed_target: EdgeEndpoint::port(graph.root, boundary_a),
                        kind: CrossHierarchySegmentKind::Output,
                    },
                    CrossHierarchySegmentRecord {
                        original_edge,
                        segment_edge: target_segment,
                        container: graph.root,
                        routed_source: EdgeEndpoint::port(graph.root, boundary_b),
                        routed_target: EdgeEndpoint::port(target_node, target_port),
                        kind: CrossHierarchySegmentKind::Input,
                    },
                ],
            )]),
            segment_plan_by_original: BTreeMap::new(),
            temporary_ports: Vec::new(),
            temporary_dummy_nodes: Vec::new(),
            temporary_dummy_ports: Vec::new(),
        };

        let points = concat_cross_hierarchy_segments_java_style(
            &graph,
            &map,
            graph.root,
            original_edge,
            map.original_record(original_edge).expect("record"),
        )
        .expect("points");

        assert_eq!(
            points,
            vec![
                endpoint_declared_abs_center(&graph, EdgeEndpoint::port(source_node, source_port)),
                Point::new(100.0, 20.0),
                Point::new(100.0, 100.0),
                Point::new(100.0, 140.0),
                Point::new(164.0, 140.0),
                Point::new(180.0, 140.0),
                endpoint_declared_abs_center(&graph, EdgeEndpoint::port(target_node, target_port)),
            ]
        );
        assert!(polyline_is_orthogonal(&points));
    }

    #[test]
    fn correct_hierarchical_port_route_terminals_snaps_terminals_to_refreshed_anchor() {
        let mut graph = ElkGraph::new();
        let outer = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 100.0,
                y: 100.0,
                width: 220.0,
                height: 180.0,
            },
        );
        let routed_port = graph.add_port(
            outer,
            PortSide::West,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 8.0,
                height: 8.0,
            },
        );
        place_hierarchical_port_on_boundary_at_tangent(
            &mut graph,
            outer,
            routed_port,
            PortSide::West,
            120.0,
        );
        let edge_id = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::port(outer, routed_port)],
            vec![EdgeEndpoint::node(outer)],
        );
        let _ = graph.add_edge_section(
            edge_id,
            Point::new(0.0, 0.0),
            vec![Point::new(140.0, 80.0)],
            Point::new(200.0, 80.0),
        );
        let map = CompoundRoutingMap {
            original_edges: BTreeMap::from([(
                edge_id,
                CompoundRouteRecord {
                    original_source: EdgeEndpoint::node(outer),
                    original_target: EdgeEndpoint::node(outer),
                    effective_source: outer,
                    effective_target: outer,
                },
            )]),
            segments_by_original: BTreeMap::from([(
                edge_id,
                vec![CrossHierarchySegmentRecord {
                    original_edge: edge_id,
                    segment_edge: edge_id,
                    container: graph.root,
                    routed_source: EdgeEndpoint::port(outer, routed_port),
                    routed_target: EdgeEndpoint::node(outer),
                    kind: CrossHierarchySegmentKind::Output,
                }],
            )]),
            segment_plan_by_original: BTreeMap::new(),
            temporary_ports: Vec::new(),
            temporary_dummy_nodes: Vec::new(),
            temporary_dummy_ports: Vec::new(),
        };

        correct_hierarchical_port_route_terminals(&mut graph, &map);

        let section = &graph.edge_sections[graph.edges[edge_id.index()].sections[0].index()];
        let anchor = endpoint_abs_center(&graph, EdgeEndpoint::port(outer, routed_port));
        assert_eq!(section.start, anchor);
        assert!((section.bend_points[0].y - anchor.y).abs() <= 1e-5);
    }

    #[test]
    fn hide_temporary_hierarchical_ports_marks_ports_for_export_filtering() {
        let mut graph = ElkGraph::new();
        let node = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 80.0,
            },
        );
        let stable_port = graph.add_port(
            node,
            PortSide::East,
            ShapeGeometry {
                x: 116.0,
                y: 16.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let temp_a = graph.add_port(
            node,
            PortSide::North,
            ShapeGeometry {
                x: 20.0,
                y: -4.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let temp_b = graph.add_port(
            node,
            PortSide::South,
            ShapeGeometry {
                x: 80.0,
                y: 76.0,
                width: 8.0,
                height: 8.0,
            },
        );
        let map = CompoundRoutingMap {
            original_edges: BTreeMap::new(),
            segments_by_original: BTreeMap::new(),
            segment_plan_by_original: BTreeMap::new(),
            temporary_ports: vec![(node, temp_a), (node, temp_b)],
            temporary_dummy_nodes: Vec::new(),
            temporary_dummy_ports: Vec::new(),
        };

        hide_temporary_hierarchical_ports(&mut graph, &map);

        assert_eq!(graph.nodes[node.index()].ports, vec![stable_port, temp_a, temp_b]);
        assert_eq!(
            graph.ports[temp_a.index()]
                .properties
                .get(&elk_graph::PropertyKey::from("spec42.temporary_hierarchical_port"))
                .and_then(|value| value.as_bool()),
            Some(true)
        );
        assert_eq!(
            graph.ports[temp_b.index()]
                .properties
                .get(&elk_graph::PropertyKey::from("spec42.temporary_hierarchical_port"))
                .and_then(|value| value.as_bool()),
            Some(true)
        );
    }
}
