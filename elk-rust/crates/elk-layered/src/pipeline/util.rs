use elk_core::{LayoutDirection, Point, PortSide, Size};
use elk_graph::{EdgeEndpoint, ElkGraph, LabelId, NodeId, PortId};

use crate::ir::{IrNodeId, LayeredIr};
pub(crate) use elk_alg_common::geometry::dedup_points;

pub(crate) fn major_size(size: Size, direction: LayoutDirection) -> f32 {
    match direction {
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => size.width,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => size.height,
    }
}

pub(crate) fn minor_size(size: Size, direction: LayoutDirection) -> f32 {
    match direction {
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => size.height,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => size.width,
    }
}

pub(crate) fn placeholder_padding(ir: &LayeredIr, node_id: IrNodeId) -> f32 {
    if matches!(
        ir.nodes[node_id].kind,
        crate::ir::IrNodeKind::LabelPlaceholder { .. }
    ) {
        ir.nodes[node_id].label_size.height / 2.0
    } else if matches!(ir.nodes[node_id].kind, crate::ir::IrNodeKind::Dummy { .. }) {
        ir.nodes[node_id].size.height / 4.0
    } else {
        0.0
    }
}

pub(crate) fn node_minor_start(
    ir: &LayeredIr,
    node_id: IrNodeId,
    direction: LayoutDirection,
) -> f32 {
    match direction {
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => ir.nodes[node_id].position.y,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => ir.nodes[node_id].position.x,
    }
}

pub(crate) fn set_node_minor_start(
    ir: &mut LayeredIr,
    node_id: IrNodeId,
    direction: LayoutDirection,
    value: f32,
) {
    match direction {
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => {
            ir.nodes[node_id].position.y = value
        }
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => {
            ir.nodes[node_id].position.x = value
        }
    }
}

pub(crate) fn node_minor_center(
    ir: &LayeredIr,
    node_id: IrNodeId,
    direction: LayoutDirection,
) -> f32 {
    node_minor_start(ir, node_id, direction) + minor_size(ir.nodes[node_id].size, direction) / 2.0
}

/// Ensures every consecutive pair of points is axis-aligned by inserting one intermediate
/// point where needed. Use after routing so that the final path has only orthogonal segments.
#[allow(dead_code)]
pub(crate) fn ensure_orthogonal_path(points: Vec<Point>) -> Vec<Point> {
    const EPS: f32 = 1e-6;
    if points.len() < 2 {
        return points;
    }
    let mut out = Vec::with_capacity(points.len() * 2);
    out.push(points[0]);
    for i in 1..points.len() {
        let a = out.last().copied().unwrap();
        let b = points[i];
        if (a.x - b.x).abs() > EPS && (a.y - b.y).abs() > EPS {
            out.push(Point::new(a.x, b.y));
        }
        out.push(b);
    }
    out
}

/// Like [`ensure_orthogonal_path`], but chooses the inserted corner to prefer moving
/// along the layout's major axis first (reduces early detours/intrusions in layered routing).
#[allow(dead_code)]
pub(crate) fn ensure_orthogonal_path_prefer_major(
    points: Vec<Point>,
    direction: LayoutDirection,
) -> Vec<Point> {
    const EPS: f32 = 1e-6;
    if points.len() < 2 {
        return points;
    }
    let horizontal_major = matches!(
        direction,
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft
    );
    let mut out = Vec::with_capacity(points.len() * 2);
    out.push(points[0]);
    for i in 1..points.len() {
        let a = out.last().copied().unwrap();
        let b = points[i];
        if (a.x - b.x).abs() > EPS && (a.y - b.y).abs() > EPS {
            // Prefer a corner that moves along the major axis first.
            // - Horizontal-major: go (b.x, a.y) first (horizontal), then vertical.
            // - Vertical-major: go (a.x, b.y) first (vertical), then horizontal.
            let corner = if horizontal_major {
                Point::new(b.x, a.y)
            } else {
                Point::new(a.x, b.y)
            };
            out.push(corner);
        }
        out.push(b);
    }
    out
}

pub(crate) fn node_abs_origin(graph: &ElkGraph, node: NodeId) -> Point {
    let n = &graph.nodes[node.index()];
    match n.parent {
        Some(parent) if parent != graph.root => {
            let p = node_abs_origin(graph, parent);
            Point::new(p.x + n.geometry.x, p.y + n.geometry.y)
        }
        _ => Point::new(n.geometry.x, n.geometry.y),
    }
}

pub(crate) fn node_abs_size(graph: &ElkGraph, node: NodeId) -> Size {
    let n = &graph.nodes[node.index()];
    Size::new(n.geometry.width, n.geometry.height)
}

pub(crate) fn node_abs_center(graph: &ElkGraph, node: NodeId) -> Point {
    let o = node_abs_origin(graph, node);
    let s = node_abs_size(graph, node);
    Point::new(o.x + s.width / 2.0, o.y + s.height / 2.0)
}

pub(crate) fn port_abs_center(graph: &ElkGraph, port: PortId) -> Point {
    let p = &graph.ports[port.index()];
    let n = node_abs_origin(graph, p.node);
    Point::new(
        n.x + p.geometry.x + p.geometry.width / 2.0,
        n.y + p.geometry.y + p.geometry.height / 2.0,
    )
}

pub(crate) fn endpoint_abs_center(graph: &ElkGraph, endpoint: EdgeEndpoint) -> Point {
    if let Some(port) = endpoint.port {
        port_abs_center(graph, port)
    } else {
        node_abs_center(graph, endpoint.node)
    }
}

/// Explicit local<->absolute coordinate transform for one recursive layout scope.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalScopeFrame {
    pub origin_abs: Point,
}

impl LocalScopeFrame {
    #[must_use]
    pub const fn new(origin_abs: Point) -> Self {
        Self { origin_abs }
    }

    #[must_use]
    pub fn to_local(self, abs: Point) -> Point {
        Point::new(abs.x - self.origin_abs.x, abs.y - self.origin_abs.y)
    }

    #[must_use]
    pub fn to_abs(self, local: Point) -> Point {
        Point::new(local.x + self.origin_abs.x, local.y + self.origin_abs.y)
    }
}

/// Compute a deterministic local scope origin for a set of nodes.
#[must_use]
pub(crate) fn local_scope_frame(graph: &ElkGraph, scope_nodes: &std::collections::BTreeSet<NodeId>) -> LocalScopeFrame {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    for &node_id in scope_nodes {
        let o = node_abs_origin(graph, node_id);
        min_x = min_x.min(o.x);
        min_y = min_y.min(o.y);
    }
    if min_x.is_finite() && min_y.is_finite() {
        LocalScopeFrame::new(Point::new(min_x, min_y))
    } else {
        LocalScopeFrame::new(Point::new(0.0, 0.0))
    }
}

/// Port side for an endpoint, if it is a port; otherwise `None`.
pub(crate) fn endpoint_port_side(graph: &ElkGraph, endpoint: EdgeEndpoint) -> Option<PortSide> {
    endpoint.port.map(|pid| graph.ports[pid.index()].side)
}

/// Point obtained by moving from the port center along the outward normal (away from the node).
pub(crate) fn point_along_outward_normal(center: Point, side: PortSide, delta: f32) -> Point {
    match side {
        PortSide::North => Point::new(center.x, center.y - delta),
        PortSide::South => Point::new(center.x, center.y + delta),
        PortSide::East => Point::new(center.x + delta, center.y),
        PortSide::West => Point::new(center.x - delta, center.y),
    }
}

pub(crate) fn label_size(graph: &ElkGraph, label: LabelId) -> Size {
    let l = &graph.labels[label.index()];
    Size::new(l.geometry.width, l.geometry.height)
}

#[cfg(test)]
mod tests {
    use elk_alg_common::geometry::simplify_orthogonal_points_vec;
    use elk_core::Point;

    #[test]
    fn simplify_orthogonal_points_removes_collinear_vertices() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 10.0),
            Point::new(0.0, 20.0),
            Point::new(20.0, 20.0),
        ];

        assert_eq!(
            simplify_orthogonal_points_vec(points),
            vec![
                Point::new(0.0, 0.0),
                Point::new(0.0, 20.0),
                Point::new(20.0, 20.0),
            ]
        );
    }
}
