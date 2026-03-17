use crate::layout::{Bounds, DiagramLayout, LayoutMetrics, Point};

pub(crate) fn evaluate(layout: &DiagramLayout) -> LayoutMetrics {
    let mut overlap_count = 0usize;
    let mut overlap_area = 0.0f32;
    let mut minimum_node_clearance = f32::MAX;
    for (index, left) in layout.nodes.iter().enumerate() {
        for right in layout.nodes.iter().skip(index + 1) {
            let is_parent_child = left.parent_id.as_deref() == Some(right.id.as_str())
                || right.parent_id.as_deref() == Some(left.id.as_str());
            if is_parent_child {
                continue;
            }
            let overlap = overlap(left.bounds, right.bounds);
            if overlap > 0.0 {
                overlap_count += 1;
                overlap_area += overlap;
            }
            minimum_node_clearance =
                minimum_node_clearance.min(clearance(left.bounds, right.bounds));
        }
    }

    let mut edge_crossing_count = 0usize;
    for (index, left) in layout.edges.iter().enumerate() {
        for right in layout.edges.iter().skip(index + 1) {
            if left.source_node == right.source_node
                || left.source_node == right.target_node
                || left.target_node == right.source_node
                || left.target_node == right.target_node
            {
                continue;
            }
            for segment_left in left.points.windows(2) {
                for segment_right in right.points.windows(2) {
                    if segments_cross(
                        segment_left[0],
                        segment_left[1],
                        segment_right[0],
                        segment_right[1],
                    ) {
                        edge_crossing_count += 1;
                    }
                }
            }
        }
    }

    let mut edge_node_intrusion_count = 0usize;
    for edge in &layout.edges {
        for segment in edge.points.windows(2) {
            for node in &layout.nodes {
                if node.id == edge.source_node || node.id == edge.target_node {
                    continue;
                }
                if segment_hits_rect(segment[0], segment[1], node.bounds) {
                    edge_node_intrusion_count += 1;
                }
            }
        }
    }

    let total_edge_length = layout
        .edges
        .iter()
        .flat_map(|edge| edge.points.windows(2))
        .map(|pair| distance(pair[0], pair[1]))
        .sum();

    let bend_count = layout
        .edges
        .iter()
        .map(|edge| edge.points.len().saturating_sub(2))
        .sum();

    let orthogonal_violation_count = layout
        .edges
        .iter()
        .flat_map(|edge| edge.points.windows(2))
        .filter(|pair| (pair[0].x - pair[1].x).abs() > 0.1 && (pair[0].y - pair[1].y).abs() > 0.1)
        .count();

    let canvas_area = layout.width * layout.height;
    let node_area: f32 = layout
        .nodes
        .iter()
        .map(|node| node.bounds.width * node.bounds.height)
        .sum();

    LayoutMetrics {
        node_count: layout.nodes.len(),
        edge_count: layout.edges.len(),
        overlap_count,
        overlap_area,
        edge_crossing_count,
        edge_node_intrusion_count,
        total_edge_length,
        bend_count,
        orthogonal_violation_count,
        minimum_node_clearance: if minimum_node_clearance == f32::MAX {
            0.0
        } else {
            minimum_node_clearance
        },
        canvas_area,
        aspect_ratio: if layout.height > 0.0 {
            layout.width / layout.height
        } else {
            0.0
        },
        compactness: if canvas_area > 0.0 {
            node_area / canvas_area
        } else {
            0.0
        },
    }
}

fn overlap(left: Bounds, right: Bounds) -> f32 {
    let width = (left.right().min(right.right()) - left.x.max(right.x)).max(0.0);
    let height = (left.bottom().min(right.bottom()) - left.y.max(right.y)).max(0.0);
    width * height
}

fn clearance(left: Bounds, right: Bounds) -> f32 {
    let dx = if left.right() < right.x {
        right.x - left.right()
    } else if right.right() < left.x {
        left.x - right.right()
    } else {
        0.0
    };
    let dy = if left.bottom() < right.y {
        right.y - left.bottom()
    } else if right.bottom() < left.y {
        left.y - right.bottom()
    } else {
        0.0
    };
    if dx == 0.0 {
        dy
    } else if dy == 0.0 {
        dx
    } else {
        (dx * dx + dy * dy).sqrt()
    }
}

fn distance(left: Point, right: Point) -> f32 {
    let dx = right.x - left.x;
    let dy = right.y - left.y;
    (dx * dx + dy * dy).sqrt()
}

fn segment_hits_rect(left: Point, right: Point, rect: Bounds) -> bool {
    let min_x = left.x.min(right.x);
    let max_x = left.x.max(right.x);
    let min_y = left.y.min(right.y);
    let max_y = left.y.max(right.y);
    !(max_x < rect.x || min_x > rect.right() || max_y < rect.y || min_y > rect.bottom())
}

fn segments_cross(a1: Point, a2: Point, b1: Point, b2: Point) -> bool {
    fn orientation(p: Point, q: Point, r: Point) -> f32 {
        (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
    }
    let o1 = orientation(a1, a2, b1);
    let o2 = orientation(a1, a2, b2);
    let o3 = orientation(b1, b2, a1);
    let o4 = orientation(b1, b2, a2);
    (o1 > 0.0 && o2 < 0.0 || o1 < 0.0 && o2 > 0.0)
        && (o3 > 0.0 && o4 < 0.0 || o3 < 0.0 && o4 > 0.0)
}

