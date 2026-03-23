use std::collections::{BTreeMap, BTreeSet};

use elk_core::{EdgeRouting, LayoutError, LayoutOptions, LayoutStats, Point, PortSide, Rect, Size};
use elk_graph::{ElkGraph, EdgeId, NodeId, PortId};
use elk_alg_common::orthogonal::{
    point_along_tangent, sanitize_orthogonal_path, simplify_orthogonal_points,
};

use crate::ir::{IrEdge, LayeredIr};
use crate::pipeline::orthogonal_routing_generator::{assign_routing_slots, HyperEdgeSegment};
use crate::pipeline::props::decode_layout_from_props;
use crate::pipeline::util::{
    dedup_points, endpoint_abs_center, endpoint_port_side, label_size, local_scope_frame,
    node_abs_origin, point_along_outward_normal,
};

const TERMINAL_NORMAL_OFFSET: f32 = 16.0;
const TERMINAL_TANGENT_SPACING: f32 = 18.0;
const SHARED_FANOUT_SPLIT_KEY: &str = "spec42.shared.fanout.split";

pub(crate) fn export_to_graph(
    graph: &mut ElkGraph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
    warnings: &mut Vec<String>,
    stats: &mut LayoutStats,
) -> Result<usize, LayoutError> {
    let debug_enabled = std::env::var_os("SPEC42_ELK_DEBUG").is_some();
    // Write node positions/sizes.
    for node in &ir.nodes {
        if let crate::ir::IrNodeKind::Real(node_id) = node.kind {
            let n = &mut graph.nodes[node_id.index()];
            let dx = node.position.x - n.geometry.x;
            let dy = node.position.y - n.geometry.y;
            n.geometry.x = node.position.x;
            n.geometry.y = node.position.y;
            n.geometry.width = node.size.width;
            n.geometry.height = node.size.height;
            if dx.abs() > f32::EPSILON || dy.abs() > f32::EPSILON {
                translate_descendant_routed_geometry(graph, node_id, dx, dy);
            }
            layout_ports(graph, node_id, options);
            layout_node_labels(graph, node_id, options);
        }
    }

    // Route edges with either the libavoid backend or the layered orthogonal router.
    let use_libavoid = should_use_libavoid(graph, options);
    let mut routed = 0usize;
    if !use_libavoid && matches!(options.view_profile, elk_core::ViewProfile::InterconnectionView) {
        warnings.push("elk-layered: libavoid not selected for interconnection; using layered orthogonal router".to_string());
    }

    if use_libavoid {
        let local_edge_ids: Vec<EdgeId> = ir.edges
            .iter()
            .filter(|e| local_nodes.contains(&e.effective_source) && local_nodes.contains(&e.effective_target))
            .map(|e| e.original_edge)
            .collect();
        if debug_enabled {
            let deferred_cross_hierarchy = ir
                .edges
                .iter()
                .filter(|e| {
                    local_nodes.contains(&e.effective_source) ^ local_nodes.contains(&e.effective_target)
                })
                .count();
            warnings.push(format!(
                "elk-layered: routing-scope local_nodes={} local_edges={} deferred_cross_hierarchy={}",
                local_nodes.len(),
                local_edge_ids.len(),
                deferred_cross_hierarchy
            ));
        }
        if !local_edge_ids.is_empty() {
            let scope_frame = local_scope_frame(graph, local_nodes);
            match elk_libavoid::route_edges_with_diagnostics_in_scope(
                graph,
                &local_edge_ids,
                scope_frame.origin_abs,
                Some(local_nodes),
            ) {
                Err(e) => {
                    warnings.push(format!("elk-layered: libavoid routing failed: {e}"));
                    return Err(LayoutError::Routing(format!(
                        "libavoid routing failed for local scope: {e}"
                    )));
                }
                Ok(diag_lines) => {
                    if debug_enabled {
                        warnings.extend(diag_lines.into_iter());
                    }
                routed = local_edge_ids.len();
                for edge in &ir.edges {
                    if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
                        continue;
                    }
                    let edge_ref = &graph.edges[edge.original_edge.index()];
                    let source_endpoint = edge_ref.sources.first().copied().unwrap_or(edge.routed_source);
                    let target_endpoint = edge_ref.targets.first().copied().unwrap_or(edge.routed_target);
                    orthogonalize_edge_sections_with_sides(
                        graph,
                        edge.original_edge,
                        endpoint_port_side(graph, source_endpoint),
                        endpoint_port_side(graph, target_endpoint),
                    );
                    if debug_enabled {
                        warnings.push(format!(
                            "elk-layered: edge-scope edge={:?} src_scope={:?} dst_scope={:?} src_frame={} dst_frame={}",
                            edge.original_edge,
                            edge.effective_source,
                            edge.effective_target,
                            endpoint_frame_debug(graph, source_endpoint),
                            endpoint_frame_debug(graph, target_endpoint)
                        ));
                    }
                    let (start, end) = section_endpoints(graph, edge.original_edge);
                    for &sid in &graph.edges[edge.original_edge.index()].sections {
                        stats.bend_points += graph.edge_sections[sid.index()].bend_points.len();
                    }
                    place_edge_labels(graph, edge, start, end, options, stats);
                }
                }
            }
        }
        if routed > 0 {
            warnings.push("elk-layered: libavoid routing backend active".to_string());
        }
    }

    let edge_lane_by_index = edge_lane_by_ir_index(ir);
    let fanout_lane_bias_by_index = fanout_lane_bias_by_edge_index(ir);
    let keep_unnecessary_bends = keep_unnecessary_bends(graph);
    assign_endpoint_slots(graph, ir, local_nodes, options);
    assign_shared_fanout_splits(graph, ir, local_nodes, options);

    if !use_libavoid {
        for edge in &ir.edges {
            if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
                continue;
            }
            let edge_ref = &graph.edges[edge.original_edge.index()];
            let source_endpoint = edge_ref.sources.first().copied().unwrap_or(edge.routed_source);
            let target_endpoint = edge_ref.targets.first().copied().unwrap_or(edge.routed_target);
            let mut start = endpoint_abs_center(graph, source_endpoint);
            let mut end = endpoint_abs_center(graph, target_endpoint);
            apply_hierarchy_boundary_anchors(graph, edge, &mut start, &mut end);
            let routing = edge_routing_for_edge(graph, edge, options);
            let lane = edge_lane_by_index
                .get(&edge.original_edge.index())
                .copied()
                .unwrap_or(0)
                + edge_bundle_lane_offset(graph, edge.original_edge)
                + fanout_lane_bias_by_index
                    .get(&edge.original_edge.index())
                    .copied()
                    .unwrap_or(0);

            let path = if routing == EdgeRouting::Orthogonal {
                build_orthogonal_route_path(
                    graph,
                    edge,
                    source_endpoint,
                    target_endpoint,
                    start,
                    end,
                    lane,
                    options,
                )?
            } else {
                vec![start, end]
            };

            let edge_idx = edge.original_edge.index();
            graph.edges[edge_idx].sections.clear();
            let bends = if path.len() >= 2 {
                path[1..path.len() - 1].to_vec()
            } else {
                Vec::new()
            };
            let start = path.first().copied().unwrap_or(start);
            let end = path.last().copied().unwrap_or(end);
            let bends = normalize_bends(bends, keep_unnecessary_bends);
            let _ = graph.add_edge_section(edge.original_edge, start, bends.clone(), end);
            restore_nested_endpoint_terminals(graph, edge);
            let (start, end) = section_endpoints(graph, edge.original_edge);
            stats.bend_points += bends.len();
            routed += 1;
            place_edge_labels(graph, edge, start, end, options, stats);
        }
        if routed > 0 {
            warnings.push("elk-layered: layered orthogonal routing backend active".to_string());
        }
    }

    Ok(routed)
}

fn translate_descendant_routed_geometry(graph: &mut ElkGraph, root: NodeId, dx: f32, dy: f32) {
    for edge_idx in 0..graph.edges.len() {
        let translate = {
            let edge = &graph.edges[edge_idx];
            edge.sections.iter().any(|_| {
                edge.sources.first().is_some_and(|source| is_descendant_of(graph, source.node, root))
                    && edge
                        .targets
                        .first()
                        .is_some_and(|target| is_descendant_of(graph, target.node, root))
            })
        };
        if !translate {
            continue;
        }

        let section_ids = graph.edges[edge_idx].sections.clone();
        for section_id in section_ids {
            let section = &mut graph.edge_sections[section_id.index()];
            section.start.x += dx;
            section.start.y += dy;
            section.end.x += dx;
            section.end.y += dy;
            for bend in &mut section.bend_points {
                bend.x += dx;
                bend.y += dy;
            }
        }

        let label_ids = graph.edges[edge_idx].labels.clone();
        for label_id in label_ids {
            let label = &mut graph.labels[label_id.index()].geometry;
            label.x += dx;
            label.y += dy;
        }
    }
}

fn is_descendant_of(graph: &ElkGraph, node_id: NodeId, ancestor: NodeId) -> bool {
    let mut current = Some(node_id);
    while let Some(node_id) = current {
        if node_id == ancestor {
            return true;
        }
        current = graph.nodes[node_id.index()].parent;
    }
    false
}

fn restore_nested_endpoint_terminals(graph: &mut ElkGraph, edge: &IrEdge) {
    let edge_ref = &graph.edges[edge.original_edge.index()];
    let Some(first_id) = edge_ref.sections.first().copied() else {
        return;
    };
    let Some(last_id) = edge_ref.sections.last().copied() else {
        return;
    };

    if edge.source.port.is_none() && edge.source.node != edge.effective_source {
        let start = endpoint_abs_center(graph, edge.source);
        set_section_start_preserve_orthogonality(&mut graph.edge_sections[first_id.index()], start);
    }
    if edge.target.port.is_none() && edge.target.node != edge.effective_target {
        let end = endpoint_abs_center(graph, edge.target);
        set_section_end_preserve_orthogonality(&mut graph.edge_sections[last_id.index()], end);
    }
}

fn set_section_start_preserve_orthogonality(section: &mut elk_graph::EdgeSection, start: Point) {
    section.start = start;
    if section.bend_points.is_empty() {
        if (section.end.x - start.x).abs() > f32::EPSILON && (section.end.y - start.y).abs() > f32::EPSILON {
            section.bend_points.push(Point::new(section.end.x, start.y));
        }
        return;
    }
    let first = section.bend_points[0];
    if (first.x - start.x).abs() > f32::EPSILON && (first.y - start.y).abs() > f32::EPSILON {
        let dx = (first.x - start.x).abs();
        let dy = (first.y - start.y).abs();
        section.bend_points[0] = if dx <= dy {
            Point::new(start.x, first.y)
        } else {
            Point::new(first.x, start.y)
        };
    }
}

fn set_section_end_preserve_orthogonality(section: &mut elk_graph::EdgeSection, end: Point) {
    section.end = end;
    if section.bend_points.is_empty() {
        if (section.start.x - end.x).abs() > f32::EPSILON && (section.start.y - end.y).abs() > f32::EPSILON {
            section.bend_points.push(Point::new(section.start.x, end.y));
        }
        return;
    }
    let last_idx = section.bend_points.len() - 1;
    let last = section.bend_points[last_idx];
    if (last.x - end.x).abs() > f32::EPSILON && (last.y - end.y).abs() > f32::EPSILON {
        let dx = (last.x - end.x).abs();
        let dy = (last.y - end.y).abs();
        section.bend_points[last_idx] = if dx <= dy {
            Point::new(end.x, last.y)
        } else {
            Point::new(last.x, end.y)
        };
    }
}

fn orthogonalize_polyline(
    points: Vec<Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<Point> {
    const EPS: f32 = 1e-5;
    if points.len() < 2 {
        return points;
    }
    let mut out = vec![points[0]];
    for idx in 0..points.len() - 1 {
        let a = *out.last().unwrap_or(&points[idx]);
        let b = points[idx + 1];
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        if dx <= EPS && dy <= EPS {
            continue;
        }
        if dx > EPS && dy > EPS {
            let via = choose_orthogonal_elbow(&points, idx, a, b, start_side, end_side);
            if out.last().copied() != Some(via) {
                out.push(via);
            }
        }
        if out.last().copied() != Some(b) {
            out.push(b);
        }
    }
    simplify_orthogonal_points(out)
}

fn choose_orthogonal_elbow(
    points: &[Point],
    idx: usize,
    a: Point,
    b: Point,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Point {
    if idx == 0 {
        if let Some(side) = start_side {
            return match side {
                PortSide::East | PortSide::West => Point::new(b.x, a.y),
                PortSide::North | PortSide::South => Point::new(a.x, b.y),
            };
        }
    }
    if idx + 1 == points.len() - 1 {
        if let Some(side) = end_side {
            return match side {
                PortSide::East | PortSide::West => Point::new(a.x, b.y),
                PortSide::North | PortSide::South => Point::new(b.x, a.y),
            };
        }
    }

    if idx > 0 {
        let prev = points[idx - 1];
        if (prev.x - a.x).abs() <= f32::EPSILON {
            return Point::new(a.x, b.y);
        }
        if (prev.y - a.y).abs() <= f32::EPSILON {
            return Point::new(b.x, a.y);
        }
    }

    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    if dx >= dy {
        Point::new(b.x, a.y)
    } else {
        Point::new(a.x, b.y)
    }
}

fn endpoint_frame_debug(graph: &ElkGraph, endpoint: elk_graph::EdgeEndpoint) -> String {
    match endpoint.port {
        Some(port_id) => {
            let p = &graph.ports[port_id.index()];
            let n = &graph.nodes[p.node.index()];
            let abs = endpoint_abs_center(graph, endpoint);
            format!(
                "port(node={:?},port={:?},node_xy=({:.1},{:.1}),raw=({:.1},{:.1}),abs=({:.1},{:.1}))",
                p.node,
                port_id,
                n.geometry.x,
                n.geometry.y,
                p.geometry.x + p.geometry.width / 2.0,
                p.geometry.y + p.geometry.height / 2.0,
                abs.x,
                abs.y
            )
        }
        None => {
            let n = &graph.nodes[endpoint.node.index()];
            let abs = endpoint_abs_center(graph, endpoint);
            format!(
                "node(node={:?},raw=({:.1},{:.1}),abs=({:.1},{:.1}))",
                endpoint.node,
                n.geometry.x + n.geometry.width / 2.0,
                n.geometry.y + n.geometry.height / 2.0,
                abs.x,
                abs.y
            )
        }
    }
}

fn fanout_lane_bias_by_edge_index(ir: &LayeredIr) -> BTreeMap<usize, i32> {
    let mut groups: BTreeMap<(NodeId, Option<u32>), Vec<(usize, usize, usize)>> = BTreeMap::new();
    for edge in &ir.edges {
        groups
            .entry((edge.effective_source, edge.bundle_key))
            .or_default()
            .push((
                edge.original_edge.index(),
                edge.model_order,
                edge.original_edge.index(),
            ));
    }

    let mut out = BTreeMap::new();
    for mut edges in groups.into_values() {
        if edges.len() <= 1 {
            continue;
        }
        edges.sort_by_key(|(_, model_order, edge_idx)| (*model_order, *edge_idx));
        let center = (edges.len() as i32 - 1) / 2;
        for (i, (edge_idx, _, _)) in edges.into_iter().enumerate() {
            out.insert(edge_idx, i as i32 - center);
        }
    }
    merge_adjacent_fanout_biases(out)
}

fn merge_adjacent_fanout_biases(mut biases: BTreeMap<usize, i32>) -> BTreeMap<usize, i32> {
    // Hyperedge-dummy-merger style smoothing: collapse near-identical adjacent fanout offsets
    // so dense bundles share trunks longer before splitting.
    let mut entries: Vec<(usize, i32)> = biases.iter().map(|(k, v)| (*k, *v)).collect();
    entries.sort_by_key(|(edge_idx, _)| *edge_idx);
    for i in 1..entries.len() {
        let prev = entries[i - 1].1;
        let cur = entries[i].1;
        if (cur - prev).abs() <= 1 {
            entries[i].1 = prev;
        }
    }
    biases.clear();
    for (k, v) in entries {
        biases.insert(k, v);
    }
    biases
}

pub(crate) fn should_use_libavoid(graph: &ElkGraph, options: &LayoutOptions) -> bool {
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    let backend = by_key
        .get("elk.layered.routingbackend")
        .or_else(|| by_key.get("org.eclipse.elk.layered.routingbackend"))
        .and_then(|v| elk_graph::PropertyValue::as_str(v))
        .map(|s| s.trim().to_ascii_lowercase());
    match backend.as_deref() {
        Some("libavoid") => true,
        Some("default") | Some("simple") | Some("elkgraph") => false,
        Some(_) => false,
        None => matches!(options.view_profile, elk_core::ViewProfile::InterconnectionView),
    }
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

pub(crate) fn refresh_all_port_positions(graph: &mut ElkGraph, options: &LayoutOptions) {
    let node_ids: Vec<NodeId> = graph
        .nodes
        .iter()
        .map(|n| n.id)
        .filter(|id| *id != graph.root)
        .collect();
    for node_id in node_ids {
        layout_ports(graph, node_id, options);
    }
}

fn orthogonalize_edge_sections_with_sides(
    graph: &mut ElkGraph,
    edge_id: EdgeId,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
) {
    let section_ids = graph.edges[edge_id.index()].sections.clone();
    let last_section_idx = section_ids.len().saturating_sub(1);
    for (section_idx, sid) in section_ids.into_iter().enumerate() {
        let section = &graph.edge_sections[sid.index()];
        let points: Vec<Point> = std::iter::once(section.start)
            .chain(section.bend_points.iter().copied())
            .chain(std::iter::once(section.end))
            .collect();
        let orthogonal = orthogonalize_polyline(
            points,
            if section_idx == 0 { source_side } else { None },
            if section_idx == last_section_idx {
                target_side
            } else {
                None
            },
        );
        let orthogonal = ensure_terminal_normals(
            orthogonal,
            if section_idx == 0 { source_side } else { None },
            if section_idx == last_section_idx {
                target_side
            } else {
                None
            },
        );
        if orthogonal.len() < 2 {
            continue;
        }
        let section_mut = &mut graph.edge_sections[sid.index()];
        section_mut.start = orthogonal[0];
        section_mut.end = *orthogonal.last().unwrap_or(&orthogonal[0]);
        section_mut.bend_points = orthogonal[1..orthogonal.len() - 1].to_vec();
    }
}

fn ensure_terminal_normals(
    mut points: Vec<Point>,
    start_side: Option<PortSide>,
    end_side: Option<PortSide>,
) -> Vec<Point> {
    if let Some(side) = start_side {
        points = ensure_start_terminal_normal(points, side);
    }
    if let Some(side) = end_side {
        points = ensure_end_terminal_normal(points, side);
    }
    simplify_orthogonal_points(points)
}

fn ensure_start_terminal_normal(points: Vec<Point>, side: PortSide) -> Vec<Point> {
    const EPS: f32 = 1e-5;
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    if points.len() < 2 {
        return points;
    }
    let start = points[0];
    let next = points[1];
    if terminal_matches_side(start, next, side) {
        return points;
    }
    let route = point_along_outward_normal(start, side, PORT_NORMAL_OFFSET);
    let mut rebuilt = Vec::with_capacity(points.len() + 2);
    rebuilt.push(start);
    if rebuilt.last().copied() != Some(route) {
        rebuilt.push(route);
    }
    if route != next {
        let bridge = match side {
            PortSide::East | PortSide::West => {
                let bridge_y = if (next.y - route.y).abs() > EPS {
                    next.y
                } else {
                    points.get(2).map(|point| point.y).unwrap_or(route.y)
                };
                Point::new(route.x, bridge_y)
            }
            PortSide::North | PortSide::South => {
                let bridge_x = if (next.x - route.x).abs() > EPS {
                    next.x
                } else {
                    points.get(2).map(|point| point.x).unwrap_or(route.x)
                };
                Point::new(bridge_x, route.y)
            }
        };
        if bridge != route && bridge != next {
            rebuilt.push(bridge);
        }
    }
    rebuilt.extend(points.into_iter().skip(1));
    rebuilt
}

fn ensure_end_terminal_normal(points: Vec<Point>, side: PortSide) -> Vec<Point> {
    const EPS: f32 = 1e-5;
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    if points.len() < 2 {
        return points;
    }
    let end = *points.last().unwrap_or(&Point::new(0.0, 0.0));
    let prev = points[points.len() - 2];
    if terminal_matches_side(end, prev, side) {
        return points;
    }
    let route = point_along_outward_normal(end, side, PORT_NORMAL_OFFSET);
    let mut rebuilt = Vec::with_capacity(points.len() + 2);
    rebuilt.extend(points.iter().copied().take(points.len() - 1));
    if rebuilt.last().copied() != Some(route) {
        let bridge = match side {
            PortSide::East | PortSide::West => {
                let bridge_y = if (prev.y - route.y).abs() > EPS {
                    prev.y
                } else {
                    points
                        .get(points.len().saturating_sub(3))
                        .map(|point| point.y)
                        .unwrap_or(route.y)
                };
                Point::new(route.x, bridge_y)
            }
            PortSide::North | PortSide::South => {
                let bridge_x = if (prev.x - route.x).abs() > EPS {
                    prev.x
                } else {
                    points
                        .get(points.len().saturating_sub(3))
                        .map(|point| point.x)
                        .unwrap_or(route.x)
                };
                Point::new(bridge_x, route.y)
            }
        };
        if rebuilt.last().copied() != Some(bridge) && bridge != route && bridge != end {
            rebuilt.push(bridge);
        }
        rebuilt.push(route);
    }
    rebuilt.push(end);
    rebuilt
}

fn terminal_matches_side(endpoint: Point, neighbor: Point, side: PortSide) -> bool {
    match side {
        PortSide::East => (neighbor.y - endpoint.y).abs() <= 1e-5 && neighbor.x >= endpoint.x - 1e-5,
        PortSide::West => (neighbor.y - endpoint.y).abs() <= 1e-5 && neighbor.x <= endpoint.x + 1e-5,
        PortSide::North => (neighbor.x - endpoint.x).abs() <= 1e-5 && neighbor.y <= endpoint.y + 1e-5,
        PortSide::South => (neighbor.x - endpoint.x).abs() <= 1e-5 && neighbor.y >= endpoint.y - 1e-5,
    }
}

fn layout_ports(graph: &mut ElkGraph, node_id: NodeId, options: &LayoutOptions) {
    let node = graph.nodes[node_id.index()].clone();
    let node_width = node.geometry.width;
    let node_height = node.geometry.height;
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
                    node_width * fraction - size.width / 2.0,
                    -size.height / 2.0,
                ),
                PortSide::South => Point::new(
                    node_width * fraction - size.width / 2.0,
                    node_height - size.height / 2.0,
                ),
                PortSide::East => Point::new(
                    node_width - size.width / 2.0,
                    node_height * fraction - size.height / 2.0,
                ),
                PortSide::West => Point::new(
                    -size.width / 2.0,
                    node_height * fraction - size.height / 2.0,
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
    let mut by_edge: BTreeMap<usize, Vec<(usize, i32)>> = BTreeMap::new();
    for ne in &ir.normalized_edges {
        by_edge
            .entry(ne.original_edge.index())
            .or_default()
            .push((ne.segment_order, ne.lane));
    }
    let base_lanes: BTreeMap<usize, i32> = by_edge
        .into_iter()
        .map(|(edge_idx, mut lanes)| {
            lanes.sort_by_key(|(segment_order, lane)| (*segment_order, *lane));
            let primary_lane = lanes.first().map(|(_, lane)| *lane).unwrap_or(0);
            let mut only_lanes: Vec<i32> = lanes.iter().map(|(_, lane)| *lane).collect();
            only_lanes.sort_unstable();
            let median_lane = only_lanes
                .get(only_lanes.len() / 2)
                .copied()
                .unwrap_or(primary_lane);
            lanes.sort_unstable();
            let lane = if (primary_lane - median_lane).abs() <= 1 {
                primary_lane
            } else {
                median_lane
            };
            (edge_idx, lane)
        })
        .collect();
    apply_orthogonal_slot_refinement(base_lanes, ir)
}

fn apply_orthogonal_slot_refinement(
    base_lanes: BTreeMap<usize, i32>,
    ir: &LayeredIr,
) -> BTreeMap<usize, i32> {
    if base_lanes.len() <= 2 {
        return base_lanes;
    }
    let mut edge_order: Vec<usize> = base_lanes.keys().copied().collect();
    edge_order.sort_unstable();
    let mut segments = Vec::with_capacity(edge_order.len());
    for (seg_id, edge_idx) in edge_order.iter().copied().enumerate() {
        let lane = base_lanes.get(&edge_idx).copied().unwrap_or_default();
        let model_order = ir
            .edges
            .iter()
            .find(|e| e.original_edge.index() == edge_idx)
            .map(|e| e.model_order as f32)
            .unwrap_or(edge_idx as f32);
        segments.push(HyperEdgeSegment {
            id: seg_id,
            start_coordinate: lane as f32,
            end_coordinate: lane as f32 + 0.001,
            incoming_connection_coordinates: vec![model_order],
            outgoing_connection_coordinates: vec![edge_idx as f32],
            routing_slot: 0,
            in_weight: 0,
            out_weight: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            split_partner: None,
            split_by: None,
            mark: -1,
        });
    }
    let slots = assign_routing_slots(segments, 1.0);
    let mut out = BTreeMap::new();
    for (seg_id, edge_idx) in edge_order.into_iter().enumerate() {
        let base = base_lanes.get(&edge_idx).copied().unwrap_or_default();
        let refined = slots.get(seg_id).copied().unwrap_or(0);
        out.insert(edge_idx, base + refined);
    }
    out
}

fn build_orthogonal_route_path(
    graph: &ElkGraph,
    edge: &IrEdge,
    source_endpoint: elk_graph::EdgeEndpoint,
    target_endpoint: elk_graph::EdgeEndpoint,
    start: Point,
    end: Point,
    lane: i32,
    options: &LayoutOptions,
) -> Result<Vec<Point>, LayoutError> {
    let source_side = endpoint_side_for_routing(graph, source_endpoint, end, options.layered.direction, true);
    let target_side = endpoint_side_for_routing(graph, target_endpoint, start, options.layered.direction, false);
    let start = endpoint_anchor_for_routing(
        graph,
        source_endpoint,
        source_side,
        endpoint_slot(graph, edge.original_edge, true),
    );
    let end = endpoint_anchor_for_routing(
        graph,
        target_endpoint,
        target_side,
        endpoint_slot(graph, edge.original_edge, false),
    );
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Ok(vec![start, end]);
    }

    let source_slot = endpoint_slot(graph, edge.original_edge, true);
    let target_slot = endpoint_slot(graph, edge.original_edge, false);
    let source_lead = point_along_outward_normal(start, source_side, terminal_lead_for_slot(source_slot));
    let target_lead = point_along_outward_normal(end, target_side, terminal_lead_for_slot(target_slot));
    let route_start = point_along_tangent(
        source_lead,
        source_side,
        tangent_route_offset(source_endpoint, source_slot),
    );
    let route_end = point_along_tangent(
        target_lead,
        target_side,
        tangent_route_offset(target_endpoint, target_slot),
    );

    let shared_fanout_split = graph.edges[edge.original_edge.index()]
        .properties
        .get(&elk_graph::PropertyKey::from(SHARED_FANOUT_SPLIT_KEY))
        .and_then(|value| value.as_f64())
        .map(|value| value as f32);
    let trunk = shared_fanout_split
        .and_then(|split| build_shared_fanout_trunk(route_start, route_end, source_side, target_side, split))
        .unwrap_or_else(|| build_layered_orthogonal_trunk(route_start, route_end, lane, options));
    let route_points = std::iter::once(route_start)
        .chain(trunk)
        .chain(std::iter::once(route_end))
        .collect::<Vec<_>>();

    sanitize_orthogonal_path(
        route_points,
        start,
        end,
        source_lead,
        target_lead,
        route_start,
        route_end,
    )
    .map_err(LayoutError::Routing)
}

fn endpoint_slot(graph: &ElkGraph, edge_id: EdgeId, is_source: bool) -> i32 {
    let key = elk_graph::PropertyKey::from(if is_source {
        "spec42.endpoint.slot.source"
    } else {
        "spec42.endpoint.slot.target"
    });
    graph.edges[edge_id.index()]
        .properties
        .get(&key)
        .and_then(|value| value.as_i64())
        .unwrap_or(0) as i32
}

fn terminal_lead_for_slot(slot: i32) -> f32 {
    TERMINAL_NORMAL_OFFSET + slot.abs() as f32 * (TERMINAL_TANGENT_SPACING * 0.75)
}

fn build_layered_orthogonal_trunk(
    start: Point,
    end: Point,
    lane: i32,
    options: &LayoutOptions,
) -> Vec<Point> {
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Vec::new();
    }

    let mut bends = build_lane_orthogonal_bends(start, end, lane, options);

    // Degenerate orthogonal trunks still need one elbow.
    if bends.is_empty() {
        bends.extend(default_elbow(start, end));
    }
    bends
}

fn build_shared_fanout_trunk(
    start: Point,
    end: Point,
    source_side: PortSide,
    target_side: PortSide,
    split: f32,
) -> Option<Vec<Point>> {
    let bends = match (source_side, target_side) {
        (PortSide::South, PortSide::North) | (PortSide::North, PortSide::South) => {
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);
            if split <= min_y || split >= max_y {
                return None;
            }
            vec![Point::new(start.x, split), Point::new(end.x, split)]
        }
        (PortSide::East, PortSide::West) | (PortSide::West, PortSide::East) => {
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            if split <= min_x || split >= max_x {
                return None;
            }
            vec![Point::new(split, start.y), Point::new(split, end.y)]
        }
        _ => return None,
    };
    Some(normalize_bends(bends, false))
}

fn tangent_route_offset(endpoint: elk_graph::EdgeEndpoint, slot: i32) -> f32 {
    let _ = (endpoint, slot);
    0.0
}

fn endpoint_side_for_routing(
    graph: &ElkGraph,
    endpoint: elk_graph::EdgeEndpoint,
    toward: Point,
    direction: elk_core::LayoutDirection,
    is_source: bool,
) -> PortSide {
    endpoint_port_side(graph, endpoint)
        .unwrap_or_else(|| infer_node_side_for_target(graph, endpoint.node, toward, direction, is_source))
}

fn infer_node_side_for_target(
    _graph: &ElkGraph,
    _node_id: NodeId,
    _toward: Point,
    direction: elk_core::LayoutDirection,
    is_source: bool,
) -> PortSide {
    match direction {
        elk_core::LayoutDirection::TopToBottom => if is_source { PortSide::South } else { PortSide::North },
        elk_core::LayoutDirection::BottomToTop => if is_source { PortSide::North } else { PortSide::South },
        elk_core::LayoutDirection::LeftToRight => if is_source { PortSide::East } else { PortSide::West },
        elk_core::LayoutDirection::RightToLeft => if is_source { PortSide::West } else { PortSide::East },
    }
}

fn endpoint_anchor_for_routing(
    graph: &ElkGraph,
    endpoint: elk_graph::EdgeEndpoint,
    side: PortSide,
    slot: i32,
) -> Point {
    if let Some(port_id) = endpoint.port {
        let center = endpoint_abs_center(graph, endpoint);
        let port = &graph.ports[port_id.index()];
        let origin = node_abs_origin(graph, port.node);
        let node = &graph.nodes[port.node.index()];
        let tangent = slot as f32 * TERMINAL_TANGENT_SPACING;
        let margin = 12.0;
        let min_x = origin.x + margin;
        let max_x = (origin.x + node.geometry.width - margin).max(min_x);
        let min_y = origin.y + margin;
        let max_y = (origin.y + node.geometry.height - margin).max(min_y);

        return match side {
            PortSide::North | PortSide::South => {
                Point::new((center.x + tangent).clamp(min_x, max_x), center.y)
            }
            PortSide::East | PortSide::West => {
                Point::new(center.x, (center.y + tangent).clamp(min_y, max_y))
            }
        };
    }

    let node = &graph.nodes[endpoint.node.index()];
    let origin = node_abs_origin(graph, endpoint.node);
    let center = Point::new(
        origin.x + node.geometry.width / 2.0,
        origin.y + node.geometry.height / 2.0,
    );
    let tangent = slot as f32 * TERMINAL_TANGENT_SPACING;
    let margin = 12.0;
    let min_x = origin.x + margin;
    let max_x = (origin.x + node.geometry.width - margin).max(min_x);
    let min_y = origin.y + margin;
    let max_y = (origin.y + node.geometry.height - margin).max(min_y);

    match side {
        PortSide::North => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y),
        PortSide::South => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y + node.geometry.height),
        PortSide::East => Point::new(origin.x + node.geometry.width, (center.y + tangent).clamp(min_y, max_y)),
        PortSide::West => Point::new(origin.x, (center.y + tangent).clamp(min_y, max_y)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EndpointGroup {
    endpoint_kind: u8,
    endpoint_index: usize,
    side: u8,
    bundle_key: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GroupSortKey {
    endpoint_position: i64,
    opposite_position: i64,
    bundle_key: Option<u32>,
    model_order: usize,
    edge_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FanoutGroupKey {
    endpoint_kind: u8,
    endpoint_index: usize,
    source_side: u8,
    target_side: u8,
    bundle_key: Option<u32>,
    source_layer: usize,
    target_layer: usize,
}

fn assign_endpoint_slots(
    graph: &mut ElkGraph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
) {
    let mut source_group_by_edge = BTreeMap::new();
    let mut target_group_by_edge = BTreeMap::new();
    let mut groups_by_side: BTreeMap<(NodeId, PortSide), BTreeMap<EndpointGroup, GroupSortKey>> =
        BTreeMap::new();

    for edge in &ir.edges {
        if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
            continue;
        }
        let edge_ref = &graph.edges[edge.original_edge.index()];
        let source_endpoint = edge_ref.sources.first().copied().unwrap_or(edge.routed_source);
        let target_endpoint = edge_ref.targets.first().copied().unwrap_or(edge.routed_target);
        let source_side = endpoint_side_for_routing(
            graph,
            source_endpoint,
            endpoint_abs_center(graph, target_endpoint),
            options.layered.direction,
            true,
        );
        let target_side = endpoint_side_for_routing(
            graph,
            target_endpoint,
            endpoint_abs_center(graph, source_endpoint),
            options.layered.direction,
            false,
        );

        let source_group = endpoint_group(source_endpoint, source_side, edge.bundle_key);
        let target_group = endpoint_group(target_endpoint, target_side, edge.bundle_key);
        let source_sort = endpoint_group_sort_key(
            graph,
            source_endpoint,
            target_endpoint,
            source_side,
            edge,
        );
        let target_sort = endpoint_group_sort_key(
            graph,
            target_endpoint,
            source_endpoint,
            target_side,
            edge,
        );

        groups_by_side
            .entry((source_endpoint.node, source_side))
            .or_default()
            .entry(source_group.clone())
            .and_modify(|existing| *existing = (*existing).min(source_sort))
            .or_insert(source_sort);
        groups_by_side
            .entry((target_endpoint.node, target_side))
            .or_default()
            .entry(target_group.clone())
            .and_modify(|existing| *existing = (*existing).min(target_sort))
            .or_insert(target_sort);

        source_group_by_edge.insert(edge.original_edge, source_group);
        target_group_by_edge.insert(edge.original_edge, target_group);
    }

    let mut slot_by_group = BTreeMap::new();
    for groups in groups_by_side.values() {
        let mut ordered = groups
            .iter()
            .map(|(group, sort_key)| (group.clone(), *sort_key))
            .collect::<Vec<_>>();
        ordered.sort_by_key(|(group, sort_key)| (*sort_key, group.clone()));
        let count = ordered.len();
        for (index, (group, _)) in ordered.into_iter().enumerate() {
            slot_by_group.insert(group, symmetric_slot(index, count));
        }
    }

    for edge in &ir.edges {
        let edge_props = &mut graph.edges[edge.original_edge.index()].properties;
        let source_slot = source_group_by_edge
            .get(&edge.original_edge)
            .and_then(|group| slot_by_group.get(group))
            .copied()
            .unwrap_or(0);
        let target_slot = target_group_by_edge
            .get(&edge.original_edge)
            .and_then(|group| slot_by_group.get(group))
            .copied()
            .unwrap_or(0);

        edge_props.insert(
            "spec42.endpoint.slot.source",
            elk_graph::PropertyValue::Int(source_slot as i64),
        );
        edge_props.insert(
            "spec42.endpoint.slot.target",
            elk_graph::PropertyValue::Int(target_slot as i64),
        );
    }
}

fn assign_shared_fanout_splits(
    graph: &mut ElkGraph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
) {
    let mut groups: BTreeMap<FanoutGroupKey, Vec<(EdgeId, Point, Point)>> = BTreeMap::new();

    for edge in &ir.edges {
        if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
            continue;
        }
        let edge_ref = &graph.edges[edge.original_edge.index()];
        let source_endpoint = edge_ref.sources.first().copied().unwrap_or(edge.routed_source);
        let target_endpoint = edge_ref.targets.first().copied().unwrap_or(edge.routed_target);
        let source_side = endpoint_side_for_routing(
            graph,
            source_endpoint,
            endpoint_abs_center(graph, target_endpoint),
            options.layered.direction,
            true,
        );
        let target_side = endpoint_side_for_routing(
            graph,
            target_endpoint,
            endpoint_abs_center(graph, source_endpoint),
            options.layered.direction,
            false,
        );
        if !supports_shared_fanout(source_side, target_side) {
            continue;
        }

        let source_slot = endpoint_slot(graph, edge.original_edge, true);
        let target_slot = endpoint_slot(graph, edge.original_edge, false);
        let start = endpoint_anchor_for_routing(graph, source_endpoint, source_side, source_slot);
        let end = endpoint_anchor_for_routing(graph, target_endpoint, target_side, target_slot);
        let source_lead = point_along_outward_normal(start, source_side, terminal_lead_for_slot(source_slot));
        let target_lead = point_along_outward_normal(end, target_side, terminal_lead_for_slot(target_slot));

        groups
            .entry(FanoutGroupKey {
                endpoint_kind: if source_endpoint.port.is_some() { 1 } else { 0 },
                endpoint_index: source_endpoint.port.map(|p| p.index()).unwrap_or(source_endpoint.node.index()),
                source_side: side_ordinal(source_side),
                target_side: side_ordinal(target_side),
                bundle_key: edge.bundle_key,
                source_layer: ir.nodes[ir.real_to_ir[&edge.effective_source]].layer,
                target_layer: ir.nodes[ir.real_to_ir[&edge.effective_target]].layer,
            })
            .or_default()
            .push((edge.original_edge, source_lead, target_lead));
    }

    for (group_key, edges) in groups.into_iter() {
        if edges.len() <= 1 {
            continue;
        }
        let split = shared_fanout_split_coordinate(
            &edges,
            options.layered.spacing.segment_spacing.max(8.0),
            group_key.source_side,
            group_key.target_side,
        );
        for (edge_id, source_lead, target_lead) in edges {
            if is_valid_shared_split(source_lead, target_lead, split, group_key.source_side, group_key.target_side) {
                graph.edges[edge_id.index()]
                    .properties
                    .insert(SHARED_FANOUT_SPLIT_KEY, elk_graph::PropertyValue::Float(split as f64));
            }
        }
    }
}

fn supports_shared_fanout(source_side: PortSide, target_side: PortSide) -> bool {
    matches!(
        (source_side, target_side),
        (PortSide::South, PortSide::North)
            | (PortSide::North, PortSide::South)
            | (PortSide::East, PortSide::West)
            | (PortSide::West, PortSide::East)
    )
}

fn shared_fanout_split_coordinate(
    edges: &[(EdgeId, Point, Point)],
    spacing: f32,
    source_side: u8,
    target_side: u8,
) -> f32 {
    match (source_side, target_side) {
        (2, 0) => {
            edges
                .iter()
                .map(|(_, _, target)| target.y)
                .fold(f32::INFINITY, f32::min)
                - spacing
        }
        (0, 2) => {
            edges
                .iter()
                .map(|(_, _, target)| target.y)
                .fold(f32::NEG_INFINITY, f32::max)
                + spacing
        }
        (1, 3) => edges
            .iter()
            .map(|(_, _, target)| target.x)
            .fold(f32::INFINITY, f32::min)
            - spacing,
        (3, 1) => edges
            .iter()
            .map(|(_, _, target)| target.x)
            .fold(f32::NEG_INFINITY, f32::max)
            + spacing,
        _ => edges[0].2.y,
    }
}

fn is_valid_shared_split(
    source_lead: Point,
    target_lead: Point,
    split: f32,
    source_side: u8,
    target_side: u8,
) -> bool {
    if matches!((source_side, target_side), (2, 0) | (0, 2)) {
        let min_y = source_lead.y.min(target_lead.y);
        let max_y = source_lead.y.max(target_lead.y);
        split > min_y && split < max_y
    } else {
        let min_x = source_lead.x.min(target_lead.x);
        let max_x = source_lead.x.max(target_lead.x);
        split > min_x && split < max_x
    }
}

fn endpoint_group(
    endpoint: elk_graph::EdgeEndpoint,
    side: PortSide,
    bundle_key: Option<u32>,
) -> EndpointGroup {
    match endpoint.port {
        Some(port_id) => EndpointGroup {
            endpoint_kind: 1,
            endpoint_index: port_id.index(),
            side: side_ordinal(side),
            bundle_key,
        },
        None => EndpointGroup {
            endpoint_kind: 0,
            endpoint_index: endpoint.node.index(),
            side: side_ordinal(side),
            bundle_key,
        },
    }
}

fn side_ordinal(side: PortSide) -> u8 {
    match side {
        PortSide::North => 0,
        PortSide::East => 1,
        PortSide::South => 2,
        PortSide::West => 3,
    }
}

fn endpoint_group_sort_key(
    graph: &ElkGraph,
    endpoint: elk_graph::EdgeEndpoint,
    opposite_endpoint: elk_graph::EdgeEndpoint,
    side: PortSide,
    edge: &IrEdge,
) -> GroupSortKey {
    GroupSortKey {
        endpoint_position: quantize_tangent_coordinate(endpoint_abs_center(graph, endpoint), side),
        opposite_position: quantize_tangent_coordinate(endpoint_abs_center(graph, opposite_endpoint), side),
        bundle_key: edge.bundle_key,
        model_order: edge.model_order,
        edge_index: edge.original_edge.index(),
    }
}

fn quantize_tangent_coordinate(point: Point, side: PortSide) -> i64 {
    let coordinate = match side {
        PortSide::North | PortSide::South => point.x,
        PortSide::East | PortSide::West => point.y,
    };
    (coordinate * 1000.0).round() as i64
}

fn symmetric_slot(index: usize, count: usize) -> i32 {
    if count <= 1 {
        return 0;
    }
    let mid = count / 2;
    if count % 2 == 1 {
        index as i32 - mid as i32
    } else if index < mid {
        index as i32 - mid as i32
    } else {
        index as i32 - mid as i32 + 1
    }
}

fn default_elbow(start: Point, end: Point) -> Vec<Point> {
    if (end.x - start.x).abs() >= (end.y - start.y).abs() {
        vec![Point::new(end.x, start.y)]
    } else {
        vec![Point::new(start.x, end.y)]
    }
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
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let mid_x = ((start.x + end.x) * 0.5 + lane_offset).clamp(min_x, max_x);
        vec![Point::new(mid_x, start.y), Point::new(mid_x, end.y)]
    } else {
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        let mid_y = ((start.y + end.y) * 0.5 + lane_offset).clamp(min_y, max_y);
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
        let inner = endpoint_abs_center(graph, edge.source);
        *start = boundary_anchor_for_inner_point(
            graph,
            edge.effective_source,
            *end,
            inner,
        );
    }
    if edge.target.port.is_none() && edge.target.node != edge.effective_target {
        let inner = endpoint_abs_center(graph, edge.target);
        *end = boundary_anchor_for_inner_point(
            graph,
            edge.effective_target,
            *start,
            inner,
        );
    }
}

fn boundary_anchor_for_inner_point(
    graph: &ElkGraph,
    node: NodeId,
    toward: Point,
    inner: Point,
) -> Point {
    let r = node_rect(graph, node);
    let center = Point::new(r.origin.x + r.size.width * 0.5, r.origin.y + r.size.height * 0.5);
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;
    if dx.abs() >= dy.abs() {
        let y = inner.y.clamp(r.origin.y, r.max_y());
        if dx >= 0.0 {
            Point::new(r.max_x(), y)
        } else {
            Point::new(r.origin.x, y)
        }
    } else {
        let x = inner.x.clamp(r.origin.x, r.max_x());
        if dy >= 0.0 {
            Point::new(x, r.max_y())
        } else {
            Point::new(x, r.origin.y)
        }
    }
}

fn edge_bundle_lane_offset(graph: &ElkGraph, edge_id: EdgeId) -> i32 {
    let edge = &graph.edges[edge_id.index()];
    let opts = decode_layout_from_props(&edge.properties);
    if let Some(k) = opts.edge_bundle_key {
        // Keep offsets small and deterministic; this is only a tie-breaker.
        return ((k % 5) as i32) - 2;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::import::import_graph;
    use std::collections::BTreeSet;

    #[test]
    fn orthogonalize_polyline_fixes_diagonal_terminal_segments() {
        let points = vec![
            Point::new(2228.0, 1391.3),
            Point::new(2316.0, 1319.3),
            Point::new(2316.0, 1627.4),
        ];
        let fixed = orthogonalize_polyline(points, Some(PortSide::East), Some(PortSide::West));
        assert_eq!(
            fixed,
            vec![
                Point::new(2228.0, 1391.3),
                Point::new(2316.0, 1391.3),
                Point::new(2316.0, 1627.4),
            ]
        );
    }

    #[test]
    fn orthogonalize_polyline_removes_internal_diagonals() {
        let points = vec![
            Point::new(476.0, 1156.3),
            Point::new(564.0, 151.2),
            Point::new(564.0, 1145.0),
        ];
        let fixed = orthogonalize_polyline(points, Some(PortSide::East), Some(PortSide::West));
        for seg in fixed.windows(2) {
            let a = seg[0];
            let b = seg[1];
            assert!(
                (a.x - b.x).abs() <= 1e-5 || (a.y - b.y).abs() <= 1e-5,
                "segment should be orthogonal: {:?} -> {:?}",
                a,
                b
            );
        }
    }

    #[test]
    fn ensure_terminal_normals_keeps_outward_stub_for_orthogonal_inward_start() {
        let points = vec![
            Point::new(100.0, 50.0),
            Point::new(100.0, 20.0),
            Point::new(60.0, 20.0),
            Point::new(60.0, 120.0),
        ];

        let fixed = ensure_terminal_normals(points, Some(PortSide::South), Some(PortSide::North));

        assert_eq!(fixed[0], Point::new(100.0, 50.0));
        assert_eq!(fixed[1], Point::new(100.0, 58.0));
        assert!(fixed.contains(&Point::new(60.0, 58.0)));
        assert_eq!(*fixed.last().unwrap_or(&Point::new(0.0, 0.0)), Point::new(60.0, 120.0));
    }

    #[test]
    fn assign_endpoint_slots_separates_node_endpoints_by_bundle() {
        let mut graph = ElkGraph::new();
        let source_left = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let source_right = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 160.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let target = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 80.0, y: 120.0, width: 80.0, height: 40.0 },
        );

        let left_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source_left)],
            vec![elk_graph::EdgeEndpoint::node(target)],
        );
        graph.edges[left_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(1));

        let right_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source_right)],
            vec![elk_graph::EdgeEndpoint::node(target)],
        );
        graph.edges[right_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(2));

        let options = LayoutOptions {
            layered: elk_core::LayeredOptions {
                direction: elk_core::LayoutDirection::TopToBottom,
                ..LayoutOptions::default().layered
            },
            ..LayoutOptions::default()
        };
        let local_nodes = BTreeSet::from([source_left, source_right, target]);
        let ir = import_graph(&graph, &[source_left, source_right, target], &local_nodes, &options);

        assign_endpoint_slots(&mut graph, &ir, &local_nodes, &options);

        let left_target_slot = graph.edges[left_edge.index()]
            .properties
            .get(&elk_graph::PropertyKey::from("spec42.endpoint.slot.target"))
            .and_then(|value| value.as_i64())
            .unwrap_or(0);
        let right_target_slot = graph.edges[right_edge.index()]
            .properties
            .get(&elk_graph::PropertyKey::from("spec42.endpoint.slot.target"))
            .and_then(|value| value.as_i64())
            .unwrap_or(0);

        assert_ne!(left_target_slot, 0);
        assert_ne!(right_target_slot, 0);
        assert_ne!(left_target_slot, right_target_slot);
        assert!(left_target_slot < right_target_slot);
    }

    #[test]
    fn build_orthogonal_route_path_uses_slotted_node_anchor() {
        let mut graph = ElkGraph::new();
        let source_left = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let source_right = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 160.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let target = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 80.0, y: 120.0, width: 80.0, height: 40.0 },
        );

        let left_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source_left)],
            vec![elk_graph::EdgeEndpoint::node(target)],
        );
        graph.edges[left_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(1));

        let right_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source_right)],
            vec![elk_graph::EdgeEndpoint::node(target)],
        );
        graph.edges[right_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(2));

        let options = LayoutOptions {
            layered: elk_core::LayeredOptions {
                direction: elk_core::LayoutDirection::TopToBottom,
                ..LayoutOptions::default().layered
            },
            ..LayoutOptions::default()
        };
        let local_nodes = BTreeSet::from([source_left, source_right, target]);
        let ir = import_graph(&graph, &[source_left, source_right, target], &local_nodes, &options);

        assign_endpoint_slots(&mut graph, &ir, &local_nodes, &options);

        let left_ir = ir.edges.iter().find(|edge| edge.original_edge == left_edge).unwrap();
        let right_ir = ir.edges.iter().find(|edge| edge.original_edge == right_edge).unwrap();

        let left_source = graph.edges[left_edge.index()].sources[0];
        let left_target = graph.edges[left_edge.index()].targets[0];
        let right_source = graph.edges[right_edge.index()].sources[0];
        let right_target = graph.edges[right_edge.index()].targets[0];

        let left_path = build_orthogonal_route_path(
            &graph,
            left_ir,
            left_source,
            left_target,
            endpoint_abs_center(&graph, left_source),
            endpoint_abs_center(&graph, left_target),
            0,
            &options,
        )
        .unwrap();
        let right_path = build_orthogonal_route_path(
            &graph,
            right_ir,
            right_source,
            right_target,
            endpoint_abs_center(&graph, right_source),
            endpoint_abs_center(&graph, right_target),
            0,
            &options,
        )
        .unwrap();

        let target_center_x = endpoint_abs_center(&graph, right_target).x;
        assert_ne!(left_path.last().unwrap().x, target_center_x);
        assert_ne!(right_path.last().unwrap().x, target_center_x);
        assert_ne!(left_path.last().unwrap().x, right_path.last().unwrap().x);
        assert!(left_path.last().unwrap().x < right_path.last().unwrap().x);
    }

    #[test]
    fn build_orthogonal_route_path_uses_slotted_port_anchor() {
        let mut graph = ElkGraph::new();
        let source_left = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let source_right = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 160.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let target = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 80.0, y: 120.0, width: 80.0, height: 40.0 },
        );

        let source_left_port = graph.add_port(
            source_left,
            PortSide::South,
            elk_graph::ShapeGeometry { x: 36.0, y: 36.0, width: 8.0, height: 8.0 },
        );
        let source_right_port = graph.add_port(
            source_right,
            PortSide::South,
            elk_graph::ShapeGeometry { x: 36.0, y: 36.0, width: 8.0, height: 8.0 },
        );
        let target_port = graph.add_port(
            target,
            PortSide::North,
            elk_graph::ShapeGeometry { x: 36.0, y: -4.0, width: 8.0, height: 8.0 },
        );

        let left_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::port(source_left, source_left_port)],
            vec![elk_graph::EdgeEndpoint::port(target, target_port)],
        );
        graph.edges[left_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(1));

        let right_edge = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::port(source_right, source_right_port)],
            vec![elk_graph::EdgeEndpoint::port(target, target_port)],
        );
        graph.edges[right_edge.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(2));

        let options = LayoutOptions {
            layered: elk_core::LayeredOptions {
                direction: elk_core::LayoutDirection::TopToBottom,
                ..LayoutOptions::default().layered
            },
            ..LayoutOptions::default()
        };
        let local_nodes = BTreeSet::from([source_left, source_right, target]);
        let ir = import_graph(&graph, &[source_left, source_right, target], &local_nodes, &options);

        assign_endpoint_slots(&mut graph, &ir, &local_nodes, &options);

        let left_ir = ir.edges.iter().find(|edge| edge.original_edge == left_edge).unwrap();
        let right_ir = ir.edges.iter().find(|edge| edge.original_edge == right_edge).unwrap();

        let left_source = graph.edges[left_edge.index()].sources[0];
        let left_target = graph.edges[left_edge.index()].targets[0];
        let right_source = graph.edges[right_edge.index()].sources[0];
        let right_target = graph.edges[right_edge.index()].targets[0];

        let left_path = build_orthogonal_route_path(
            &graph,
            left_ir,
            left_source,
            left_target,
            endpoint_abs_center(&graph, left_source),
            endpoint_abs_center(&graph, left_target),
            0,
            &options,
        )
        .unwrap();
        let right_path = build_orthogonal_route_path(
            &graph,
            right_ir,
            right_source,
            right_target,
            endpoint_abs_center(&graph, right_source),
            endpoint_abs_center(&graph, right_target),
            0,
            &options,
        )
        .unwrap();

        let target_port_center_x = endpoint_abs_center(&graph, right_target).x;
        assert_ne!(left_path.last().unwrap().x, target_port_center_x);
        assert_ne!(right_path.last().unwrap().x, target_port_center_x);
        assert_ne!(left_path.last().unwrap().x, right_path.last().unwrap().x);
        assert!(left_path.last().unwrap().x < right_path.last().unwrap().x);
    }

    #[test]
    fn assign_shared_fanout_splits_marks_common_source_group() {
        let mut graph = ElkGraph::new();
        let source = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 100.0, y: 0.0, width: 80.0, height: 40.0 },
        );
        let left = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 0.0, y: 120.0, width: 80.0, height: 40.0 },
        );
        let center = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 100.0, y: 120.0, width: 80.0, height: 40.0 },
        );
        let right = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry { x: 200.0, y: 120.0, width: 80.0, height: 40.0 },
        );

        let e1 = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source)],
            vec![elk_graph::EdgeEndpoint::node(left)],
        );
        let e2 = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source)],
            vec![elk_graph::EdgeEndpoint::node(center)],
        );
        let e3 = graph.add_edge(
            graph.root,
            vec![elk_graph::EdgeEndpoint::node(source)],
            vec![elk_graph::EdgeEndpoint::node(right)],
        );
        for edge_id in [e1, e2, e3] {
            graph.edges[edge_id.index()]
                .properties
                .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(1));
        }

        let options = LayoutOptions {
            layered: elk_core::LayeredOptions {
                direction: elk_core::LayoutDirection::TopToBottom,
                ..LayoutOptions::default().layered
            },
            ..LayoutOptions::default()
        };
        let local_nodes = BTreeSet::from([source, left, center, right]);
        let ir = import_graph(&graph, &[source, left, center, right], &local_nodes, &options);

        assign_endpoint_slots(&mut graph, &ir, &local_nodes, &options);
        assign_shared_fanout_splits(&mut graph, &ir, &local_nodes, &options);

        for edge_id in [e1, e2, e3] {
            assert!(
                graph.edges[edge_id.index()]
                    .properties
                    .get(&elk_graph::PropertyKey::from(SHARED_FANOUT_SPLIT_KEY))
                    .and_then(|value| value.as_f64())
                    .is_some()
            );
        }
    }
}
