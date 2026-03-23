//! Compound graph preprocessing and postprocessing for cross-hierarchy edges.
//!
//! Cross-hierarchy edges connect ports in different subtrees. We temporarily replace
//! their endpoints with hierarchical ports on boundary nodes so routing produces
//! boundary-to-boundary paths. During postprocessing we rebuild the original edge by
//! concatenating the routed boundary trunk with explicit source/target boundary branches
//! instead of simply snapping one section back to the deep endpoints.

use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutOptions, Point, PortSide, Rect, Size};
use elk_graph::{EdgeEndpoint, EdgeId, ElkGraph, NodeId, PropertyValue, ShapeGeometry};

use crate::pipeline::routing::restore_declared_port_terminals;
use crate::pipeline::util::{
    dedup_points, endpoint_abs_center, endpoint_declared_abs_center, point_along_outward_normal,
};

#[derive(Clone, Copy, Debug)]
pub struct CompoundRouteRecord {
    pub original_source: EdgeEndpoint,
    pub original_target: EdgeEndpoint,
    pub routed_source: EdgeEndpoint,
    pub routed_target: EdgeEndpoint,
    pub effective_source: NodeId,
    pub effective_target: NodeId,
}

/// Map from edge id to cross-hierarchy routing metadata.
#[derive(Clone, Debug, Default)]
pub struct CompoundRoutingMap {
    pub edges: BTreeMap<EdgeId, CompoundRouteRecord>,
    pub temporary_ports: Vec<(NodeId, elk_graph::PortId)>,
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
    first_boundary_point: Point,
    last_boundary_point: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BranchGroupKey {
    endpoint_kind: u8,
    endpoint_index: usize,
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

fn place_hierarchical_port_on_boundary_at_tangent(
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
                place_hierarchical_port_on_boundary(graph, effective_source, port_id, hp_source_side);
                temporary_ports.push((effective_source, port_id));
                port_id
            })
        } else {
            let port_id = graph.add_port(effective_source, hp_source_side, port_geom);
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
                place_hierarchical_port_on_boundary(graph, effective_target, port_id, hp_target_side);
                temporary_ports.push((effective_target, port_id));
                port_id
            })
        } else {
            let port_id = graph.add_port(effective_target, hp_target_side, port_geom);
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

        map.edges.insert(
            edge_id,
            CompoundRouteRecord {
                original_source,
                original_target,
                routed_source: EdgeEndpoint::port(effective_source, hp_source),
                routed_target: EdgeEndpoint::port(effective_target, hp_target),
                effective_source,
                effective_target,
            },
        );
    }

    map.temporary_ports = temporary_ports;
    map
}

/// Postprocess: rebuild cross-hierarchy geometry from preserved boundary anchors.
pub fn postprocess_cross_hierarchy_edges(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
    warnings: &mut Vec<String>,
) {
    let debug_enabled = std::env::var_os("SPEC42_ELK_DEBUG").is_some();
    refresh_hierarchical_port_coordinates(graph, map);
    correct_hierarchical_port_route_terminals(graph, map);
    let mut contexts = Vec::new();
    for (edge_id, record) in &map.edges {
        let routed_source_center = endpoint_abs_center(graph, record.routed_source);
        let routed_target_center = endpoint_abs_center(graph, record.routed_target);
        let source_center = endpoint_declared_abs_center(graph, record.original_source);
        let target_center = endpoint_declared_abs_center(graph, record.original_target);

        let Some(mut routed_points) = flatten_edge_points(graph, *edge_id) else {
            let edge = &mut graph.edges[edge_id.index()];
            if let Some(first) = edge.sources.first_mut() {
                *first = record.original_source;
            }
            if let Some(first) = edge.targets.first_mut() {
                *first = record.original_target;
            }
            continue;
        };

        if should_reverse_points(&routed_points, routed_source_center, routed_target_center) {
            routed_points.reverse();
        }

        let source_side = record
            .routed_source
            .port
            .map(|port| graph.ports[port.index()].side)
            .or_else(|| dominant_side_toward(graph, record.effective_source, routed_target_center));
        let target_side = record
            .routed_target
            .port
            .map(|port| graph.ports[port.index()].side)
            .or_else(|| dominant_side_toward(graph, record.effective_target, routed_source_center));

        let Some(first_boundary_point) = routed_points.first().copied() else {
            continue;
        };
        let Some(last_boundary_point) = routed_points.last().copied() else {
            continue;
        };

        contexts.push(RebuildContext {
            edge_id: *edge_id,
            record: *record,
            source_center,
            target_center,
            routed_source_center,
            routed_target_center,
            source_side,
            target_side,
            first_boundary_point,
            last_boundary_point,
        });
    }

    let source_shared_splits = assign_shared_branch_splits(
        graph,
        contexts.iter().map(|ctx| {
            (
                ctx.edge_id,
                ctx.record.original_source,
                ctx.record.effective_source,
                ctx.source_side,
                ctx.first_boundary_point,
            )
        }),
    );
    let target_shared_splits = assign_shared_branch_splits(
        graph,
        contexts.iter().map(|ctx| {
            (
                ctx.edge_id,
                ctx.record.original_target,
                ctx.record.effective_target,
                ctx.target_side,
                ctx.last_boundary_point,
            )
        }),
    );

    for ctx in contexts {
        let edge_id = ctx.edge_id;
        let record = &ctx.record;
        let source_center = ctx.source_center;
        let target_center = ctx.target_center;
        let mut routed_points = flatten_edge_points(graph, edge_id).unwrap_or_default();
        if routed_points.is_empty() {
            continue;
        }
        if should_reverse_points(&routed_points, ctx.routed_source_center, ctx.routed_target_center) {
            routed_points.reverse();
        }

        let mut rebuilt_points = build_endpoint_branch(
            graph,
            record.original_source,
            record.effective_source,
            routed_points[0],
            ctx.source_side,
            source_shared_splits.get(&edge_id).copied().flatten(),
        );
        append_points(&mut rebuilt_points, routed_points.iter().copied().skip(1));
        let target_branch = build_endpoint_branch(
            graph,
            record.original_target,
            record.effective_target,
            ctx.last_boundary_point,
            ctx.target_side,
            target_shared_splits.get(&edge_id).copied().flatten(),
        );
        let target_branch_reversed = target_branch.into_iter().rev().skip(1).collect::<Vec<_>>();
        append_points(&mut rebuilt_points, target_branch_reversed);

        let mut legacy_points = routed_points;
        if let Some(first) = legacy_points.first_mut() {
            *first = source_center;
        }
        if let Some(last) = legacy_points.last_mut() {
            *last = target_center;
        }

        let start_side = record.original_source.port.map(|port| graph.ports[port.index()].side);
        let end_side = record.original_target.port.map(|port| graph.ports[port.index()].side);
        let rebuilt_points = normalize_candidate_polyline(rebuilt_points, start_side, end_side);
        let legacy_points = normalize_candidate_polyline(legacy_points, start_side, end_side);
        let (points, decision) =
            choose_best_candidate(graph, record, legacy_points, rebuilt_points);

        if points.len() < 2 {
            continue;
        }

        if debug_enabled {
            warnings.push(format!(
                "elk-layered compound: edge={:?} decision={} legacy={} rebuilt={}",
                edge_id,
                decision.reason,
                format_polyline(&decision.legacy_points),
                format_polyline(&decision.rebuilt_points)
            ));
            warnings.push(format!(
                "elk-layered compound: edge={:?} legacy_bends={} rebuilt_bends={} legacy_inside_start={} rebuilt_inside_start={} legacy_inside_end={} rebuilt_inside_end={} legacy_endpoint_intrusions={} rebuilt_endpoint_intrusions={} legacy_sibling_intrusions={} rebuilt_sibling_intrusions={}",
                edge_id,
                decision.legacy_bends,
                decision.rebuilt_bends,
                decision.legacy_inside_start,
                decision.rebuilt_inside_start,
                decision.legacy_inside_end,
                decision.rebuilt_inside_end,
                decision.legacy_endpoint_intrusions,
                decision.rebuilt_endpoint_intrusions,
                decision.legacy_sibling_intrusions,
                decision.rebuilt_sibling_intrusions,
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
        if !polyline_is_orthogonal(&points) {
            orthogonalize_edge_sections_with_sides(
                graph,
                edge_id,
                start_side,
                end_side,
            );
        }
        restore_declared_port_terminals(graph, edge_id);
    }
}

fn refresh_hierarchical_port_coordinates(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    let mut tangent_samples: BTreeMap<elk_graph::PortId, (PortSide, NodeId, f32, usize)> =
        BTreeMap::new();

    for record in map.edges.values() {
        for (routed_endpoint, original_endpoint) in [
            (record.routed_source, record.original_source),
            (record.routed_target, record.original_target),
        ] {
            let Some(port_id) = routed_endpoint.port else {
                continue;
            };
            let side = graph.ports[port_id.index()].side;
            let sample = endpoint_abs_center(graph, original_endpoint);
            let tangent = match side {
                PortSide::North | PortSide::South => {
                    sample.x - graph.nodes[routed_endpoint.node.index()].geometry.x
                }
                PortSide::East | PortSide::West => {
                    sample.y - graph.nodes[routed_endpoint.node.index()].geometry.y
                }
            };
            let entry = tangent_samples
                .entry(port_id)
                .or_insert((side, routed_endpoint.node, 0.0, 0));
            entry.2 += tangent;
            entry.3 += 1;
        }
    }

    for (port_id, (side, node_id, tangent_sum, count)) in tangent_samples {
        if count == 0 {
            continue;
        }
        let tangent = tangent_sum / count as f32;
        place_hierarchical_port_on_boundary_at_tangent(graph, node_id, port_id, side, tangent);
    }
}

fn correct_hierarchical_port_route_terminals(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    for (edge_id, record) in &map.edges {
        let section_ids = graph.edges[edge_id.index()].sections.clone();
        if section_ids.is_empty() {
            continue;
        }

        if let Some(port_id) = record.routed_source.port {
            let anchor = endpoint_abs_center(graph, record.routed_source);
            if let Some(first_section_id) = section_ids.first().copied() {
                let section = &mut graph.edge_sections[first_section_id.index()];
                section.start = anchor;
                if let Some(first_bend) = section.bend_points.first_mut() {
                    match graph.ports[port_id.index()].side {
                        PortSide::East | PortSide::West => first_bend.y = anchor.y,
                        PortSide::North | PortSide::South => first_bend.x = anchor.x,
                    }
                }
            }
        }

        if let Some(port_id) = record.routed_target.port {
            let anchor = endpoint_abs_center(graph, record.routed_target);
            if let Some(last_section_id) = section_ids.last().copied() {
                let section = &mut graph.edge_sections[last_section_id.index()];
                section.end = anchor;
                if let Some(last_bend) = section.bend_points.last_mut() {
                    match graph.ports[port_id.index()].side {
                        PortSide::East | PortSide::West => last_bend.y = anchor.y,
                        PortSide::North | PortSide::South => last_bend.x = anchor.x,
                    }
                }
            }
        }
    }
    hide_temporary_hierarchical_ports(graph, map);
}

fn hide_temporary_hierarchical_ports(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    for (_, port_id) in &map.temporary_ports {
        if let Some(port) = graph.ports.get_mut(port_id.index()) {
            port.properties
                .insert("spec42.temporary_hierarchical_port", PropertyValue::Bool(true));
        }
    }
}

fn normalize_candidate_polyline(
    points: Vec<Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<Point> {
    let mut points = simplify_polyline(points);
    if points.len() < 2 {
        return points;
    }
    if !polyline_is_orthogonal(&points) {
        points = orthogonalize_polyline(points, start_side, end_side);
    }
    ensure_terminal_normals(points, start_side, end_side)
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
}

fn choose_best_candidate(
    graph: &ElkGraph,
    record: &CompoundRouteRecord,
    legacy_points: Vec<Point>,
    rebuilt_points: Vec<Point>,
) -> (Vec<Point>, CandidateDecision) {
    const REBUILT_BEND_HARD_CAP: usize = 8;
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
            },
        )
    };
    if rebuilt_points.len() < 2 {
        return decision("rebuilt-invalid", legacy_points.clone(), legacy_points, rebuilt_points);
    }
    if legacy_points.len() < 2 {
        return decision("legacy-invalid", rebuilt_points.clone(), legacy_points, rebuilt_points);
    }
    let rebuilt_within_hard_cap = rebuilt_bends <= REBUILT_BEND_HARD_CAP;
    if (legacy_inside_start && !rebuilt_inside_start) || (legacy_inside_end && !rebuilt_inside_end) {
        if (rebuilt_within_hard_cap || legacy_sibling_intrusions > rebuilt_sibling_intrusions)
            && rebuilt_endpoint_intrusions <= legacy_endpoint_intrusions
        {
            return decision(
                "rebuilt-fixes-inside-approach",
                rebuilt_points.clone(),
                legacy_points,
                rebuilt_points,
            );
        }
        return decision(
            "legacy-preserves-bend-cap",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_endpoint_intrusions != legacy_endpoint_intrusions {
        if rebuilt_endpoint_intrusions < legacy_endpoint_intrusions && rebuilt_within_hard_cap {
            return decision(
                "rebuilt-reduces-endpoint-node-intrusions",
                rebuilt_points.clone(),
                legacy_points,
                rebuilt_points,
            );
        }
        return decision(
            "legacy-avoids-endpoint-node-intrusions",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_sibling_intrusions != legacy_sibling_intrusions {
        if rebuilt_sibling_intrusions < legacy_sibling_intrusions && rebuilt_within_hard_cap {
            return decision(
                "rebuilt-avoids-sibling-obstacles",
                rebuilt_points.clone(),
                legacy_points,
                rebuilt_points,
            );
        }
        return decision(
            "legacy-preserves-bend-cap",
            legacy_points.clone(),
            legacy_points,
            rebuilt_points,
        );
    }
    if rebuilt_bends <= legacy_bends + 1 {
        decision("rebuilt-within-bend-budget", rebuilt_points.clone(), legacy_points, rebuilt_points)
    } else {
        decision("legacy-lower-bend-budget", legacy_points.clone(), legacy_points, rebuilt_points)
    }
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

fn assign_shared_branch_splits<I>(
    graph: &ElkGraph,
    entries: I,
) -> BTreeMap<EdgeId, Option<f32>>
where
    I: IntoIterator<Item = (EdgeId, EdgeEndpoint, NodeId, Option<PortSide>, Point)>,
{
    let mut grouped: BTreeMap<BranchGroupKey, Vec<(EdgeId, EdgeEndpoint, Point)>> = BTreeMap::new();
    for (edge_id, endpoint, outer_node, boundary_side, boundary_point) in entries {
        let Some(key) = endpoint_group_key(endpoint, outer_node, boundary_side) else {
            continue;
        };
        grouped.entry(key).or_default().push((edge_id, endpoint, boundary_point));
    }

    let mut result = BTreeMap::new();
    for (_key, entries) in grouped {
        if entries.len() < 2 {
            continue;
        }
        let Some((_, sample_endpoint, _)) = entries.first().copied() else {
            continue;
        };
        let Some((port_side, outward)) = branch_outward_point(graph, sample_endpoint) else {
            continue;
        };
        let origin_axis = branch_axis_value(outward, port_side);
        let mut values = entries
            .iter()
            .map(|(_, _, boundary_point)| branch_axis_value(*boundary_point, port_side))
            .collect::<Vec<_>>();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let all_non_negative = values.iter().all(|value| *value >= origin_axis + 1e-3);
        let all_non_positive = values.iter().all(|value| *value <= origin_axis - 1e-3);
        let split = if all_non_negative {
            values.first().copied()
        } else if all_non_positive {
            values.last().copied()
        } else {
            None
        };
        for (edge_id, _, _) in entries {
            result.insert(edge_id, split);
        }
    }
    result
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
            PortSide::North | PortSide::South => {
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
            edges: BTreeMap::from([(
                edge_id,
                CompoundRouteRecord {
                    original_source: EdgeEndpoint::port(inner, inner_port),
                    original_target: EdgeEndpoint::port(inner, inner_port),
                    routed_source: EdgeEndpoint::port(outer, routed_port),
                    routed_target: EdgeEndpoint::port(outer, routed_port),
                    effective_source: outer,
                    effective_target: outer,
                },
            )]),
            temporary_ports: Vec::new(),
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
            edges: BTreeMap::from([(
                edge_id,
                CompoundRouteRecord {
                    original_source: EdgeEndpoint::node(outer),
                    original_target: EdgeEndpoint::node(outer),
                    routed_source: EdgeEndpoint::port(outer, routed_port),
                    routed_target: EdgeEndpoint::node(outer),
                    effective_source: outer,
                    effective_target: outer,
                },
            )]),
            temporary_ports: Vec::new(),
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
            edges: BTreeMap::new(),
            temporary_ports: vec![(node, temp_a), (node, temp_b)],
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
