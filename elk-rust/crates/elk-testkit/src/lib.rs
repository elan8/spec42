#![forbid(unsafe_code)]
#![allow(deprecated)]

mod parity;
mod routing_invariants;
mod svg;

pub use parity::{compare_layout_json, compare_layout_json_relaxed, node_ids_from_json};
pub use routing_invariants::{assert_routed_paths_avoid_obstacles, segment_intersects_rect_interior};

pub use svg::{maybe_write_svg_snapshot, render_graph_svg, snapshot_dir, snapshot_file_name};

use elk_core::{
    EdgeEndpoint, Graph, LayoutDirection, LayoutOptions, LayerConstraint, NodeId,
    Point, PortConstraint, PortSide, Rect, Size, ViewProfile,
};
use elk_graph::ElkGraph;

#[derive(Clone, Copy, Debug)]
pub struct DifferentialBaseline {
    pub layers_range: (usize, usize),
    pub min_dummy_nodes: usize,
    pub min_normalized_edges: usize,
    pub max_crossings_after: usize,
    pub min_straight_segments: usize,
    pub width_range: (f32, f32),
    pub height_range: (f32, f32),
}

pub fn canonical_dag() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let b = graph.add_node(Size::new(80.0, 40.0));
    let c = graph.add_node(Size::new(80.0, 40.0));
    let d = graph.add_node(Size::new(80.0, 40.0));

    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(c));
    graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(d));
    graph.add_edge(EdgeEndpoint::node(c), EdgeEndpoint::node(d));
    graph
}

pub fn crossing_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let b = graph.add_node(Size::new(80.0, 40.0));
    let c = graph.add_node(Size::new(80.0, 40.0));
    let d = graph.add_node(Size::new(80.0, 40.0));

    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(d));
    graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
    graph
}

pub fn dense_crossing_graph() -> Graph {
    let mut graph = Graph::new();
    let left: Vec<_> = (0..3)
        .map(|_| graph.add_node(Size::new(80.0, 40.0)))
        .collect();
    let right: Vec<_> = (0..3)
        .map(|_| graph.add_node(Size::new(80.0, 40.0)))
        .collect();

    graph.add_edge(EdgeEndpoint::node(left[0]), EdgeEndpoint::node(right[2]));
    graph.add_edge(EdgeEndpoint::node(left[0]), EdgeEndpoint::node(right[1]));
    graph.add_edge(EdgeEndpoint::node(left[1]), EdgeEndpoint::node(right[2]));
    graph.add_edge(EdgeEndpoint::node(left[1]), EdgeEndpoint::node(right[0]));
    graph.add_edge(EdgeEndpoint::node(left[2]), EdgeEndpoint::node(right[1]));
    graph.add_edge(EdgeEndpoint::node(left[2]), EdgeEndpoint::node(right[0]));
    graph
}

pub fn parallel_edges_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let b = graph.add_node(Size::new(80.0, 40.0));
    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph
}

pub fn label_heavy_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(110.0, 60.0));
    let b = graph.add_node(Size::new(110.0, 60.0));
    let c = graph.add_node(Size::new(110.0, 60.0));
    graph.add_node_label(a, "Input", Size::new(44.0, 18.0));
    graph.add_node_label(b, "Transform", Size::new(68.0, 18.0));
    let edge_ab = graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    let edge_bc = graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
    graph.add_edge_label(edge_ab, "throughput", Size::new(70.0, 18.0));
    graph.add_edge_label(edge_bc, "latency", Size::new(54.0, 18.0));
    graph
}

pub fn mixed_port_sides_graph() -> Graph {
    let mut graph = Graph::new();
    let left = graph.add_node(Size::new(90.0, 60.0));
    let center = graph.add_node(Size::new(90.0, 60.0));
    let right = graph.add_node(Size::new(90.0, 60.0));
    let left_east = graph.add_port(left, PortSide::East, Size::new(10.0, 10.0));
    let center_west = graph.add_port(center, PortSide::West, Size::new(10.0, 10.0));
    let center_south = graph.add_port(center, PortSide::South, Size::new(10.0, 10.0));
    let right_north = graph.add_port(right, PortSide::North, Size::new(10.0, 10.0));
    graph.add_edge(EdgeEndpoint::port(left, left_east), EdgeEndpoint::port(center, center_west));
    graph.add_edge(EdgeEndpoint::port(center, center_south), EdgeEndpoint::port(right, right_north));
    graph
}

pub fn compound_graph() -> Graph {
    let mut graph = Graph::new();
    let parent = graph.add_node(Size::new(160.0, 110.0));
    let child_a = graph.add_child_node(parent, Size::new(60.0, 40.0));
    let child_b = graph.add_child_node(parent, Size::new(60.0, 40.0));
    let outside = graph.add_node(Size::new(90.0, 50.0));

    graph.add_edge(EdgeEndpoint::node(child_a), EdgeEndpoint::node(child_b));
    graph.add_edge(EdgeEndpoint::node(parent), EdgeEndpoint::node(outside));
    graph
}

pub fn port_graph() -> Graph {
    let mut graph = Graph::new();
    let left = graph.add_node(Size::new(80.0, 40.0));
    let right = graph.add_node(Size::new(80.0, 40.0));
    let left_top = graph.add_port(left, PortSide::East, Size::new(10.0, 10.0));
    let left_bottom = graph.add_port(left, PortSide::East, Size::new(10.0, 10.0));
    let right_top = graph.add_port(right, PortSide::West, Size::new(10.0, 10.0));
    let right_bottom = graph.add_port(right, PortSide::West, Size::new(10.0, 10.0));
    graph.add_edge(EdgeEndpoint::port(left, left_top), EdgeEndpoint::port(right, right_top));
    graph.add_edge(EdgeEndpoint::port(left, left_bottom), EdgeEndpoint::port(right, right_bottom));
    graph
}

pub fn long_edge_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let b = graph.add_node(Size::new(80.0, 40.0));
    let c = graph.add_node(Size::new(80.0, 40.0));
    let d = graph.add_node(Size::new(80.0, 40.0));

    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
    let edge = graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(d));
    graph.add_edge(EdgeEndpoint::node(c), EdgeEndpoint::node(d));
    graph.add_edge_label(edge, "span", Size::new(42.0, 18.0));
    graph
}

pub fn back_edge_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let b = graph.add_node(Size::new(80.0, 40.0));
    let c = graph.add_node(Size::new(80.0, 40.0));
    graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
    graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
    graph.add_edge(EdgeEndpoint::node(c), EdgeEndpoint::node(a));
    graph
}

pub fn self_loop_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(Size::new(80.0, 40.0));
    let edge = graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(a));
    graph.add_edge_label(edge, "loop", Size::new(30.0, 18.0));
    graph
}

pub fn deep_dag_graph() -> Graph {
    let mut graph = Graph::new();
    let mut previous = None;
    for _ in 0..8 {
        let node = graph.add_node(Size::new(90.0, 40.0));
        if let Some(prev) = previous {
            graph.add_edge(EdgeEndpoint::node(prev), EdgeEndpoint::node(node));
        }
        previous = Some(node);
    }
    graph
}

pub fn sysml_block_definition_graph() -> Graph {
    let mut graph = Graph::new();
    let vehicle = graph.add_node(Size::new(180.0, 120.0));
    let power = graph.add_child_node(vehicle, Size::new(90.0, 50.0));
    let control = graph.add_child_node(vehicle, Size::new(90.0, 50.0));
    let interface = graph.add_node(Size::new(120.0, 60.0));

    graph.add_node_label(vehicle, "Vehicle", Size::new(60.0, 18.0));
    graph.add_node_label(power, "PowerUnit", Size::new(72.0, 18.0));
    graph.add_node_label(control, "Controller", Size::new(70.0, 18.0));
    graph.add_node_label(interface, "External IF", Size::new(72.0, 18.0));

    graph.node_mut(power).layout.layer_constraint = Some(LayerConstraint::First);
    graph.node_mut(control).layout.layer_constraint = Some(LayerConstraint::Last);
    graph.add_edge(EdgeEndpoint::node(power), EdgeEndpoint::node(control));
    graph.add_edge(EdgeEndpoint::node(control), EdgeEndpoint::node(interface));
    graph
}

pub fn sysml_internal_block_graph() -> Graph {
    let mut graph = Graph::new();
    let system = graph.add_node(Size::new(220.0, 180.0));
    let sensor = graph.add_child_node(system, Size::new(90.0, 50.0));
    let controller = graph.add_child_node(system, Size::new(100.0, 60.0));
    let gateway = graph.add_node(Size::new(120.0, 60.0));

    let sensor_out = graph.add_port(sensor, PortSide::East, Size::new(12.0, 12.0));
    let control_in = graph.add_port(controller, PortSide::West, Size::new(12.0, 12.0));
    let control_out = graph.add_port(controller, PortSide::East, Size::new(12.0, 12.0));
    let gateway_in = graph.add_port(gateway, PortSide::West, Size::new(12.0, 12.0));

    graph.port_mut(sensor_out).layout.port_constraint = Some(PortConstraint::FixedSide);
    graph.port_mut(control_in).layout.port_constraint = Some(PortConstraint::FixedOrder);
    graph.port_mut(control_out).layout.port_constraint = Some(PortConstraint::FixedOrder);
    graph.port_mut(gateway_in).layout.port_constraint = Some(PortConstraint::FixedSide);

    graph.add_port_label(sensor_out, "flow", Size::new(28.0, 14.0));
    graph.add_port_label(control_out, "cmd", Size::new(24.0, 14.0));

    graph.add_edge(EdgeEndpoint::port(sensor, sensor_out), EdgeEndpoint::port(controller, control_in));
    let cross = graph.add_edge(EdgeEndpoint::port(controller, control_out), EdgeEndpoint::port(gateway, gateway_in));
    graph.add_edge_label(cross, "connector", Size::new(56.0, 18.0));
    graph
}

pub fn general_view_small_graph() -> Graph {
    let mut graph = Graph::new();
    let mission = graph.add_node(Size::new(220.0, 160.0));
    let platform = graph.add_child_node(mission, Size::new(110.0, 60.0));
    let payload = graph.add_child_node(mission, Size::new(110.0, 60.0));
    let ops = graph.add_node(Size::new(140.0, 70.0));
    let support = graph.add_node(Size::new(140.0, 70.0));

    graph.add_node_label(mission, "Mission System", Size::new(92.0, 18.0));
    graph.add_node_label(platform, "Platform", Size::new(52.0, 18.0));
    graph.add_node_label(payload, "Payload", Size::new(48.0, 18.0));
    graph.add_node_label(ops, "Operations", Size::new(72.0, 18.0));
    graph.add_node_label(support, "Support", Size::new(56.0, 18.0));

    graph.node_mut(platform).layout.layer_constraint = Some(LayerConstraint::First);
    graph.node_mut(payload).layout.layer_constraint = Some(LayerConstraint::Last);
    graph.add_edge(EdgeEndpoint::node(platform), EdgeEndpoint::node(payload));
    graph
}

pub fn general_view_dense_graph() -> Graph {
    let mut graph = general_view_small_graph();
    let ext_a = graph.add_node(Size::new(140.0, 70.0));
    let ext_b = graph.add_node(Size::new(140.0, 70.0));
    graph.add_node_label(ext_a, "External A", Size::new(74.0, 18.0));
    graph.add_node_label(ext_b, "External B", Size::new(74.0, 18.0));
    graph
}

pub fn interconnection_view_small_graph() -> Graph {
    let mut graph = Graph::new();
    let bus = graph.add_node(Size::new(220.0, 180.0));
    let sensor = graph.add_child_node(bus, Size::new(90.0, 50.0));
    let controller = graph.add_child_node(bus, Size::new(100.0, 60.0));
    let gateway = graph.add_node(Size::new(120.0, 60.0));

    let s_out = graph.add_port(sensor, PortSide::East, Size::new(12.0, 12.0));
    let c_in = graph.add_port(controller, PortSide::West, Size::new(12.0, 12.0));
    let c_out = graph.add_port(controller, PortSide::East, Size::new(12.0, 12.0));
    let g_in = graph.add_port(gateway, PortSide::West, Size::new(12.0, 12.0));

    graph.port_mut(s_out).layout.port_constraint = Some(PortConstraint::FixedOrder);
    graph.port_mut(c_in).layout.port_constraint = Some(PortConstraint::FixedOrder);
    graph.port_mut(c_out).layout.port_constraint = Some(PortConstraint::FixedOrder);
    graph.port_mut(g_in).layout.port_constraint = Some(PortConstraint::FixedOrder);

    graph.add_node_label(bus, "Backbone", Size::new(58.0, 18.0));
    graph.add_port_label(s_out, "sample", Size::new(34.0, 14.0));
    graph.add_port_label(c_out, "command", Size::new(46.0, 14.0));
    graph.add_edge(EdgeEndpoint::port(sensor, s_out), EdgeEndpoint::port(controller, c_in));
    let cross = graph.add_edge(EdgeEndpoint::port(controller, c_out), EdgeEndpoint::port(gateway, g_in));
    graph.add_edge_label(cross, "uplink", Size::new(38.0, 16.0));
    graph
}

pub fn interconnection_view_dense_graph() -> Graph {
    let mut graph = interconnection_view_small_graph();
    let gateway = NodeId(3);
    let actuator = graph.add_node(Size::new(120.0, 60.0));
    let a_in = graph.add_port(actuator, PortSide::West, Size::new(12.0, 12.0));
    let g_out = graph.add_port(gateway, PortSide::East, Size::new(12.0, 12.0));
    let g_out2 = graph.add_port(gateway, PortSide::East, Size::new(12.0, 12.0));
    graph.add_port_label(a_in, "drive", Size::new(28.0, 14.0));
    graph.add_edge(EdgeEndpoint::port(gateway, g_out), EdgeEndpoint::port(actuator, a_in));
    let echo = graph.add_edge(EdgeEndpoint::port(gateway, g_out2), EdgeEndpoint::port(NodeId(1), elk_core::PortId(0)));
    graph.add_edge_label(echo, "feedback", Size::new(52.0, 16.0));
    graph
}

pub fn run_layered(graph: &mut Graph) -> elk_core::LayoutReport {
    run_layered_with_options(graph, LayoutOptions::default())
}

pub fn run_layered_with_direction(
    graph: &mut Graph,
    direction: LayoutDirection,
) -> elk_core::LayoutReport {
    let mut options = LayoutOptions::default();
    options.layered.direction = direction;
    run_layered_with_options(graph, options)
}

pub fn run_layered_general_view(graph: &mut Graph) -> elk_core::LayoutReport {
    run_layered_with_options(graph, LayoutOptions::default().with_view_profile(ViewProfile::GeneralView))
}

pub fn run_layered_interconnection_view(graph: &mut Graph) -> elk_core::LayoutReport {
    run_layered_with_options(graph, LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView))
}

pub fn run_layered_with_options(
    graph: &mut Graph,
    options: LayoutOptions,
) -> elk_core::LayoutReport {
    // Legacy entry point removed: layered now operates on ElkGraph.
    // Keep this function only for transitional compilation; it will be removed in follow-up edits.
    let mut elk = elk_graph_from_legacy(graph);
    elk_layered::layout(&mut elk, &options).expect("layered layout should succeed")
}

pub fn run_layered_elk_graph_with_options(
    graph: &mut elk_graph::ElkGraph,
    options: LayoutOptions,
) -> elk_core::LayoutReport {
    elk_layered::layout(graph, &options).expect("layered layout should succeed")
}

fn elk_graph_from_legacy(_graph: &Graph) -> ElkGraph {
    // Minimal placeholder conversion to unblock compilation while migrating tests.
    // The legacy `Graph`-based tests will be migrated to construct `ElkGraph` directly.
    ElkGraph::new()
}

pub fn assert_no_overlap(graph: &Graph, nodes: &[NodeId]) {
    for (index, left) in nodes.iter().enumerate() {
        for right in nodes.iter().skip(index + 1) {
            let left_bounds = graph.node(*left).bounds;
            let right_bounds = graph.node(*right).bounds;
            assert!(
                !left_bounds.intersects(right_bounds),
                "node {left:?} overlaps with node {right:?}: {left_bounds:?} vs {right_bounds:?}"
            );
        }
    }
}

pub fn assert_contains(outer: Rect, inner: Rect) {
    assert!(inner.origin.x >= outer.origin.x);
    assert!(inner.origin.y >= outer.origin.y);
    assert!(inner.max_x() <= outer.max_x());
    assert!(inner.max_y() <= outer.max_y());
}

pub fn assert_labels_outside_nodes(graph: &Graph) {
    for label in &graph.labels {
        let label_rect = Rect::new(label.position, label.size);
        for node in &graph.nodes {
            assert!(!label_rect.intersects(node.bounds), "label {:?} overlaps node {:?}", label.id, node.id);
        }
    }
}

pub fn assert_labels_do_not_overlap_ports(graph: &Graph) {
    for label in &graph.labels {
        let rect = Rect::new(label.position, label.size);
        for port in &graph.ports {
            assert!(!rect.intersects(port.bounds), "label {:?} overlaps port {:?}", label.id, port.id);
        }
    }
}

pub fn assert_edges_avoid_nodes(graph: &Graph) {
    for edge in &graph.edges {
        for section in &edge.sections {
            let points = section_points(section);
            for window in points.windows(2) {
                for node in &graph.nodes {
                    let allowed = node.id == edge.source.node
                        || node.id == edge.target.node
                        || is_ancestor(graph, node.id, edge.source.node)
                        || is_ancestor(graph, node.id, edge.target.node);
                    if allowed {
                        continue;
                    }
                    if segment_intersects_rect(window[0], window[1], node.bounds) {
                        let attached = window[0] == section.start || window[1] == section.end;
                        if !attached {
                            panic!("edge {:?} crosses node {:?}", edge.id, node.id);
                        }
                    }
                }
            }
        }
    }
}

pub fn assert_component_spacing(graph: &Graph, nodes: &[NodeId], min_gap: f32) {
    for (index, left) in nodes.iter().enumerate() {
        for right in nodes.iter().skip(index + 1) {
            let a = graph.node(*left).bounds;
            let b = graph.node(*right).bounds;
            let gap_x = if a.max_x() < b.origin.x {
                b.origin.x - a.max_x()
            } else if b.max_x() < a.origin.x {
                a.origin.x - b.max_x()
            } else {
                0.0
            };
            let gap_y = if a.max_y() < b.origin.y {
                b.origin.y - a.max_y()
            } else if b.max_y() < a.origin.y {
                a.origin.y - b.max_y()
            } else {
                0.0
            };
            assert!(gap_x >= min_gap || gap_y >= min_gap, "components {:?} and {:?} are too close", left, right);
        }
    }
}

pub fn assert_label_owner_proximity(graph: &Graph, max_distance: f32) {
    for node in &graph.nodes {
        let anchor = Point::new(node.bounds.origin.x + node.bounds.size.width / 2.0, node.bounds.origin.y);
        for label_id in &node.labels {
            let label = &graph.labels[label_id.index()];
            assert!(distance(anchor, label.position) <= max_distance, "node label {:?} is too far from owner {:?}", label.id, node.id);
        }
    }
}

pub fn assert_edge_bend_budget(graph: &Graph, max_bends: usize) {
    for edge in &graph.edges {
        for section in &edge.sections {
            assert!(section.bend_points.len() <= max_bends, "edge {:?} exceeded bend budget", edge.id);
        }
    }
}

pub fn section_points(section: &elk_core::EdgeSection) -> Vec<Point> {
    let mut points = vec![section.start];
    points.extend(section.bend_points.iter().copied());
    points.push(section.end);
    points
}

pub fn assert_baseline(report: &elk_core::LayoutReport, graph: &Graph, baseline: DifferentialBaseline) {
    assert!(report.stats.layers >= baseline.layers_range.0);
    assert!(report.stats.layers <= baseline.layers_range.1);
    assert!(report.stats.dummy_nodes >= baseline.min_dummy_nodes);
    assert!(report.stats.normalized_edges >= baseline.min_normalized_edges);
    assert!(report.stats.crossings_after <= baseline.max_crossings_after);
    assert!(report.stats.straight_segments >= baseline.min_straight_segments);
    assert!(graph.bounds.size.width >= baseline.width_range.0);
    assert!(graph.bounds.size.width <= baseline.width_range.1);
    assert!(graph.bounds.size.height >= baseline.height_range.0);
    assert!(graph.bounds.size.height <= baseline.height_range.1);
}

fn is_ancestor(graph: &Graph, ancestor: NodeId, node: NodeId) -> bool {
    let mut current = graph.node(node).parent;
    while let Some(parent) = current {
        if parent == ancestor {
            return true;
        }
        current = graph.node(parent).parent;
    }
    false
}

fn distance(a: Point, b: Point) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn segment_intersects_rect(start: Point, end: Point, rect: Rect) -> bool {
    if (start.x - end.x).abs() <= f32::EPSILON {
        let x = start.x;
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        x > rect.origin.x && x < rect.max_x() && max_y > rect.origin.y && min_y < rect.max_y()
    } else if (start.y - end.y).abs() <= f32::EPSILON {
        let y = start.y;
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        y > rect.origin.y && y < rect.max_y() && max_x > rect.origin.x && min_x < rect.max_x()
    } else {
        false
    }
}
