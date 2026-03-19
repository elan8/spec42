//! Helpers to assert that routed edge paths do not cross obstacle (node) interiors.

use elk_core::{Point, Rect, Size};
use elk_graph::{ElkGraph, NodeId};

const DEFAULT_TOL: f32 = 1e-4;

/// Returns true if the open segment (a, b) intersects the interior of rect (with tolerance).
#[must_use]
pub fn segment_intersects_rect_interior(a: Point, b: Point, r: &Rect, tol: f32) -> bool {
    let min_x = r.origin.x + tol;
    let max_x = r.origin.x + r.size.width - tol;
    let min_y = r.origin.y + tol;
    let max_y = r.origin.y + r.size.height - tol;
    if (a.x > min_x && a.x < max_x && a.y > min_y && a.y < max_y)
        || (b.x > min_x && b.x < max_x && b.y > min_y && b.y < max_y)
    {
        return true;
    }
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;
    let edges = [(dx, min_x - a.x), (-dx, a.x - max_x), (dy, min_y - a.y), (-dy, a.y - max_y)];
    for (denom, num) in edges {
        if denom.abs() < tol {
            if num > 0.0 {
                return false;
            }
            continue;
        }
        let t = num / denom;
        if denom > 0.0 {
            t1 = t1.min(t);
        } else {
            t0 = t0.max(t);
        }
    }
    t0 <= t1 + tol
}

fn node_rect(graph: &ElkGraph, n: NodeId) -> Rect {
    let g = &graph.nodes[n.index()].geometry;
    Rect::new(Point::new(g.x, g.y), Size::new(g.width.max(0.0), g.height.max(0.0)))
}

/// Asserts that every edge section polyline (start → bends → end) does not intersect
/// the interior of any node rect except the edge's source and target nodes.
/// Uses `tolerance` for interior boundary and segment-rect tests.
pub fn assert_routed_paths_avoid_obstacles(graph: &ElkGraph, tolerance: f32) {
    let tol = tolerance.max(DEFAULT_TOL);
    for edge in &graph.edges {
        let source_node = edge.sources.first().map(|ep| ep.node);
        let target_node = edge.targets.first().map(|ep| ep.node);
        let source_target: Vec<NodeId> = source_node
            .into_iter()
            .chain(target_node)
            .filter(|n| *n != graph.root)
            .collect();

        for &sid in &edge.sections {
            let section = &graph.edge_sections[sid.index()];
            let points: Vec<Point> = std::iter::once(section.start)
                .chain(section.bend_points.iter().copied())
                .chain(std::iter::once(section.end))
                .collect();

            for seg in points.windows(2) {
                let (a, b) = (seg[0], seg[1]);
                for node in &graph.nodes {
                    if node.id == graph.root {
                        continue;
                    }
                    if source_target.contains(&node.id) {
                        continue;
                    }
                    // Crossing a source/target container boundary is valid for
                    // cross-hierarchy routes; skip ancestor compounds of endpoints.
                    if source_target.iter().any(|ep| graph.is_ancestor(node.id, *ep)) {
                        continue;
                    }
                    let r = node_rect(graph, node.id);
                    assert!(
                        !segment_intersects_rect_interior(a, b, &r, tol),
                        "edge {:?} segment ({}, {}) -> ({}, {}) intersects node {:?} rect {:?}",
                        edge.id,
                        a.x,
                        a.y,
                        b.x,
                        b.y,
                        node.id,
                        (r.origin.x, r.origin.y, r.size.width, r.size.height)
                    );
                }
            }
        }
    }
}
