//! Port of `render/ibd-route.ts`.

use std::collections::HashMap;

use crate::types::{attr_str, EdgeSection, LaidOutEdge, LaidOutNode, Point};

const CONNECTOR_LANE_SPACING: f64 = 4.0;

/// A resolved connector route ready for the presentation-only lane separation pass.
#[derive(Debug, Clone)]
pub struct ConnectorRoute {
    pub id: String,
    pub source_port_id: String,
    pub target_port_id: String,
    pub points: Vec<Point>,
}

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

fn segment_overlap(a0: Point, a1: Point, b0: Point, b1: Point) -> f64 {
    if (a0.y - a1.y).abs() < 1e-6 && (b0.y - b1.y).abs() < 1e-6 && (a0.y - b0.y).abs() < 1e-6 {
        return a0.x.max(a1.x).min(b0.x.max(b1.x)) - a0.x.min(a1.x).max(b0.x.min(b1.x));
    }
    if (a0.x - a1.x).abs() < 1e-6 && (b0.x - b1.x).abs() < 1e-6 && (a0.x - b0.x).abs() < 1e-6 {
        return a0.y.max(a1.y).min(b0.y.max(b1.y)) - a0.y.min(a1.y).max(b0.y.min(b1.y));
    }
    0.0
}

fn routes_share_segment(left: &[Point], right: &[Point]) -> bool {
    left.windows(2).any(|a| {
        right
            .windows(2)
            .any(|b| segment_overlap(a[0], a[1], b[0], b[1]) > 1e-6)
    })
}

fn shifted(point: Point, horizontal: bool, offset: f64) -> Point {
    if horizontal {
        Point {
            x: point.x,
            y: point.y + offset,
        }
    } else {
        Point {
            x: point.x + offset,
            y: point.y,
        }
    }
}

fn offset_orthogonal_route(points: &[Point], offset: f64) -> Vec<Point> {
    let points = prune_route_points(points);
    if points.len() < 2 || offset.abs() < 1e-6 {
        return points;
    }
    if points
        .windows(2)
        .any(|segment| !is_orthogonal_segment(segment[0], segment[1]))
    {
        return points;
    }

    let last = points.len() - 1;
    let first_horizontal = (points[0].y - points[1].y).abs() < 1e-6;
    let last_horizontal = (points[last - 1].y - points[last].y).abs() < 1e-6;

    let mut routed = vec![points[0], shifted(points[0], first_horizontal, offset)];
    for vertex in 1..last {
        let previous_horizontal = (points[vertex - 1].y - points[vertex].y).abs() < 1e-6;
        let next_horizontal = (points[vertex].y - points[vertex + 1].y).abs() < 1e-6;
        let corner = match (previous_horizontal, next_horizontal) {
            (true, false) => Point {
                x: points[vertex].x + offset,
                y: points[vertex].y + offset,
            },
            (false, true) => Point {
                x: points[vertex].x + offset,
                y: points[vertex].y + offset,
            },
            _ => shifted(points[vertex], previous_horizontal, offset),
        };
        routed.push(corner);
    }
    routed.extend([shifted(points[last], last_horizontal, offset), points[last]]);
    prune_route_points(&routed)
}

fn find_root(parents: &mut [usize], index: usize) -> usize {
    if parents[index] != index {
        parents[index] = find_root(parents, parents[index]);
    }
    parents[index]
}

/// Turns only ELK's coincident hyperedge centerlines into stable parallel connector lanes. ELK's
/// route shape and exact port endpoints remain authoritative.
pub fn separate_shared_connector_routes(routes: &mut [ConnectorRoute]) {
    let mut parents: Vec<usize> = (0..routes.len()).collect();
    for left in 0..routes.len() {
        for right in (left + 1)..routes.len() {
            if routes_share_segment(&routes[left].points, &routes[right].points) {
                let left_root = find_root(&mut parents, left);
                let right_root = find_root(&mut parents, right);
                parents[right_root] = left_root;
            }
        }
    }

    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for index in 0..routes.len() {
        let root = find_root(&mut parents, index);
        groups.entry(root).or_default().push(index);
    }
    for mut group in groups.into_values().filter(|group| group.len() > 1) {
        group.sort_by(|left, right| routes[*left].id.cmp(&routes[*right].id));
        let center = (group.len() - 1) as f64 / 2.0;
        for (rank, index) in group.into_iter().enumerate() {
            let offset = (rank as f64 - center) * CONNECTOR_LANE_SPACING;
            routes[index].points = offset_orthogonal_route(&routes[index].points, offset);
        }
    }
}

fn container_chain(node: &LaidOutNode, nodes_by_id: &HashMap<String, LaidOutNode>) -> Vec<String> {
    let mut chain = Vec::new();
    let mut current = Some(node);
    while let Some(node) = current {
        chain.push(node.id.clone());
        let parent_id = attr_str(&node.attributes, "containerId").unwrap_or_default();
        current = if parent_id.is_empty() {
            None
        } else {
            nodes_by_id.get(&parent_id)
        };
    }
    chain
}

/// Port of `lcaOffsetForNodes`.
pub fn lca_offset_for_nodes(
    source_node: &LaidOutNode,
    target_node: &LaidOutNode,
    laid_out_nodes: &HashMap<String, LaidOutNode>,
) -> Point {
    let source_chain = container_chain(source_node, laid_out_nodes);
    let target_set: std::collections::HashSet<String> =
        container_chain(target_node, laid_out_nodes)
            .into_iter()
            .collect();
    let Some(lca_id) = source_chain.into_iter().find(|id| target_set.contains(id)) else {
        return Point { x: 0.0, y: 0.0 };
    };
    laid_out_nodes
        .get(&lca_id)
        .map(|node| Point {
            x: node.x,
            y: node.y,
        })
        .unwrap_or(Point { x: 0.0, y: 0.0 })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn route(id: &str) -> ConnectorRoute {
        ConnectorRoute {
            id: id.into(),
            source_port_id: "source.p".into(),
            target_port_id: format!("{id}.p"),
            points: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 60.0, y: 0.0 },
                Point { x: 60.0, y: 40.0 },
                Point { x: 120.0, y: 40.0 },
            ],
        }
    }

    #[test]
    fn separates_shared_routes_into_deterministic_parallel_lanes() {
        let mut routes = vec![route("b"), route("a")];
        separate_shared_connector_routes(&mut routes);

        for route in &routes {
            assert!(same_point(route.points[0], Point { x: 0.0, y: 0.0 }));
            assert!(same_point(
                *route.points.last().unwrap(),
                Point { x: 120.0, y: 40.0 }
            ));
            assert!(route
                .points
                .windows(2)
                .all(|segment| is_orthogonal_segment(segment[0], segment[1])));
        }

        let a = routes.iter().find(|route| route.id == "a").unwrap();
        let b = routes.iter().find(|route| route.id == "b").unwrap();
        assert!(a.points.iter().any(|point| (point.y + 2.0).abs() < 1e-6));
        assert!(b.points.iter().any(|point| (point.y - 2.0).abs() < 1e-6));
        let longest_overlap = a
            .points
            .windows(2)
            .flat_map(|left| {
                b.points
                    .windows(2)
                    .map(move |right| segment_overlap(left[0], left[1], right[0], right[1]))
            })
            .fold(0.0_f64, f64::max);
        assert!(longest_overlap <= 1e-6);
    }

    #[test]
    fn leaves_routes_without_coincident_segments_unchanged() {
        let mut routes = vec![
            ConnectorRoute {
                id: "horizontal".into(),
                source_port_id: "shared.p".into(),
                target_port_id: "right.p".into(),
                points: vec![Point { x: 0.0, y: 0.0 }, Point { x: 80.0, y: 0.0 }],
            },
            ConnectorRoute {
                id: "vertical".into(),
                source_port_id: "shared.p".into(),
                target_port_id: "bottom.p".into(),
                points: vec![Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 80.0 }],
            },
        ];
        let original: Vec<Vec<(f64, f64)>> = routes
            .iter()
            .map(|route| {
                route
                    .points
                    .iter()
                    .map(|point| (point.x, point.y))
                    .collect()
            })
            .collect();

        separate_shared_connector_routes(&mut routes);

        let actual: Vec<Vec<(f64, f64)>> = routes
            .iter()
            .map(|route| {
                route
                    .points
                    .iter()
                    .map(|point| (point.x, point.y))
                    .collect()
            })
            .collect();
        assert_eq!(actual, original);
    }
}
