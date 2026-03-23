#![forbid(unsafe_code)]
#![doc = "Obstacle-avoiding edge routing (ELK libavoid baseline, native Rust)."]

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, PortSide, Rect, Size};
use std::collections::BTreeSet;

use elk_graph::{ElkGraph, EdgeId, EdgeEndpoint, NodeId};
use elk_alg_common::orthogonal::{point_along_tangent, sanitize_orthogonal_path};

mod router;

use router::{Obstacle, RoutingFailure, route, route_with_debug};

const DEFAULT_CLEARANCE: f32 = 4.0;
const DEFAULT_SEGMENT_PENALTY: f32 = 1.0;
const DEFAULT_BEND_PENALTY: f32 = 6.0;
const TERMINAL_NORMAL_OFFSET: f32 = 16.0;
const TERMINAL_TANGENT_SPACING: f32 = 18.0;

#[derive(Clone, Copy, Debug)]
struct RoutePlan {
    actual_start_abs: Point,
    actual_end_abs: Point,
    start_lead_abs: Point,
    end_lead_abs: Point,
    route_start_local: Point,
    route_end_local: Point,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
}

fn read_penalty_f32(
    by_key: &std::collections::BTreeMap<String, &elk_graph::PropertyValue>,
    keys: &[&str],
    default: f32,
    min: f32,
) -> f32 {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            if let Some(x) = elk_alg_common::options::value_to_f32(v) {
                return x.max(min);
            }
        }
    }
    default.max(min)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LibavoidLayoutEngine;

impl LibavoidLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Route edges only; does not move nodes. Use after a layout that set node positions.
    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        _options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;

        let by_key = elk_alg_common::options::casefold_map(&graph.properties);
        let clearance = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.clearance",
                "org.eclipse.elk.libavoid.clearance",
            ],
            DEFAULT_CLEARANCE,
            0.0,
        );
        let segment_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.segmentpenalty",
                "org.eclipse.elk.libavoid.segmentPenalty",
                "org.eclipse.elk.libavoid.segmentpenalty",
            ],
            DEFAULT_SEGMENT_PENALTY,
            1e-6,
        );
        let bend_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.bendpenalty",
                "org.eclipse.elk.libavoid.bendPenalty",
                "org.eclipse.elk.libavoid.bendpenalty",
            ],
            DEFAULT_BEND_PENALTY,
            0.0,
        );

        let mut edge_ids: Vec<EdgeId> = graph.edges.iter().map(|e| e.id).collect();
        edge_ids.sort_by_key(|e| e.index());

        for edge_id in edge_ids {
            let plan = build_route_plan(graph, edge_id, Point::new(0.0, 0.0), clearance);
            let excluded = excluded_obstacle_nodes(graph, edge_id, plan.source_side, plan.target_side);
            let obstacles = collect_obstacles(
                graph,
                clearance,
                &excluded,
                Point::new(0.0, 0.0),
                None,
            );
            let path = route(plan.route_start_local, plan.route_end_local, &obstacles, segment_penalty, bend_penalty)
                .map_err(|e| routing_failure_to_layout_error(edge_id, e))?;
            let path = finalize_route_path(
                path,
                plan.actual_start_abs,
                plan.actual_end_abs,
                plan.start_lead_abs,
                plan.end_lead_abs,
                plan.route_start_local,
                plan.route_end_local,
                Point::new(0.0, 0.0),
            )
                .map_err(|m| LayoutError::Routing(format!("edge {:?}: {m}", edge_id)))?;
            let bends = if path.len() >= 2 {
                path[1..path.len() - 1].to_vec()
            } else {
                Vec::new()
            };
            let start_pt = path.first().copied().unwrap_or(plan.actual_start_abs);
            let end_pt = path.last().copied().unwrap_or(plan.actual_end_abs);
            graph.edges[edge_id.index()].sections.clear();
            let _ = graph.add_edge_section(edge_id, start_pt, bends, end_pt);
        }

        Ok(LayoutReport::default())
    }
}

pub fn layout(graph: &mut ElkGraph, options: &LayoutOptions) -> Result<LayoutReport, LayoutError> {
    LibavoidLayoutEngine::new().layout(graph, options)
}

/// Route only the given edges; does not move nodes. Reads options from `graph.properties` (root).
/// Use after a layout that set node positions. Keeps routing local to the current graph (obstacles
/// from its nodes only).
pub fn route_edges(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
) -> Result<(), LayoutError> {
    route_edges_with_diagnostics(graph, edge_ids).map(|_| ())
}

/// Same as `route_edges`, but returns structured debug lines when `SPEC42_ELK_DEBUG` is set.
pub fn route_edges_with_diagnostics(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
) -> Result<Vec<String>, LayoutError> {
    route_edges_with_diagnostics_in_scope(graph, edge_ids, Point::new(0.0, 0.0), None)
}

/// Route only `edge_ids` using a local frame origin and optional obstacle-node subset.
/// Inputs/outputs to the router are scope-local; edge sections are written back in absolute coords.
pub fn route_edges_with_diagnostics_in_scope(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
    scope_origin_abs: Point,
    scope_nodes: Option<&BTreeSet<NodeId>>,
) -> Result<Vec<String>, LayoutError> {
    graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    let root_clearance = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.clearance",
            "org.eclipse.elk.libavoid.clearance",
        ],
        DEFAULT_CLEARANCE,
        0.0,
    );
    let root_segment_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.segmentpenalty",
            "org.eclipse.elk.libavoid.segmentPenalty",
            "org.eclipse.elk.libavoid.segmentpenalty",
        ],
        DEFAULT_SEGMENT_PENALTY,
        1e-6,
    );
    let root_bend_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.bendpenalty",
            "org.eclipse.elk.libavoid.bendPenalty",
            "org.eclipse.elk.libavoid.bendpenalty",
        ],
        DEFAULT_BEND_PENALTY,
        0.0,
    );

    let mut sorted_ids: Vec<EdgeId> = edge_ids.to_vec();
    sorted_ids.sort_by_key(|e| e.index());
    let debug_enabled = std::env::var_os("SPEC42_ELK_DEBUG").is_some();
    let mut diagnostics = Vec::new();

    for edge_id in sorted_ids {
        let clearance = root_clearance;
        let segment_penalty = root_segment_penalty;
        let bend_penalty = root_bend_penalty;
        let plan = build_route_plan(graph, edge_id, scope_origin_abs, clearance);
        let excluded = excluded_obstacle_nodes(graph, edge_id, plan.source_side, plan.target_side);
        let obstacles = collect_obstacles(graph, clearance, &excluded, scope_origin_abs, scope_nodes);
        let (path, route_dbg) = if debug_enabled {
            route_with_debug(
                plan.route_start_local,
                plan.route_end_local,
                &obstacles,
                segment_penalty,
                bend_penalty,
            )
                .map_err(|e| routing_failure_to_layout_error(edge_id, e))?
        } else {
            (
                route(
                    plan.route_start_local,
                    plan.route_end_local,
                    &obstacles,
                    segment_penalty,
                    bend_penalty,
                )
                    .map_err(|e| routing_failure_to_layout_error(edge_id, e))?,
                router::RouteDebug::default(),
            )
        };
        let path = finalize_route_path(
            path,
            plan.actual_start_abs,
            plan.actual_end_abs,
            plan.start_lead_abs,
            plan.end_lead_abs,
            plan.route_start_local,
            plan.route_end_local,
            scope_origin_abs,
        )
            .map_err(|m| LayoutError::Routing(format!("edge {:?}: {m}", edge_id)))?;
        let bends = if path.len() >= 2 {
            path[1..path.len() - 1].to_vec()
        } else {
            Vec::new()
        };
        let start_pt = path.first().copied().unwrap_or(plan.actual_start_abs);
        let end_pt = path.last().copied().unwrap_or(plan.actual_end_abs);
        let bends_abs = bends;
        let start_abs_pt = start_pt;
        let end_abs_pt = end_pt;
        if debug_enabled {
            let start_contract_ok = nearly_same_point(start_abs_pt, plan.actual_start_abs, 1.0);
            let end_contract_ok = nearly_same_point(end_abs_pt, plan.actual_end_abs, 1.0);
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} obstacles={} excluded_nodes={:?} scope_origin=({:.1},{:.1}) endpoint_contract=start:{} end:{} start_abs=({:.1},{:.1})->({:.1},{:.1}) end_abs=({:.1},{:.1})->({:.1},{:.1}) candidates={} expanded={} accepted={} blocked={} found={}",
                edge_id,
                obstacles.len(),
                excluded,
                scope_origin_abs.x,
                scope_origin_abs.y,
                start_contract_ok,
                end_contract_ok,
                plan.actual_start_abs.x,
                plan.actual_start_abs.y,
                start_abs_pt.x,
                start_abs_pt.y,
                plan.actual_end_abs.x,
                plan.actual_end_abs.y,
                end_abs_pt.x,
                end_abs_pt.y,
                route_dbg.candidate_points,
                route_dbg.expanded_states,
                route_dbg.accepted_neighbors,
                route_dbg.blocked_neighbors,
                route_dbg.path_found
            ));
        }
        graph.edges[edge_id.index()].sections.clear();
        let _ = graph.add_edge_section(edge_id, start_abs_pt, bends_abs, end_abs_pt);
    }
    Ok(diagnostics)
}

fn nearly_same_point(a: Point, b: Point, eps: f32) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps
}

fn build_route_plan(
    graph: &ElkGraph,
    edge_id: EdgeId,
    scope_origin_abs: Point,
    clearance: f32,
) -> RoutePlan {
    let edge = &graph.edges[edge_id.index()];
    let source = edge.sources.first().copied();
    let target = edge.targets.first().copied();
    let other_end_abs = target
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let other_start_abs = source
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let source_side = source.map(|ep| endpoint_side_for_routing(graph, ep, other_end_abs));
    let target_side = target.map(|ep| endpoint_side_for_routing(graph, ep, other_start_abs));
    let source_slot = endpoint_slot(graph, edge_id, true);
    let target_slot = endpoint_slot(graph, edge_id, false);
    let lead = (clearance + TERMINAL_NORMAL_OFFSET).max(clearance + 1.0);
    let source_lead = lead + source_slot.abs() as f32 * (TERMINAL_TANGENT_SPACING * 0.75);
    let target_lead = lead + target_slot.abs() as f32 * (TERMINAL_TANGENT_SPACING * 0.75);
    let actual_start_abs = source
        .zip(source_side)
        .map(|(ep, side)| endpoint_anchor_for_routing(graph, ep, side, source_slot))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let actual_end_abs = target
        .zip(target_side)
        .map(|(ep, side)| endpoint_anchor_for_routing(graph, ep, side, target_slot))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let start_lead_abs = source_side
        .map(|side| point_along_outward_normal(actual_start_abs, side, source_lead))
        .unwrap_or(actual_start_abs);
    let end_lead_abs = target_side
        .map(|side| point_along_outward_normal(actual_end_abs, side, target_lead))
        .unwrap_or(actual_end_abs);
    let route_start_abs = source_side
        .map(|side| point_along_tangent(start_lead_abs, side, tangent_route_offset(source, source_slot)))
        .unwrap_or(start_lead_abs);
    let route_end_abs = target_side
        .map(|side| point_along_tangent(end_lead_abs, side, tangent_route_offset(target, target_slot)))
        .unwrap_or(end_lead_abs);
    RoutePlan {
        actual_start_abs,
        actual_end_abs,
        start_lead_abs,
        end_lead_abs,
        route_start_local: to_local(route_start_abs, scope_origin_abs),
        route_end_local: to_local(route_end_abs, scope_origin_abs),
        source_side,
        target_side,
    }
}

fn finalize_route_path(
    route_path_local: Vec<Point>,
    actual_start_abs: Point,
    actual_end_abs: Point,
    start_lead_abs: Point,
    end_lead_abs: Point,
    route_start_local: Point,
    route_end_local: Point,
    scope_origin_abs: Point,
) -> Result<Vec<Point>, String> {
    let route_start_abs = to_abs(route_start_local, scope_origin_abs);
    let route_end_abs = to_abs(route_end_local, scope_origin_abs);
    let path_abs: Vec<Point> = route_path_local
        .into_iter()
        .map(|point| to_abs(point, scope_origin_abs))
        .collect();
    sanitize_orthogonal_path(
        path_abs,
        actual_start_abs,
        actual_end_abs,
        start_lead_abs,
        end_lead_abs,
        route_start_abs,
        route_end_abs,
    )
}

fn endpoint_slot(graph: &ElkGraph, edge_id: EdgeId, is_source: bool) -> i32 {
    let key = elk_graph::PropertyKey::from(if is_source {
        "spec42.endpoint.slot.source"
    } else {
        "spec42.endpoint.slot.target"
    });
    graph.edges[edge_id.index()]
        .properties
        .get(&key)
        .and_then(|value| value.as_i64())
        .unwrap_or(0) as i32
}

fn routing_failure_to_layout_error(edge_id: EdgeId, failure: RoutingFailure) -> LayoutError {
    let reason = match failure {
        RoutingFailure::NoCandidatePoints => "no_candidate_points",
        RoutingFailure::DegenerateEndpoints => "degenerate_endpoints",
        RoutingFailure::NoRouteFound => "no_route_found",
    };
    LayoutError::Routing(format!("edge {:?}: libavoid route failed ({reason})", edge_id))
}

fn node_rect(graph: &ElkGraph, n: NodeId) -> Rect {
    let g = &graph.nodes[n.index()].geometry;
    let o = node_abs_origin(graph, n);
    Rect::new(
        Point::new(o.x, o.y),
        Size::new(g.width.max(0.0), g.height.max(0.0)),
    )
}

fn node_abs_origin(graph: &ElkGraph, node_id: NodeId) -> Point {
    let n = &graph.nodes[node_id.index()];
    match n.parent {
        Some(parent) if parent != graph.root => {
            let p = node_abs_origin(graph, parent);
            Point::new(p.x + n.geometry.x, p.y + n.geometry.y)
        }
        _ => Point::new(n.geometry.x, n.geometry.y),
    }
}

fn collect_obstacles(
    graph: &ElkGraph,
    clearance: f32,
    excluded_nodes: &[NodeId],
    scope_origin_abs: Point,
    scope_nodes: Option<&BTreeSet<NodeId>>,
) -> Vec<Obstacle> {
    let mut out = Vec::new();
    for node in &graph.nodes {
        if node.id == graph.root {
            continue;
        }
        if scope_nodes.is_some_and(|s| !s.contains(&node.id)) {
            continue;
        }
        if excluded_nodes.contains(&node.id) {
            continue;
        }
        let r_abs = node_rect(graph, node.id);
        let r = Rect::new(
            to_local(r_abs.origin, scope_origin_abs),
            r_abs.size,
        );
        let expanded = Rect::new(
            Point::new(r.origin.x - clearance, r.origin.y - clearance),
            Size::new(r.size.width + 2.0 * clearance, r.size.height + 2.0 * clearance),
        );
        out.push(Obstacle { rect: expanded });
    }
    out.sort_by(|a, b| {
        a.rect.origin.x
            .partial_cmp(&b.rect.origin.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.rect.origin.y.partial_cmp(&b.rect.origin.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    out
}

fn excluded_obstacle_nodes(
    graph: &ElkGraph,
    edge_id: EdgeId,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
) -> Vec<NodeId> {
    let edge = &graph.edges[edge_id.index()];
    let mut nodes = Vec::new();
    if let Some(source) = edge.sources.first() {
        if source.node != graph.root && source_side.is_none() && !nodes.contains(&source.node) {
            nodes.push(source.node);
        }
    }
    if let Some(target) = edge.targets.first() {
        if target.node != graph.root && target_side.is_none() && !nodes.contains(&target.node) {
            nodes.push(target.node);
        }
    }
    nodes
}

fn endpoint_center(graph: &ElkGraph, ep: EdgeEndpoint) -> Point {
    if let Some(port_id) = ep.port {
        let p = &graph.ports[port_id.index()];
        let o = node_abs_origin(graph, p.node);
        Point::new(
            o.x + p.geometry.x + p.geometry.width / 2.0,
            o.y + p.geometry.y + p.geometry.height / 2.0,
        )
    } else {
        let n = &graph.nodes[ep.node.index()];
        let o = node_abs_origin(graph, ep.node);
        Point::new(
            o.x + n.geometry.width / 2.0,
            o.y + n.geometry.height / 2.0,
        )
    }
}

fn endpoint_port_side(graph: &ElkGraph, ep: EdgeEndpoint) -> Option<PortSide> {
    ep.port.map(|port_id| graph.ports[port_id.index()].side)
}

fn endpoint_side_for_routing(graph: &ElkGraph, endpoint: EdgeEndpoint, toward: Point) -> PortSide {
    endpoint_port_side(graph, endpoint).unwrap_or_else(|| infer_node_side_for_target(graph, endpoint.node, toward))
}

fn infer_node_side_for_target(graph: &ElkGraph, node_id: NodeId, toward: Point) -> PortSide {
    let center = endpoint_center(graph, EdgeEndpoint::node(node_id));
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;
    if dx.abs() >= dy.abs() {
        if dx >= 0.0 {
            PortSide::East
        } else {
            PortSide::West
        }
    } else if dy >= 0.0 {
        PortSide::South
    } else {
        PortSide::North
    }
}

fn endpoint_anchor_for_routing(
    graph: &ElkGraph,
    endpoint: EdgeEndpoint,
    side: PortSide,
    slot: i32,
) -> Point {
    if let Some(port_id) = endpoint.port {
        let center = endpoint_center(graph, endpoint);
        let port = &graph.ports[port_id.index()];
        let node = &graph.nodes[port.node.index()];
        let origin = node_abs_origin(graph, port.node);
        let tangent = slot as f32 * TERMINAL_TANGENT_SPACING;
        let margin = 12.0;
        let min_x = origin.x + margin;
        let max_x = (origin.x + node.geometry.width - margin).max(min_x);
        let min_y = origin.y + margin;
        let max_y = (origin.y + node.geometry.height - margin).max(min_y);

        return match side {
            PortSide::North | PortSide::South => Point::new((center.x + tangent).clamp(min_x, max_x), center.y),
            PortSide::East | PortSide::West => Point::new(center.x, (center.y + tangent).clamp(min_y, max_y)),
        };
    }

    let node = &graph.nodes[endpoint.node.index()];
    let origin = node_abs_origin(graph, endpoint.node);
    let center = Point::new(origin.x + node.geometry.width / 2.0, origin.y + node.geometry.height / 2.0);
    let tangent = slot as f32 * TERMINAL_TANGENT_SPACING;
    let margin = 12.0;
    let min_x = origin.x + margin;
    let max_x = (origin.x + node.geometry.width - margin).max(min_x);
    let min_y = origin.y + margin;
    let max_y = (origin.y + node.geometry.height - margin).max(min_y);

    match side {
        PortSide::North => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y),
        PortSide::South => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y + node.geometry.height),
        PortSide::East => Point::new(origin.x + node.geometry.width, (center.y + tangent).clamp(min_y, max_y)),
        PortSide::West => Point::new(origin.x, (center.y + tangent).clamp(min_y, max_y)),
    }
}

fn tangent_route_offset(endpoint: Option<EdgeEndpoint>, slot: i32) -> f32 {
    let _ = (endpoint, slot);
    0.0
}

fn point_along_outward_normal(center: Point, side: PortSide, delta: f32) -> Point {
    match side {
        PortSide::North => Point::new(center.x, center.y - delta),
        PortSide::South => Point::new(center.x, center.y + delta),
        PortSide::East => Point::new(center.x + delta, center.y),
        PortSide::West => Point::new(center.x - delta, center.y),
    }
}

fn to_local(abs: Point, origin_abs: Point) -> Point {
    Point::new(abs.x - origin_abs.x, abs.y - origin_abs.y)
}

fn to_abs(local: Point, origin_abs: Point) -> Point {
    Point::new(local.x + origin_abs.x, local.y + origin_abs.y)
}
