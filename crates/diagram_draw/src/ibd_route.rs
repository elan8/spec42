//! Port of `render/ibd-route.ts`, minus `lcaOffsetForNodes` (layout-only, never reached from
//! drawing -- `edge.layout.lcaOffset` already carries its result by the time drawing runs).

use crate::types::{EdgeSection, LaidOutEdge, Point};

/// Port of `pruneRoutePoints`: dedupes adjacent points and collapses runs of 3+ collinear
/// (axis-aligned) points down to their endpoints.
pub fn prune_route_points(points: &[Point]) -> Vec<Point> {
    let mut pruned: Vec<Point> = Vec::new();
    for &point in points {
        if let Some(last) = pruned.last() {
            if (last.x - point.x).abs() < 1e-6 && (last.y - point.y).abs() < 1e-6 {
                continue;
            }
        }
        pruned.push(point);
        while pruned.len() >= 3 {
            let a = pruned[pruned.len() - 3];
            let b = pruned[pruned.len() - 2];
            let c = pruned[pruned.len() - 1];
            let same_x = (a.x - b.x).abs() < 1e-6 && (b.x - c.x).abs() < 1e-6;
            let same_y = (a.y - b.y).abs() < 1e-6 && (b.y - c.y).abs() < 1e-6;
            if !same_x && !same_y {
                break;
            }
            pruned.remove(pruned.len() - 2);
        }
    }
    pruned
}

/// Port of `pointsFromElkSections`.
pub fn points_from_elk_sections(sections: &[EdgeSection], offset: Point) -> Vec<Point> {
    let mut points = Vec::new();
    for section in sections {
        if let Some(start) = section.start_point {
            points.push(Point {
                x: start.x + offset.x,
                y: start.y + offset.y,
            });
        }
        for bend in &section.bend_points {
            points.push(Point {
                x: bend.x + offset.x,
                y: bend.y + offset.y,
            });
        }
        if let Some(end) = section.end_point {
            points.push(Point {
                x: end.x + offset.x,
                y: end.y + offset.y,
            });
        }
    }
    prune_route_points(&points)
}

/// Port of `uniqueOffsets`: dedupes by the same `toFixed(3)` string key the TS source uses.
pub fn unique_offsets(offsets: Vec<Point>) -> Vec<Point> {
    let mut seen = std::collections::HashSet::new();
    let mut unique = Vec::new();
    for offset in offsets {
        let key = (format!("{:.3}", offset.x), format!("{:.3}", offset.y));
        if seen.insert(key) {
            unique.push(offset);
        }
    }
    unique
}

/// Port of `routeEndpointError`.
pub fn route_endpoint_error(points: &[Point], source: Point, target: Point) -> f64 {
    if points.len() < 2 {
        return f64::INFINITY;
    }
    let start = points[0];
    let end = points[points.len() - 1];
    (start.x - source.x).hypot(start.y - source.y) + (end.x - target.x).hypot(end.y - target.y)
}

fn same_point(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() < 1e-6 && (a.y - b.y).abs() < 1e-6
}

fn is_orthogonal_segment(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() < 1e-6 || (a.y - b.y).abs() < 1e-6
}

/// Port of `stitchOrthogonalEndpoint`.
fn stitch_orthogonal_endpoint(endpoint: Point, route_point: Point) -> Vec<Point> {
    if same_point(endpoint, route_point) {
        return vec![endpoint];
    }
    if is_orthogonal_segment(endpoint, route_point) {
        return vec![endpoint, route_point];
    }
    vec![
        endpoint,
        Point {
            x: route_point.x,
            y: endpoint.y,
        },
        route_point,
    ]
}

/// Port of `snapRouteEndpoints`.
pub fn snap_route_endpoints(
    points: Vec<Point>,
    source: Option<Point>,
    target: Option<Point>,
) -> Vec<Point> {
    if points.len() < 2 {
        return points;
    }
    let mut route = points;
    if let Some(source) = source {
        let mut stitched = stitch_orthogonal_endpoint(source, route[0]);
        stitched.extend(route.into_iter().skip(1));
        route = stitched;
    }
    if let Some(target) = target {
        let last_route_point = *route.last().unwrap();
        let mut target_stitch = stitch_orthogonal_endpoint(target, last_route_point);
        target_stitch.reverse();
        route.truncate(route.len() - 1);
        route.extend(target_stitch);
    }
    prune_route_points(&route)
}

fn resolve_route_offset_candidates(edge: &LaidOutEdge) -> Vec<Point> {
    let edge_owner_offset = edge
        .layout
        .as_ref()
        .and_then(|l| l.edge_owner_offset)
        .unwrap_or(Point { x: 0.0, y: 0.0 });
    let lca_offset = edge
        .layout
        .as_ref()
        .and_then(|l| l.lca_offset)
        .unwrap_or(Point { x: 0.0, y: 0.0 });
    unique_offsets(vec![
        Point { x: 0.0, y: 0.0 },
        edge_owner_offset,
        lca_offset,
        Point {
            x: edge_owner_offset.x + lca_offset.x,
            y: edge_owner_offset.y + lca_offset.y,
        },
    ])
}

fn point_from_attribute(edge: &LaidOutEdge, key: &str) -> Option<Point> {
    let value = edge.attributes.get(key)?;
    let x = value.get("x")?.as_f64()?;
    let y = value.get("y")?.as_f64()?;
    Some(Point { x, y })
}

/// Port of `resolveIbdRoutePoints`.
pub fn resolve_ibd_route_points(edge: &LaidOutEdge) -> Option<Vec<Point>> {
    let sections = &edge.layout.as_ref()?.sections;
    if sections.is_empty() {
        return None;
    }

    let source_port = point_from_attribute(edge, "_sourcePortCenter");
    let target_port = point_from_attribute(edge, "_targetPortCenter");
    let candidates = resolve_route_offset_candidates(edge);

    let mut best_points: Option<Vec<Point>> = None;
    let mut best_error = f64::INFINITY;
    for offset in candidates {
        let points = points_from_elk_sections(sections, offset);
        if points.len() < 2 {
            continue;
        }
        let error = match (source_port, target_port) {
            (Some(source), Some(target)) => route_endpoint_error(&points, source, target),
            _ => {
                if offset.x == 0.0 && offset.y == 0.0 {
                    0.0
                } else {
                    offset.x.hypot(offset.y)
                }
            }
        };
        if error < best_error {
            best_error = error;
            best_points = Some(points);
        }
    }

    let best_points = best_points?;
    Some(snap_route_endpoints(best_points, source_port, target_port))
}
