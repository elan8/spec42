//! Deterministic A* routing on a visibility graph with rectangular obstacles.

use std::collections::{BTreeMap, BinaryHeap};

use elk_core::{Point, Rect};

const EPS: f32 = 1e-5;

/// Expanded rectangle obstacles (e.g. node bounds + clearance).
#[derive(Clone, Debug)]
pub struct Obstacle {
    pub rect: Rect,
}

impl Obstacle {
    #[must_use]
    pub fn corners(&self) -> [Point; 4] {
        let r = &self.rect;
        [
            Point::new(r.origin.x, r.origin.y),
            Point::new(r.origin.x + r.size.width, r.origin.y),
            Point::new(r.origin.x + r.size.width, r.origin.y + r.size.height),
            Point::new(r.origin.x, r.origin.y + r.size.height),
        ]
    }
}

/// Segment from (0,0) to (dx,dy) for direction checks.
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

/// Visibility graph vertex: index into points slice.
fn visible_neighbors(
    from: Point,
    points: &[Point],
    obstacles: &[Obstacle],
    from_idx: usize,
) -> Vec<(usize, f32)> {
    let mut out = Vec::new();
    for (i, &p) in points.iter().enumerate() {
        if i == from_idx {
            continue;
        }
        let dist = ((p.x - from.x).powi(2) + (p.y - from.y).powi(2)).sqrt();
        if dist < EPS {
            continue;
        }
        if !segment_intersects_obstacle(from, p, obstacles) {
            out.push((i, dist));
        }
    }
    out
}

/// A* state: (f_score, tie_break, node_idx). Min-heap so we reverse Ord.
#[derive(Clone, Copy, Debug)]
struct State {
    f: f32,
    tie: u32,
    idx: usize,
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

fn collinear(a: Point, b: Point, c: Point, eps: f32) -> bool {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let wx = c.x - b.x;
    let wy = c.y - b.y;
    (vx * wy - vy * wx).abs() < eps
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
) -> Vec<Point> {
    let mut points = vec![start, end];
    for o in obstacles {
        for c in o.corners() {
            if points.iter().all(|p| (p.x - c.x).abs() > EPS || (p.y - c.y).abs() > EPS) {
                points.push(c);
            }
        }
    }
    // Sort for deterministic ordering when building graph.
    points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal).then_with(|| {
            a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal)
        })
    });
    points.dedup_by(|a, b| (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS);
    if points.is_empty() {
        return vec![start, end];
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
        return vec![start, end];
    }

    let seg_penalty = segment_penalty.max(1e-6);
    let mut g_score: BTreeMap<usize, f32> = BTreeMap::new();
    g_score.insert(start_idx, 0.0);
    let mut came_from: BTreeMap<usize, usize> = BTreeMap::new();
    let mut open = BinaryHeap::new();
    let h_start = ((points[end_idx].x - start.x).powi(2) + (points[end_idx].y - start.y).powi(2)).sqrt() * seg_penalty;
    open.push(State {
        f: h_start,
        tie: start_idx as u32,
        idx: start_idx,
    });
    let mut tie_counter = 0u32;

    while let Some(State { idx: u, .. }) = open.pop() {
        if u == end_idx {
            let mut path = vec![points[end_idx]];
            let mut cur = end_idx;
            while let Some(&prev) = came_from.get(&cur) {
                path.push(points[prev]);
                cur = prev;
            }
            path.reverse();
            return path;
        }
        let current = points[u];
        let g_u = g_score.get(&u).copied().unwrap_or(f32::MAX);
        let prev_opt = came_from.get(&u).copied();
        for (v, w) in visible_neighbors(current, &points, obstacles, u) {
            let edge_cost = w * seg_penalty;
            let bend = prev_opt
                .map(|prev| !collinear(points[prev], current, points[v], EPS))
                .unwrap_or(false);
            let tentative = g_u + edge_cost + if bend { bend_penalty } else { 0.0 };
            if tentative < g_score.get(&v).copied().unwrap_or(f32::MAX) {
                came_from.insert(v, u);
                g_score.insert(v, tentative);
                tie_counter = tie_counter.wrapping_add(1);
                let h_v = ((points[end_idx].x - points[v].x).powi(2)
                    + (points[end_idx].y - points[v].y).powi(2))
                    .sqrt()
                    * seg_penalty;
                open.push(State {
                    f: tentative + h_v,
                    tie: tie_counter,
                    idx: v,
                });
            }
        }
    }

    vec![start, end]
}
