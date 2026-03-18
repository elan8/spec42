#![forbid(unsafe_code)]
#![doc = "Obstacle-avoiding edge routing (ELK libavoid baseline, native Rust)."]

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, Rect, Size};
use elk_graph::{ElkGraph, EdgeId, EdgeEndpoint, NodeId};

mod router;

use router::{Obstacle, route};

const DEFAULT_CLEARANCE: f32 = 4.0;
const DEFAULT_SEGMENT_PENALTY: f32 = 1.0;
const DEFAULT_BEND_PENALTY: f32 = 0.0;

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

        let meta = elk_meta::default_registry();
        let by_key = elk_alg_common::options::casefold_map(&graph.properties);
        let clearance = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.clearance")
            .and_then(elk_alg_common::options::value_to_f32)
            .unwrap_or(DEFAULT_CLEARANCE)
            .max(0.0);
        let segment_penalty = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.segmentpenalty")
            .and_then(elk_alg_common::options::value_to_f32)
            .unwrap_or(DEFAULT_SEGMENT_PENALTY)
            .max(1e-6);
        let bend_penalty = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.bendpenalty")
            .and_then(elk_alg_common::options::value_to_f32)
            .unwrap_or(DEFAULT_BEND_PENALTY)
            .max(0.0);

        let obstacles = collect_obstacles(graph, clearance);
        let mut edge_ids: Vec<EdgeId> = graph.edges.iter().map(|e| e.id).collect();
        edge_ids.sort_by_key(|e| e.index());

        for edge_id in edge_ids {
            let (start, end) = endpoints_center(graph, edge_id);
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
    graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;
    let meta = elk_meta::default_registry();
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    let clearance = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.clearance")
        .and_then(elk_alg_common::options::value_to_f32)
        .unwrap_or(DEFAULT_CLEARANCE)
        .max(0.0);
    let segment_penalty = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.segmentpenalty")
        .and_then(elk_alg_common::options::value_to_f32)
        .unwrap_or(DEFAULT_SEGMENT_PENALTY)
        .max(1e-6);
    let bend_penalty = elk_alg_common::options::find_option(&meta, &by_key, "elk.libavoid.bendpenalty")
        .and_then(elk_alg_common::options::value_to_f32)
        .unwrap_or(DEFAULT_BEND_PENALTY)
        .max(0.0);

    let obstacles = collect_obstacles(graph, clearance);
    let mut sorted_ids: Vec<EdgeId> = edge_ids.to_vec();
    sorted_ids.sort_by_key(|e| e.index());

    for edge_id in sorted_ids {
        let (start, end) = endpoints_center(graph, edge_id);
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
    Ok(())
}

fn node_rect(graph: &ElkGraph, n: NodeId) -> Rect {
    let g = &graph.nodes[n.index()].geometry;
    Rect::new(Point::new(g.x, g.y), Size::new(g.width.max(0.0), g.height.max(0.0)))
}

fn collect_obstacles(graph: &ElkGraph, clearance: f32) -> Vec<Obstacle> {
    let mut out = Vec::new();
    for node in &graph.nodes {
        if node.id == graph.root {
            continue;
        }
        let r = node_rect(graph, node.id);
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

fn endpoint_center(graph: &ElkGraph, ep: EdgeEndpoint) -> Point {
    if let Some(port_id) = ep.port {
        let p = &graph.ports[port_id.index()];
        Point::new(
            p.geometry.x + p.geometry.width / 2.0,
            p.geometry.y + p.geometry.height / 2.0,
        )
    } else {
        let n = &graph.nodes[ep.node.index()];
        Point::new(
            n.geometry.x + n.geometry.width / 2.0,
            n.geometry.y + n.geometry.height / 2.0,
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
