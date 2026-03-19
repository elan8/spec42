#![forbid(unsafe_code)]
#![doc = "Obstacle-avoiding edge routing (ELK libavoid baseline, native Rust)."]

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, Rect, Size};
use std::collections::BTreeSet;

use elk_graph::{ElkGraph, EdgeId, EdgeEndpoint, NodeId};

mod router;

use router::{Obstacle, route, route_with_debug};

const DEFAULT_CLEARANCE: f32 = 4.0;
const DEFAULT_SEGMENT_PENALTY: f32 = 1.0;
const DEFAULT_BEND_PENALTY: f32 = 6.0;

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
            let (start, end) = endpoints_center(graph, edge_id);
            let excluded = edge_endpoint_nodes(graph, edge_id);
            let obstacles = collect_obstacles(
                graph,
                clearance,
                &excluded,
                Point::new(0.0, 0.0),
                None,
            );
            let path = route(start, end, &obstacles, segment_penalty, bend_penalty);
            let bends = if path.len() >= 2 {
                path[1..path.len() - 1].to_vec()
            } else {
                Vec::new()
            };
            let start_pt = path.first().copied().unwrap_or(start);
            let end_pt = path.last().copied().unwrap_or(end);
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
        let (start_abs, end_abs) = endpoints_center(graph, edge_id);
        let start = to_local(start_abs, scope_origin_abs);
        let end = to_local(end_abs, scope_origin_abs);
        let excluded = edge_endpoint_nodes(graph, edge_id);
        let obstacles = collect_obstacles(graph, clearance, &excluded, scope_origin_abs, scope_nodes);
        let (path, route_dbg) = if debug_enabled {
            route_with_debug(start, end, &obstacles, segment_penalty, bend_penalty)
        } else {
            (
                route(start, end, &obstacles, segment_penalty, bend_penalty),
                router::RouteDebug::default(),
            )
        };
        let bends = if path.len() >= 2 {
            path[1..path.len() - 1].to_vec()
        } else {
            Vec::new()
        };
        let start_pt = path.first().copied().unwrap_or(start);
        let end_pt = path.last().copied().unwrap_or(end);
        let bends_abs: Vec<Point> = bends.into_iter().map(|p| to_abs(p, scope_origin_abs)).collect();
        let start_abs_pt = to_abs(start_pt, scope_origin_abs);
        let end_abs_pt = to_abs(end_pt, scope_origin_abs);
        if debug_enabled {
            let start_contract_ok = nearly_same_point(start_abs_pt, start_abs, 1.0);
            let end_contract_ok = nearly_same_point(end_abs_pt, end_abs, 1.0);
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} obstacles={} excluded_nodes={:?} scope_origin=({:.1},{:.1}) endpoint_contract=start:{} end:{} start_abs=({:.1},{:.1})->({:.1},{:.1}) end_abs=({:.1},{:.1})->({:.1},{:.1}) candidates={} expanded={} accepted={} blocked={} found={}",
                edge_id,
                obstacles.len(),
                excluded,
                scope_origin_abs.x,
                scope_origin_abs.y,
                start_contract_ok,
                end_contract_ok,
                start_abs.x,
                start_abs.y,
                start_abs_pt.x,
                start_abs_pt.y,
                end_abs.x,
                end_abs.y,
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

fn edge_endpoint_nodes(graph: &ElkGraph, edge_id: EdgeId) -> Vec<NodeId> {
    let edge = &graph.edges[edge_id.index()];
    let mut nodes = Vec::new();
    if let Some(source) = edge.sources.first() {
        nodes.push(source.node);
    }
    if let Some(target) = edge.targets.first() {
        if !nodes.contains(&target.node) {
            nodes.push(target.node);
        }
    }
    nodes
}

fn endpoint_center(graph: &ElkGraph, ep: EdgeEndpoint) -> Point {
    if let Some(port_id) = ep.port {
        let p = &graph.ports[port_id.index()];
        Point::new(
            p.geometry.x + p.geometry.width / 2.0,
            p.geometry.y + p.geometry.height / 2.0,
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

fn endpoints_center(graph: &ElkGraph, edge_id: EdgeId) -> (Point, Point) {
    let e = &graph.edges[edge_id.index()];
    let start = e
        .sources
        .first()
        .copied()
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let end = e
        .targets
        .first()
        .copied()
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    (start, end)
}

fn to_local(abs: Point, origin_abs: Point) -> Point {
    Point::new(abs.x - origin_abs.x, abs.y - origin_abs.y)
}

fn to_abs(local: Point, origin_abs: Point) -> Point {
    Point::new(local.x + origin_abs.x, local.y + origin_abs.y)
}
