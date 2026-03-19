use std::collections::{BTreeMap, BTreeSet};

use elk_core::{EdgeRouting, LayoutOptions, LayoutStats, Point, PortSide, Rect, Size};
use elk_graph::{ElkGraph, EdgeId, NodeId, PortId};

use crate::ir::{IrEdge, LayeredIr};
use crate::pipeline::props::decode_layout_from_props;
use crate::pipeline::util::{
    dedup_points, endpoint_abs_center, endpoint_port_side, label_size, node_abs_origin,
    point_along_outward_normal,
};

pub(crate) fn export_to_graph(
    graph: &mut ElkGraph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
    warnings: &mut Vec<String>,
    stats: &mut LayoutStats,
) -> usize {
    // Write node positions/sizes.
    for node in &ir.nodes {
        if let crate::ir::IrNodeKind::Real(node_id) = node.kind {
            let n = &mut graph.nodes[node_id.index()];
            n.geometry.x = node.position.x;
            n.geometry.y = node.position.y;
            n.geometry.width = node.size.width;
            n.geometry.height = node.size.height;
            layout_ports(graph, node_id, options);
            layout_node_labels(graph, node_id, options);
        }
    }

    // Route edges: libavoid backend (if opted in) or simple 1-bend router.
    let use_libavoid = routing_backend_is_libavoid(graph);
    let mut routed = 0usize;

    if use_libavoid {
        let local_edge_ids: Vec<EdgeId> = ir.edges
            .iter()
            .filter(|e| local_nodes.contains(&e.effective_source) && local_nodes.contains(&e.effective_target))
            .map(|e| e.original_edge)
            .collect();
        if !local_edge_ids.is_empty() {
            if let Err(e) = elk_libavoid::route_edges(graph, &local_edge_ids) {
                warnings.push(format!("elk-layered: libavoid routing failed: {e}"));
            } else {
                routed = local_edge_ids.len();
                for edge in &ir.edges {
                    if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
                        continue;
                    }
                    let (start, end) = section_endpoints(graph, edge.original_edge);
                    for &sid in &graph.edges[edge.original_edge.index()].sections {
                        stats.bend_points += graph.edge_sections[sid.index()].bend_points.len();
                    }
                    place_edge_labels(graph, edge, start, end, options, stats);
                }
            }
        }
        if routed > 0 {
            warnings.push("elk-layered: libavoid routing backend active".to_string());
        }
    }

    let edge_lane_by_index = edge_lane_by_ir_index(ir);
    let keep_unnecessary_bends = keep_unnecessary_bends(graph);

    if !use_libavoid || routed == 0 {
        for edge in &ir.edges {
            if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
                continue;
            }
            let mut start = endpoint_abs_center(graph, edge.source);
            let mut end = endpoint_abs_center(graph, edge.target);
            apply_hierarchy_boundary_anchors(graph, edge, &mut start, &mut end);
            let routing = edge_routing_for_edge(graph, edge, options);
            let lane = edge_lane_by_index
                .get(&edge.original_edge.index())
                .copied()
                .unwrap_or(0);

            let mut bends = Vec::new();
            if routing == EdgeRouting::Orthogonal
                && (start.x - end.x).abs() > f32::EPSILON
                && (start.y - end.y).abs() > f32::EPSILON
            {
                let source_side = endpoint_port_side(graph, edge.source);
                let target_side = endpoint_port_side(graph, edge.target);
                const PORT_NORMAL_OFFSET: f32 = 8.0;
                if let (Some(ss), Some(ts)) = (source_side, target_side) {
                    // Attach orthogonally to both ports: first segment perpendicular to source,
                    // last segment perpendicular to target. Exit/entry are outside the nodes.
                    let exit = point_along_outward_normal(start, ss, PORT_NORMAL_OFFSET);
                    let entry = point_along_outward_normal(end, ts, PORT_NORMAL_OFFSET);
                    bends.push(exit);
                    bends.push(Point::new(entry.x, exit.y));
                    bends.push(entry);
                } else {
                    bends.extend(build_lane_orthogonal_bends(start, end, lane, options));
                }
            }
            bends = normalize_bends(bends, keep_unnecessary_bends);
            bends = correct_terminal_slants(start, end, bends);

            let edge_idx = edge.original_edge.index();
            graph.edges[edge_idx].sections.clear();
            let _ = graph.add_edge_section(edge.original_edge, start, bends.clone(), end);
            stats.bend_points += bends.len();
            routed += 1;
            place_edge_labels(graph, edge, start, end, options, stats);
        }
        if routed > 0 {
            warnings.push("elk-layered: simplified ElkGraph router active".to_string());
        }
    }

    routed
}

fn routing_backend_is_libavoid(graph: &ElkGraph) -> bool {
    let meta = elk_meta::default_registry();
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    let v = elk_alg_common::options::find_option(&meta, &by_key, "elk.layered.routingbackend");
    v.and_then(elk_graph::PropertyValue::as_str)
        .map(|s| s.trim().eq_ignore_ascii_case("libavoid"))
        .unwrap_or(false)
}

fn section_endpoints(graph: &ElkGraph, edge_id: EdgeId) -> (Point, Point) {
    let e = &graph.edges[edge_id.index()];
    if let (Some(&first_id), Some(&last_id)) = (e.sections.first(), e.sections.last()) {
        let first = &graph.edge_sections[first_id.index()];
        let last = &graph.edge_sections[last_id.index()];
        (first.start, last.end)
    } else {
        (Point::new(0.0, 0.0), Point::new(0.0, 0.0))
    }
}

fn node_rect(graph: &ElkGraph, node: NodeId) -> Rect {
    let o = node_abs_origin(graph, node);
    let n = &graph.nodes[node.index()];
    Rect::new(o, Size::new(n.geometry.width, n.geometry.height))
}

fn layout_ports(graph: &mut ElkGraph, node_id: NodeId, options: &LayoutOptions) {
    let bounds = node_rect(graph, node_id);
    let node = graph.nodes[node_id.index()].clone();
    let graph_defaults = decode_layout_from_props(&graph.properties);
    let node_options = decode_layout_from_props(&node.properties).inherit_from(&graph_defaults);
    let respect_port_order = node_options
        .respect_port_order
        .unwrap_or(options.layered.respect_port_order);

    // Group by side.
    let mut grouped: std::collections::BTreeMap<PortSide, Vec<PortId>> = std::collections::BTreeMap::new();
    for port_id in node.ports {
        let side = graph.ports[port_id.index()].side;
        grouped.entry(side).or_default().push(port_id);
    }

    for (side, mut ports) in grouped {
        if respect_port_order {
            ports.sort_by_key(|pid| {
                decode_layout_from_props(&graph.ports[pid.index()].properties)
                    .model_order
                    .unwrap_or(pid.index())
            });
        }
        let count = ports.len().max(1) as f32;
        for (index, port_id) in ports.into_iter().enumerate() {
            let fraction = (index as f32 + 1.0) / (count + 1.0);
            let size = Size::new(
                graph.ports[port_id.index()].geometry.width,
                graph.ports[port_id.index()].geometry.height,
            );
            let origin = match side {
                PortSide::North => Point::new(
                    bounds.origin.x + bounds.size.width * fraction - size.width / 2.0,
                    bounds.origin.y - size.height / 2.0,
                ),
                PortSide::South => Point::new(
                    bounds.origin.x + bounds.size.width * fraction - size.width / 2.0,
                    bounds.max_y() - size.height / 2.0,
                ),
                PortSide::East => Point::new(
                    bounds.max_x() - size.width / 2.0,
                    bounds.origin.y + bounds.size.height * fraction - size.height / 2.0,
                ),
                PortSide::West => Point::new(
                    bounds.origin.x - size.width / 2.0,
                    bounds.origin.y + bounds.size.height * fraction - size.height / 2.0,
                ),
            };
            let p = &mut graph.ports[port_id.index()];
            p.geometry.x = origin.x;
            p.geometry.y = origin.y;
        }
    }
}

fn layout_node_labels(graph: &mut ElkGraph, node_id: NodeId, options: &LayoutOptions) {
    let bounds = node_rect(graph, node_id);
    let node = graph.nodes[node_id.index()].clone();
    if node.labels.is_empty() {
        return;
    }

    let graph_defaults = decode_layout_from_props(&graph.properties);
    let node_options = decode_layout_from_props(&node.properties).inherit_from(&graph_defaults);
    let spacing = node_options
        .spacing
        .unwrap_or(options.layered.spacing)
        .label_spacing;

    let mut cursor = 0.0;
    for label_id in node.labels {
        let size = label_size(graph, label_id);
        let l = &mut graph.labels[label_id.index()];
        l.geometry.x = bounds.origin.x + (bounds.size.width - size.width) / 2.0;
        l.geometry.y = bounds.origin.y - size.height - spacing - cursor;
        cursor += size.height + spacing;
    }
}

fn edge_routing_for_edge(graph: &ElkGraph, edge: &IrEdge, options: &LayoutOptions) -> EdgeRouting {
    let e = &graph.edges[edge.original_edge.index()];
    decode_layout_from_props(&e.properties)
        .edge_routing
        .unwrap_or(options.layered.edge_routing)
}

fn place_edge_labels(
    graph: &mut ElkGraph,
    edge: &IrEdge,
    start: Point,
    end: Point,
    options: &LayoutOptions,
    _stats: &mut LayoutStats,
) {
    if edge.label_ids.is_empty() {
        return;
    }
    let mid = Point::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
    let spacing = options.layered.spacing.label_spacing;
    for label_id in &edge.label_ids {
        let size = label_size(graph, *label_id);
        let l = &mut graph.labels[label_id.index()];
        l.geometry.x = mid.x - size.width / 2.0;
        l.geometry.y = mid.y - size.height - spacing;
    }
}

fn edge_lane_by_ir_index(ir: &LayeredIr) -> BTreeMap<usize, i32> {
    let mut lane_sum: BTreeMap<usize, (i32, i32)> = BTreeMap::new();
    for ne in &ir.normalized_edges {
        let entry = lane_sum.entry(ne.original_edge.index()).or_insert((0, 0));
        entry.0 += ne.lane;
        entry.1 += 1;
    }
    lane_sum
        .into_iter()
        .map(|(edge_idx, (sum, cnt))| (edge_idx, if cnt == 0 { 0 } else { sum / cnt }))
        .collect()
}

fn build_lane_orthogonal_bends(
    start: Point,
    end: Point,
    lane: i32,
    options: &LayoutOptions,
) -> Vec<Point> {
    let lane_offset = lane as f32 * options.layered.spacing.segment_spacing.max(8.0);
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    if dx >= dy {
        let mid_x = (start.x + end.x) * 0.5 + lane_offset;
        vec![Point::new(mid_x, start.y), Point::new(mid_x, end.y)]
    } else {
        let mid_y = (start.y + end.y) * 0.5 + lane_offset;
        vec![Point::new(start.x, mid_y), Point::new(end.x, mid_y)]
    }
}

fn normalize_bends(mut bends: Vec<Point>, keep_unnecessary_bends: bool) -> Vec<Point> {
    bends = dedup_points(bends);
    if keep_unnecessary_bends {
        return bends;
    }
    if bends.len() <= 1 {
        return bends;
    }
    let mut out = Vec::with_capacity(bends.len());
    for (i, p) in bends.iter().enumerate() {
        if i == 0 || i + 1 == bends.len() {
            out.push(*p);
            continue;
        }
        let prev = bends[i - 1];
        let next = bends[i + 1];
        let same_x = (prev.x - p.x).abs() <= f32::EPSILON && (p.x - next.x).abs() <= f32::EPSILON;
        let same_y = (prev.y - p.y).abs() <= f32::EPSILON && (p.y - next.y).abs() <= f32::EPSILON;
        if !(same_x || same_y) {
            out.push(*p);
        }
    }
    dedup_points(out)
}

fn keep_unnecessary_bends(graph: &ElkGraph) -> bool {
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    for key in [
        "elk.layered.unnecessarybendpoints",
        "org.eclipse.elk.layered.unnecessaryBendpoints",
    ] {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            match v {
                elk_graph::PropertyValue::Bool(b) => return *b,
                elk_graph::PropertyValue::String(s) => {
                    let t = s.trim().to_ascii_lowercase();
                    if t == "true" {
                        return true;
                    }
                    if t == "false" {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    false
}

fn apply_hierarchy_boundary_anchors(
    graph: &ElkGraph,
    edge: &IrEdge,
    start: &mut Point,
    end: &mut Point,
) {
    // For cross-hierarchy edges without explicit ports, anchor at the container boundary
    // nearest to the opposite endpoint, mirroring Java layered's boundary-aware behavior.
    if edge.source.port.is_none() && edge.source.node != edge.effective_source {
        *start = boundary_anchor_towards(graph, edge.effective_source, *end);
    }
    if edge.target.port.is_none() && edge.target.node != edge.effective_target {
        *end = boundary_anchor_towards(graph, edge.effective_target, *start);
    }
}

fn boundary_anchor_towards(graph: &ElkGraph, node: NodeId, toward: Point) -> Point {
    let r = node_rect(graph, node);
    let center = Point::new(r.origin.x + r.size.width * 0.5, r.origin.y + r.size.height * 0.5);
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;
    if dx.abs() >= dy.abs() {
        if dx >= 0.0 {
            Point::new(r.max_x(), center.y)
        } else {
            Point::new(r.origin.x, center.y)
        }
    } else if dy >= 0.0 {
        Point::new(center.x, r.max_y())
    } else {
        Point::new(center.x, r.origin.y)
    }
}

fn correct_terminal_slants(start: Point, end: Point, bends: Vec<Point>) -> Vec<Point> {
    if bends.is_empty() {
        return bends;
    }
    let mut out = bends;
    let first = out[0];
    if (first.x - start.x).abs() > f32::EPSILON && (first.y - start.y).abs() > f32::EPSILON {
        out.insert(0, Point::new(first.x, start.y));
    }
    let last = *out.last().unwrap_or(&end);
    if (last.x - end.x).abs() > f32::EPSILON && (last.y - end.y).abs() > f32::EPSILON {
        out.push(Point::new(last.x, end.y));
    }
    dedup_points(out)
}

