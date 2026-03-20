use std::collections::{BTreeMap, BTreeSet};

use elk_core::{EdgeRouting, LayoutError, LayoutOptions, LayoutStats, Point, PortSide, Rect, Size};
use elk_graph::{ElkGraph, EdgeId, NodeId, PortId};

use crate::ir::{IrEdge, LayeredIr};
use crate::pipeline::orthogonal_routing_generator::{assign_routing_slots, HyperEdgeSegment};
use crate::pipeline::props::decode_layout_from_props;
use crate::pipeline::util::{
    dedup_points, endpoint_abs_center, endpoint_port_side, label_size, local_scope_frame,
    node_abs_origin, point_along_outward_normal,
};

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
            n.geometry.x = node.position.x;
            n.geometry.y = node.position.y;
            n.geometry.width = node.size.width;
            n.geometry.height = node.size.height;
            layout_ports(graph, node_id, options);
            layout_node_labels(graph, node_id, options);
        }
    }

    // Route edges: libavoid backend (if opted in) or simple 1-bend router.
    let use_libavoid = should_use_libavoid(graph, options);
    let mut routed = 0usize;
    if !use_libavoid && matches!(options.view_profile, elk_core::ViewProfile::InterconnectionView) {
        warnings.push("elk-layered: libavoid not selected for interconnection; using simplified router fallback".to_string());
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
                    if debug_enabled {
                        warnings.push(format!(
                            "elk-layered: edge-scope edge={:?} src_scope={:?} dst_scope={:?} src_frame={} dst_frame={}",
                            edge.original_edge,
                            edge.effective_source,
                            edge.effective_target,
                            endpoint_frame_debug(graph, edge.source),
                            endpoint_frame_debug(graph, edge.target)
                        ));
                    }
                    canonicalize_libavoid_terminals(graph, edge, warnings);
                    restore_nested_endpoint_terminals(graph, edge);
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

    if !use_libavoid {
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
                .unwrap_or(0)
                + edge_bundle_lane_offset(graph, edge.original_edge)
                + fanout_lane_bias_by_index
                    .get(&edge.original_edge.index())
                    .copied()
                    .unwrap_or(0);

            let mut bends = if routing == EdgeRouting::Orthogonal {
                if use_experimental_orthogonal_core(graph) {
                    build_staged_orthogonal_bends(graph, edge, start, end, lane, options)
                } else {
                    build_legacy_orthogonal_bends(graph, edge, start, end, lane, options)
                }
            } else {
                Vec::new()
            };
            bends = normalize_bends(bends, keep_unnecessary_bends);
            bends = correct_terminal_slants(start, end, bends);

            let edge_idx = edge.original_edge.index();
            graph.edges[edge_idx].sections.clear();
            let _ = graph.add_edge_section(edge.original_edge, start, bends.clone(), end);
            restore_nested_endpoint_terminals(graph, edge);
            let (start, end) = section_endpoints(graph, edge.original_edge);
            stats.bend_points += bends.len();
            routed += 1;
            place_edge_labels(graph, edge, start, end, options, stats);
        }
        if routed > 0 {
            warnings.push("elk-layered: simplified ElkGraph router active".to_string());
        }
    }

    Ok(routed)
}

fn canonicalize_libavoid_terminals(graph: &mut ElkGraph, edge: &IrEdge, warnings: &mut Vec<String>) {
    let edge_ref = &graph.edges[edge.original_edge.index()];
    let Some(first_id) = edge_ref.sections.first().copied() else {
        return;
    };
    let Some(last_id) = edge_ref.sections.last().copied() else {
        return;
    };
    let observed_start = graph.edge_sections[first_id.index()].start;
    let observed_end = graph.edge_sections[last_id.index()].end;
    let start_endpoint = graph.edges[edge.original_edge.index()]
        .sources
        .first()
        .copied()
        .unwrap_or(edge.source);
    let end_endpoint = graph.edges[edge.original_edge.index()]
        .targets
        .first()
        .copied()
        .unwrap_or(edge.target);
    let start = endpoint_abs_center(graph, start_endpoint);
    let end = endpoint_abs_center(graph, end_endpoint);
    let start_d = ((observed_start.x - start.x).powi(2) + (observed_start.y - start.y).powi(2)).sqrt();
    let end_d = ((observed_end.x - end.x).powi(2) + (observed_end.y - end.y).powi(2)).sqrt();
    set_section_start_preserve_orthogonality(&mut graph.edge_sections[first_id.index()], start);
    set_section_end_preserve_orthogonality(&mut graph.edge_sections[last_id.index()], end);

    if std::env::var_os("SPEC42_ELK_DEBUG").is_some() {
        if start_d > 64.0 || end_d > 64.0 {
            warnings.push(format!(
                "elk-layered: libavoid terminal canonicalization adjusted edge {:?} (start_delta={:.1}, end_delta={:.1})",
                edge.original_edge, start_d, end_d
            ));
        }
        let extent = graph_layout_extent(graph);
        let margin = 64.0f32;
        let out = |p: Point| -> bool {
            p.x < -margin
                || p.y < -margin
                || p.x > extent.max_x + margin
                || p.y > extent.max_y + margin
        };
        if out(observed_start) || out(observed_end) || out(start) || out(end) {
            warnings.push(format!(
                "elk-layered: libavoid edge {:?} endpoint out-of-bounds observed=({:.1},{:.1})->({:.1},{:.1}) chosen=({:.1},{:.1})->({:.1},{:.1}) root=({:.1}x{:.1})",
                edge.original_edge,
                observed_start.x,
                observed_start.y,
                observed_end.x,
                observed_end.y,
                start.x,
                start.y,
                end.x,
                end.y,
                extent.max_x,
                extent.max_y
            ));
        }
    }
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


struct LayoutExtent {
    max_x: f32,
    max_y: f32,
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

fn graph_layout_extent(graph: &ElkGraph) -> LayoutExtent {
    let mut max_x = 0.0f32;
    let mut max_y = 0.0f32;
    for node in &graph.nodes {
        if node.id == graph.root {
            continue;
        }
        let o = node_abs_origin(graph, node.id);
        max_x = max_x.max(o.x + node.geometry.width.max(0.0));
        max_y = max_y.max(o.y + node.geometry.height.max(0.0));
    }
    LayoutExtent { max_x, max_y }
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

pub(crate) fn snap_all_edge_terminals_to_endpoints(graph: &mut ElkGraph) {
    let edge_ids: Vec<EdgeId> = graph.edges.iter().map(|e| e.id).collect();
    for edge_id in edge_ids {
        let edge = &graph.edges[edge_id.index()];
        let (Some(source), Some(target), Some(&first_id), Some(&last_id)) = (
            edge.sources.first().copied(),
            edge.targets.first().copied(),
            edge.sections.first(),
            edge.sections.last(),
        ) else {
            continue;
        };
        let start = endpoint_abs_center(graph, source);
        let end = endpoint_abs_center(graph, target);
        set_section_start_preserve_orthogonality(&mut graph.edge_sections[first_id.index()], start);
        set_section_end_preserve_orthogonality(&mut graph.edge_sections[last_id.index()], end);
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

fn build_legacy_orthogonal_bends(
    graph: &ElkGraph,
    edge: &IrEdge,
    start: Point,
    end: Point,
    lane: i32,
    options: &LayoutOptions,
) -> Vec<Point> {
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Vec::new();
    }
    let source_side = endpoint_port_side(graph, edge.source);
    let target_side = endpoint_port_side(graph, edge.target);
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    if let (Some(ss), Some(ts)) = (source_side, target_side) {
        let exit = point_along_outward_normal(start, ss, PORT_NORMAL_OFFSET);
        let entry = point_along_outward_normal(end, ts, PORT_NORMAL_OFFSET);
        vec![exit, Point::new(entry.x, exit.y), entry]
    } else {
        build_lane_orthogonal_bends(start, end, lane, options)
    }
}

fn build_staged_orthogonal_bends(
    graph: &ElkGraph,
    edge: &IrEdge,
    start: Point,
    end: Point,
    lane: i32,
    options: &LayoutOptions,
) -> Vec<Point> {
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Vec::new();
    }

    // Stage 1: terminal attachment from source/target ports.
    let (exit, entry, mut bends) = terminal_attachment(graph, edge, start, end);

    // Stage 2: trunk path generation with slot/lane guidance.
    bends.extend(build_lane_orthogonal_bends(exit, entry, lane, options));

    // Stage 3: detour generation for degenerate orthogonal trunks.
    if bends.is_empty() {
        bends.extend(default_elbow(exit, entry));
    }

    // Stage 4: enforce final terminal orthogonality and simplification happens later.
    if let Some(last) = bends.last().copied() {
        if (last.x - entry.x).abs() > f32::EPSILON && (last.y - entry.y).abs() > f32::EPSILON {
            bends.push(Point::new(last.x, entry.y));
        }
    }
    bends
}

fn terminal_attachment(
    graph: &ElkGraph,
    edge: &IrEdge,
    start: Point,
    end: Point,
) -> (Point, Point, Vec<Point>) {
    const PORT_NORMAL_OFFSET: f32 = 8.0;
    let source_side = endpoint_port_side(graph, edge.source);
    let target_side = endpoint_port_side(graph, edge.target);
    let mut bends = Vec::new();
    let exit = if let Some(side) = source_side {
        let p = point_along_outward_normal(start, side, PORT_NORMAL_OFFSET);
        bends.push(p);
        p
    } else {
        start
    };
    let entry = if let Some(side) = target_side {
        point_along_outward_normal(end, side, PORT_NORMAL_OFFSET)
    } else {
        end
    };
    (exit, entry, bends)
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

fn edge_bundle_lane_offset(graph: &ElkGraph, edge_id: EdgeId) -> i32 {
    let edge = &graph.edges[edge_id.index()];
    let opts = decode_layout_from_props(&edge.properties);
    if let Some(k) = opts.edge_bundle_key {
        // Keep offsets small and deterministic; this is only a tie-breaker.
        return ((k % 5) as i32) - 2;
    }
    0
}

fn use_experimental_orthogonal_core(graph: &ElkGraph) -> bool {
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    for key in [
        "elk.layered.experimentalorthogonalcore",
        "org.eclipse.elk.layered.experimentalOrthogonalCore",
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
    true
}

