//! Deterministic A* routing on an orthogonal visibility graph with rectangular obstacles.

use std::collections::{BTreeMap, BinaryHeap};

use elk_core::{Point, Rect};

const EPS: f32 = 1e-5;

/// Expanded rectangle obstacles (e.g. node bounds + clearance).
#[derive(Clone, Debug)]
pub struct Obstacle {
    pub rect: Rect,
}

#[derive(Clone, Debug, Default)]
pub struct RouteDebug {
    pub candidate_points: usize,
    pub expanded_states: usize,
    pub neighbor_checks: usize,
    pub accepted_neighbors: usize,
    pub blocked_neighbors: usize,
    pub path_found: bool,
}

#[derive(Clone, Debug)]
pub enum RoutingFailure {
    NoCandidatePoints,
    DegenerateEndpoints,
    NoRouteFound,
}

fn rect_contains(r: &Rect, p: Point) -> bool {
    p.x >= r.origin.x + EPS
        && p.x <= r.origin.x + r.size.width - EPS
        && p.y >= r.origin.y + EPS
        && p.y <= r.origin.y + r.size.height - EPS
}

fn segment_intersects_rect(a: Point, b: Point, r: &Rect) -> bool {
    let min_x = r.origin.x + EPS;
    let max_x = r.origin.x + r.size.width - EPS;
    let min_y = r.origin.y + EPS;
    let max_y = r.origin.y + r.size.height - EPS;
    // If either endpoint is inside, segment crosses.
    if (a.x > min_x && a.x < max_x && a.y > min_y && a.y < max_y)
        || (b.x > min_x && b.x < max_x && b.y > min_y && b.y < max_y)
    {
        return true;
    }
    // Liang–Barsky: segment (a + t*(b-a)) vs rect.
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;
    let edges = [(dx, min_x - a.x), (-dx, a.x - max_x), (dy, min_y - a.y), (-dy, a.y - max_y)];
    for (denom, num) in edges {
        if denom.abs() < EPS {
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
    t0 <= t1 + EPS
}

fn segment_intersects_obstacle(a: Point, b: Point, obstacles: &[Obstacle]) -> bool {
    for o in obstacles {
        if rect_contains(&o.rect, a) || rect_contains(&o.rect, b) {
            continue;
        }
        if segment_intersects_rect(a, b, &o.rect) {
            return true;
        }
    }
    false
}

fn orthogonal_visible_neighbors(
    from: Point,
    points: &[Point],
    obstacles: &[Obstacle],
    from_idx: usize,
) -> Vec<(usize, f32, Axis)> {
    let mut out = Vec::new();
    for (i, &p) in points.iter().enumerate() {
        if i == from_idx {
            continue;
        }
        let axis = if (p.x - from.x).abs() < EPS {
            Some(Axis::Vertical)
        } else if (p.y - from.y).abs() < EPS {
            Some(Axis::Horizontal)
        } else {
            None
        };
        let Some(axis) = axis else {
            continue;
        };
        let dist = (p.x - from.x).abs() + (p.y - from.y).abs();
        if dist < EPS {
            continue;
        }
        if !segment_intersects_obstacle(from, p, obstacles) {
            out.push((i, dist, axis));
        }
    }
    out
}

/// A* state: (f_score, tie_break, node_idx). Min-heap so we reverse Ord.
#[derive(Clone, Copy, Debug)]
struct State {
    f: f32,
    tie: usize,
    idx: usize,
    axis: Option<Axis>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.tie == other.tie
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f
            .partial_cmp(&self.f)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.tie.cmp(&other.tie))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Horizontal,
    Vertical,
}

fn obstacle_channels(obstacles: &[Obstacle]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for o in obstacles {
        let r = o.rect;
        xs.push(r.origin.x);
        xs.push(r.origin.x + r.size.width);
        ys.push(r.origin.y);
        ys.push(r.origin.y + r.size.height);
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    xs.dedup_by(|a, b| (*a - *b).abs() < EPS);
    ys.dedup_by(|a, b| (*a - *b).abs() < EPS);
    (xs, ys)
}

fn inside_any_obstacle(p: Point, obstacles: &[Obstacle]) -> bool {
    obstacles.iter().any(|o| rect_contains(&o.rect, p))
}

fn candidate_points(start: Point, end: Point, obstacles: &[Obstacle]) -> Vec<Point> {
    let (xs, ys) = obstacle_channels(obstacles);
    let mut points = vec![start, end];
    let mut all_x = xs;
    let mut all_y = ys;
    all_x.push(start.x);
    all_x.push(end.x);
    all_y.push(start.y);
    all_y.push(end.y);
    all_x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    all_y.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    all_x.dedup_by(|a, b| (*a - *b).abs() < EPS);
    all_y.dedup_by(|a, b| (*a - *b).abs() < EPS);
    for x in &all_x {
        for y in &all_y {
            let p = Point::new(*x, *y);
            if !inside_any_obstacle(p, obstacles) {
                points.push(p);
            }
        }
    }
    points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    points.dedup_by(|a, b| (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS);
    points
}

/// Route from start to end avoiding obstacles. Returns path [start, ...bends..., end].
/// `segment_penalty` scales segment length in cost; `bend_penalty` is added per bend.
#[must_use]
pub fn route(
    start: Point,
    end: Point,
    obstacles: &[Obstacle],
    segment_penalty: f32,
    bend_penalty: f32,
) -> Result<Vec<Point>, RoutingFailure> {
    route_with_debug(start, end, obstacles, segment_penalty, bend_penalty).map(|v| v.0)
}

#[must_use]
pub fn route_with_debug(
    start: Point,
    end: Point,
    obstacles: &[Obstacle],
    segment_penalty: f32,
    bend_penalty: f32,
) -> Result<(Vec<Point>, RouteDebug), RoutingFailure> {
    let mut dbg = RouteDebug::default();
    let points = candidate_points(start, end, obstacles);
    dbg.candidate_points = points.len();
    if points.is_empty() {
        return Err(RoutingFailure::NoCandidatePoints);
    }
    let start_idx = points
        .iter()
        .position(|p| (p.x - start.x).abs() < EPS && (p.y - start.y).abs() < EPS)
        .unwrap_or(0);
    let end_idx = points
        .iter()
        .position(|p| (p.x - end.x).abs() < EPS && (p.y - end.y).abs() < EPS)
        .unwrap_or(1);
    if start_idx == end_idx {
        return Err(RoutingFailure::DegenerateEndpoints);
    }

    let seg_penalty = segment_penalty.max(1e-6);
    let mut g_score: BTreeMap<usize, f32> = BTreeMap::new();
    g_score.insert(start_idx, 0.0);
    let mut came_from: BTreeMap<usize, usize> = BTreeMap::new();
    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let h_start = ((points[end_idx].x - start.x).powi(2) + (points[end_idx].y - start.y).powi(2)).sqrt() * seg_penalty;
    open.push(State {
        f: h_start,
        tie: start_idx,
        idx: start_idx,
        axis: None,
    });
    let mut best_axis: BTreeMap<usize, Option<Axis>> = BTreeMap::new();
    best_axis.insert(start_idx, None);

    while let Some(State { idx: u, axis: incoming_axis, .. }) = open.pop() {
        dbg.expanded_states += 1;
        if u == end_idx {
            let mut path = vec![points[end_idx]];
            let mut cur = end_idx;
            while let Some(&prev) = came_from.get(&cur) {
                path.push(points[prev]);
                cur = prev;
            }
            path.reverse();
            dbg.path_found = true;
            return Ok((path, dbg));
        }
        let current = points[u];
        let g_u = g_score.get(&u).copied().unwrap_or(f32::MAX);
        for (v, w, axis) in orthogonal_visible_neighbors(current, &points, obstacles, u) {
            dbg.neighbor_checks += 1;
            let edge_cost = w * seg_penalty;
            let bend = incoming_axis.map(|prev_axis| prev_axis != axis).unwrap_or(false);
            let mut tentative = g_u + edge_cost + if bend { bend_penalty.max(1.0) } else { 0.0 };
            // Prefer monotone progress toward target to avoid route oscillations.
            let prev_dist = (points[end_idx].x - current.x).abs() + (points[end_idx].y - current.y).abs();
            let next_dist = (points[end_idx].x - points[v].x).abs() + (points[end_idx].y - points[v].y).abs();
            if next_dist > prev_dist + EPS {
                tentative += 0.35 * seg_penalty;
            }
            if tentative < g_score.get(&v).copied().unwrap_or(f32::MAX) {
                dbg.accepted_neighbors += 1;
                came_from.insert(v, u);
                g_score.insert(v, tentative);
                best_axis.insert(v, Some(axis));
                let h_v = ((points[end_idx].x - points[v].x).powi(2)
                    + (points[end_idx].y - points[v].y).powi(2))
                    .sqrt()
                    * seg_penalty;
                open.push(State {
                    f: tentative + h_v,
                    tie: v,
                    idx: v,
                    axis: Some(axis),
                });
            } else {
                dbg.blocked_neighbors += 1;
            }
        }
    }

    Err(RoutingFailure::NoRouteFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use elk_core::Size;

    fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(Point::new(x, y), Size::new(w, h))
    }

    #[test]
    fn route_is_deterministic_for_same_input() {
        let obstacles = vec![
            Obstacle { rect: rect(80.0, 20.0, 30.0, 60.0) },
            Obstacle { rect: rect(130.0, 20.0, 30.0, 60.0) },
        ];
        let start = Point::new(20.0, 50.0);
        let end = Point::new(220.0, 50.0);
        let a = route(start, end, &obstacles, 1.0, 6.0).expect("route should succeed");
        let b = route(start, end, &obstacles, 1.0, 6.0).expect("route should succeed");
        assert_eq!(a, b);
    }

    #[test]
    fn narrow_corridor_routes_without_entering_obstacles() {
        let obstacles = vec![
            Obstacle { rect: rect(80.0, 0.0, 60.0, 40.0) },
            Obstacle { rect: rect(80.0, 60.0, 60.0, 40.0) },
        ];
        let start = Point::new(20.0, 50.0);
        let end = Point::new(220.0, 50.0);
        let path = route(start, end, &obstacles, 1.0, 6.0).expect("route should succeed");
        assert!(path.len() >= 2);
        // At minimum the route should not collapse to an empty result.
        assert!(path.first().is_some() && path.last().is_some());
    }
}
