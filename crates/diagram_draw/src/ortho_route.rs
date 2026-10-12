//! An orthogonal route between two boxes that goes around the other boxes.
//!
//! A layout that places boxes by containment (a decomposition tree, the states of a nested
//! machine) leaves the lines that are not part of that structure to be drawn afterwards: a
//! typing or specialization line between two boxes of a tree, a transition between a nested
//! state and one outside it. Drawn straight, such a line runs through whatever stands between
//! its ends. This routes it along the gaps instead.
//!
//! The route is the cheapest path over the grid spanned by the edges of every box, each grown by
//! a clearance, where a bend costs as much as a detour of [`BEND_COST`]. It leaves and enters a
//! box at the middle of a side. Everything here is geometry.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A box as `(x, y, width, height)`.
pub(crate) type Rect = (f64, f64, f64, f64);
pub(crate) type Point = (f64, f64);

/// What a bend costs, in pixels of extra length. High enough that a route prefers a longer
/// straight run to a staircase.
const BEND_COST: f64 = 40.0;
/// Routing is skipped for a diagram with more boxes than this; the grid grows with their square.
const MAX_BOXES: usize = 160;

#[derive(Clone, Copy, PartialEq)]
struct Visit {
    cost: f64,
    node: usize,
    /// 0 horizontal, 1 vertical.
    axis: usize,
}

impl Eq for Visit {}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // A min-heap on cost; ties by node and axis so that the result does not depend on
        // insertion order.
        other
            .cost
            .total_cmp(&self.cost)
            .then(other.node.cmp(&self.node))
            .then(other.axis.cmp(&self.axis))
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn grown(rect: Rect, by: f64) -> Rect {
    (
        rect.0 - by,
        rect.1 - by,
        rect.2 + 2.0 * by,
        rect.3 + 2.0 * by,
    )
}

/// The middle of each side of `rect` with the point `clearance` outside it, and the axis the
/// line runs along there.
fn ports(rect: Rect, clearance: f64) -> [(Point, Point, usize); 4] {
    let (x, y, width, height) = rect;
    let (cx, cy) = (x + width / 2.0, y + height / 2.0);
    [
        ((cx, y), (cx, y - clearance), 1),
        ((cx, y + height), (cx, y + height + clearance), 1),
        ((x, cy), (x - clearance, cy), 0),
        ((x + width, cy), (x + width + clearance, cy), 0),
    ]
}

/// A route from a side of `from` to a side of `to` that keeps `clearance` away from every box
/// in `boxes` (which may include the two ends). `None` when the diagram is too large to route
/// or no such route exists; the caller then draws its own direct line.
pub(crate) fn route(from: Rect, to: Rect, boxes: &[Rect], clearance: f64) -> Option<Vec<Point>> {
    if boxes.len() > MAX_BOXES {
        return None;
    }
    // A line may run along the grown edge of a box, not inside it.
    let blocked: Vec<Rect> = boxes
        .iter()
        .chain([&from, &to])
        .map(|rect| grown(*rect, clearance - 0.5))
        .collect();
    let starts = ports(from, clearance);
    let ends = ports(to, clearance);

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for rect in boxes.iter().chain([&from, &to]) {
        let outer = grown(*rect, clearance);
        xs.extend([outer.0, outer.0 + outer.2]);
        ys.extend([outer.1, outer.1 + outer.3]);
    }
    for (_, stub, _) in starts.iter().chain(&ends) {
        xs.push(stub.0);
        ys.push(stub.1);
    }
    for values in [&mut xs, &mut ys] {
        values.sort_by(f64::total_cmp);
        values.dedup_by(|a, b| (*a - *b).abs() < 0.01);
    }
    let index_of = |values: &[f64], value: f64| {
        values
            .iter()
            .position(|candidate| (candidate - value).abs() < 0.01)
    };
    let columns = xs.len();
    let node = |column: usize, row: usize| row * columns + column;
    let inside = |x: f64, y: f64| {
        blocked
            .iter()
            .any(|(bx, by, width, height)| x > *bx && x < bx + width && y > *by && y < by + height)
    };
    // Whether the straight run between two neighbouring grid points passes through a box.
    let crosses = |a: Point, b: Point| {
        let (left, right) = (a.0.min(b.0), a.0.max(b.0));
        let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
        blocked.iter().any(|(bx, by, width, height)| {
            if (a.1 - b.1).abs() < 0.01 {
                a.1 > *by && a.1 < by + height && left < bx + width && right > *bx
            } else {
                a.0 > *bx && a.0 < bx + width && top < by + height && bottom > *by
            }
        })
    };

    let count = columns * ys.len();
    let mut best = vec![[f64::INFINITY; 2]; count];
    let mut previous: Vec<[Option<(usize, usize)>; 2]> = vec![[None; 2]; count];
    let mut queue = BinaryHeap::new();
    for (_, stub, axis) in &starts {
        if inside(stub.0, stub.1) {
            continue;
        }
        let (Some(column), Some(row)) = (index_of(&xs, stub.0), index_of(&ys, stub.1)) else {
            continue;
        };
        let start = node(column, row);
        if best[start][*axis] > 0.0 {
            best[start][*axis] = 0.0;
            queue.push(Visit {
                cost: 0.0,
                node: start,
                axis: *axis,
            });
        }
    }
    let goals: Vec<(usize, usize, Point)> = ends
        .iter()
        .filter(|(_, stub, _)| !inside(stub.0, stub.1))
        .filter_map(|(port, stub, axis)| {
            Some((
                node(index_of(&xs, stub.0)?, index_of(&ys, stub.1)?),
                *axis,
                *port,
            ))
        })
        .collect();

    let mut reached: Option<(usize, usize, Point)> = None;
    while let Some(Visit {
        cost,
        node: at,
        axis,
    }) = queue.pop()
    {
        if cost > best[at][axis] {
            continue;
        }
        // A route ends where it can run straight into the side of the target.
        if let Some((_, _, port)) = goals
            .iter()
            .find(|(goal, goal_axis, _)| *goal == at && *goal_axis == axis)
        {
            reached = Some((at, axis, *port));
            break;
        }
        let (column, row) = (at % columns, at / columns);
        let here = (xs[column], ys[row]);
        let mut neighbours = Vec::with_capacity(4);
        if column > 0 {
            neighbours.push((column - 1, row, 0));
        }
        if column + 1 < columns {
            neighbours.push((column + 1, row, 0));
        }
        if row > 0 {
            neighbours.push((column, row - 1, 1));
        }
        if row + 1 < ys.len() {
            neighbours.push((column, row + 1, 1));
        }
        for (next_column, next_row, next_axis) in neighbours {
            let there = (xs[next_column], ys[next_row]);
            if inside(there.0, there.1) || crosses(here, there) {
                continue;
            }
            let step = (there.0 - here.0).abs() + (there.1 - here.1).abs();
            let next_cost = cost + step + if next_axis == axis { 0.0 } else { BEND_COST };
            let next = node(next_column, next_row);
            if next_cost < best[next][next_axis] {
                best[next][next_axis] = next_cost;
                previous[next][next_axis] = Some((at, axis));
                queue.push(Visit {
                    cost: next_cost,
                    node: next,
                    axis: next_axis,
                });
            }
        }
    }

    let (mut at, mut axis, end_port) = reached?;
    let mut points = vec![end_port];
    loop {
        points.push((xs[at % columns], ys[at / columns]));
        match previous[at][axis] {
            Some((before, before_axis)) => {
                at = before;
                axis = before_axis;
            }
            None => break,
        }
    }
    let first = *points.last()?;
    let start_port = starts
        .iter()
        .find(|(_, stub, _)| (stub.0 - first.0).abs() < 0.01 && (stub.1 - first.1).abs() < 0.01)
        .map(|(port, _, _)| *port)?;
    points.push(start_port);
    points.reverse();
    Some(simplified(points))
}

/// Drops the points that lie on a straight run between their neighbours.
fn simplified(points: Vec<Point>) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::with_capacity(points.len());
    for point in points {
        if let [.., before, last] = out.as_slice() {
            let same_x = (before.0 - last.0).abs() < 0.01 && (last.0 - point.0).abs() < 0.01;
            let same_y = (before.1 - last.1).abs() < 0.01 && (last.1 - point.1).abs() < 0.01;
            if same_x || same_y {
                out.pop();
            }
        }
        if out
            .last()
            .is_none_or(|last| (last.0 - point.0).abs() > 0.01 || (last.1 - point.1).abs() > 0.01)
        {
            out.push(point);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passes_through(points: &[Point], rect: Rect) -> bool {
        points.windows(2).any(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let (left, right) = (a.0.min(b.0), a.0.max(b.0));
            let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
            left < rect.0 + rect.2 && right > rect.0 && top < rect.1 + rect.3 && bottom > rect.1
        })
    }

    #[test]
    fn a_route_goes_around_the_box_between_its_ends() {
        let from = (0.0, 0.0, 100.0, 40.0);
        let to = (0.0, 200.0, 100.0, 40.0);
        let between = (-20.0, 90.0, 140.0, 40.0);
        let points = route(from, to, &[between], 10.0).expect("a route exists");
        assert!(!passes_through(&points, between), "{points:?}");
        // It leaves and enters at the middle of a side, and every run is horizontal or
        // vertical.
        assert!(ports(from, 10.0)
            .iter()
            .any(|(port, _, _)| *port == points[0]));
        assert!(ports(to, 10.0)
            .iter()
            .any(|(port, _, _)| *port == *points.last().unwrap()));
        assert!(points
            .windows(2)
            .all(|pair| pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1));
    }

    #[test]
    fn an_unobstructed_route_is_one_straight_run() {
        let from = (0.0, 0.0, 100.0, 40.0);
        let to = (0.0, 200.0, 100.0, 40.0);
        let points = route(from, to, &[], 10.0).expect("a route exists");
        assert_eq!(points, vec![(50.0, 40.0), (50.0, 200.0)]);
    }

    #[test]
    fn the_same_boxes_give_the_same_route() {
        let from = (0.0, 0.0, 80.0, 40.0);
        let to = (300.0, 220.0, 80.0, 40.0);
        let boxes = [(120.0, 60.0, 120.0, 120.0), (40.0, 150.0, 60.0, 40.0)];
        let first = route(from, to, &boxes, 12.0);
        assert!(first.is_some());
        assert_eq!(first, route(from, to, &boxes, 12.0));
    }

    #[test]
    fn a_diagram_too_large_to_route_is_left_to_the_caller() {
        let boxes = vec![(0.0, 0.0, 1.0, 1.0); MAX_BOXES + 1];
        assert!(route(
            (0.0, 0.0, 10.0, 10.0),
            (50.0, 50.0, 10.0, 10.0),
            &boxes,
            5.0
        )
        .is_none());
    }
}
