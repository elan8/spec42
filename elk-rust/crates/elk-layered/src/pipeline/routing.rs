use std::collections::{BTreeMap, BTreeSet};

use elk_core::{
    ContentAlignment, EdgeEndpoint, EdgeLabelPlacement, EdgeRouting, EdgeSection, Graph,
    LayoutDirection, LayoutOptions, LayoutStats, NodeId, NodeLabelPlacement, Point, PortConstraint,
    PortId, PortLabelPlacement, PortSide, Rect, Size, ViewProfile,
};

use crate::ir::{IrEdge, LayeredIr, NormalizedEdge};
use crate::pipeline::util::{dedup_points, ensure_orthogonal_path, simplify_orthogonal_points};

pub(crate) fn export_to_graph(
    graph: &mut Graph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
    warnings: &mut Vec<String>,
    stats: &mut LayoutStats,
) -> usize {
    for node in &ir.nodes {
        if let crate::ir::IrNodeKind::Real(node_id) = node.kind {
            let old_origin = graph.node(node_id).bounds.origin;
            let delta = Point::new(
                node.position.x - old_origin.x,
                node.position.y - old_origin.y,
            );
            graph.node_mut(node_id).bounds.origin = node.position;
            if delta.x != 0.0 || delta.y != 0.0 {
                translate_descendants(graph, node_id, delta);
            }
            layout_ports(graph, node_id, options, stats);
            layout_node_labels(graph, node_id, options, stats);
        }
    }

    let mut routed_segments = 0usize;
    for (edge_index, edge) in ir.edges.iter().enumerate() {
        if !local_nodes.contains(&edge.effective_source)
            || !local_nodes.contains(&edge.effective_target)
        {
            continue;
        }

        if edge.self_loop {
            route_self_loop(graph, edge, options);
            warnings.push(format!(
                "self-loop edge {} routed with stable loop fallback",
                edge.original_edge
            ));
            routed_segments += 1;
            let section = graph.edge(edge.original_edge).sections[0].clone();
            stats.bend_points += section.bend_points.len();
            place_edge_labels(graph, edge, &section, options, stats);
            continue;
        }

        let start_hint = endpoint_center(graph, edge.target);
        let end_hint = endpoint_center(graph, edge.source);
        let start = spread_anchor_point_towards(
            graph,
            ir,
            edge_index,
            edge.source,
            start_hint,
            options.view_profile,
        );
        let end = spread_anchor_point_towards(
            graph,
            ir,
            edge_index,
            edge.target,
            end_hint,
            options.view_profile,
        );
        let start_side = endpoint_anchor_side(graph, edge.source, start_hint, options.view_profile);
        let end_side = endpoint_anchor_side(graph, edge.target, end_hint, options.view_profile);
        let stub_len = options.layered.spacing.edge_spacing.clamp(12.0, 24.0);
        let routed_start = start_side
            .map(|side| extend_from_side(start, side, stub_len))
            .unwrap_or(start);
        let routed_end = end_side
            .map(|side| extend_from_side(end, side, stub_len))
            .unwrap_or(end);
        let bends = match edge_routing_for_edge(graph, edge, options) {
            EdgeRouting::Straight => join_with_endpoint_stubs(
                start,
                routed_start,
                straight_path(ir, edge, routed_start, routed_end),
                routed_end,
                end,
            ),
            EdgeRouting::Orthogonal => {
                let raw = orthogonal_path(ir, edge_index, routed_start, routed_end, options);
                join_with_endpoint_stubs(
                    start,
                    routed_start,
                    obstacle_aware_bends(graph, edge, routed_start, routed_end, raw, options),
                    routed_end,
                    end,
                )
            }
        };

        let full_path: Vec<Point> = std::iter::once(start)
            .chain(bends)
            .chain(std::iter::once(end))
            .collect();
        let orthogonal = ensure_orthogonal_path(full_path);
        let (start_orth, mid, end_orth) = if orthogonal.len() >= 2 {
            let start_orth = orthogonal[0];
            let end_orth = *orthogonal.last().unwrap();
            let bend_points: Vec<Point> = orthogonal[1..orthogonal.len() - 1].to_vec();
            (start_orth, bend_points, end_orth)
        } else {
            (start, vec![], end)
        };

        let bounds = graph_bounds(graph);
        let margin = options.layered.spacing.edge_spacing.max(8.0);
        // Keep port endpoints exact: clamping can shift section start/end away from the port center.
        // We still clamp bend points so detours stay on-canvas.
        let section_start = if edge.source.port.is_some() {
            start_orth
        } else {
            clamp_point_to_rect(start_orth, bounds, margin)
        };
        let section_end = if edge.target.port.is_some() {
            end_orth
        } else {
            clamp_point_to_rect(end_orth, bounds, margin)
        };
        let section = EdgeSection {
            start: section_start,
            bend_points: mid
                .into_iter()
                .map(|p| clamp_point_to_rect(p, bounds, margin))
                .collect(),
            end: section_end,
        };
        let edge_mut = graph.edge_mut(edge.original_edge);
        edge_mut.was_reversed = edge.reversed;
        edge_mut.sections = vec![section.clone()];
        stats.bend_points += section.bend_points.len();
        place_edge_labels(graph, edge, &section, options, stats);
        routed_segments += edge_segments_for_edge(ir, edge_index).len().max(1);
    }

    routed_segments
}

fn edge_routing_for_edge(graph: &Graph, edge: &IrEdge, options: &LayoutOptions) -> EdgeRouting {
    graph
        .edge(edge.original_edge)
        .layout
        .edge_routing
        .or(graph.layout.edge_routing)
        .unwrap_or(options.layered.edge_routing)
}

fn straight_path(ir: &LayeredIr, edge: &IrEdge, start: Point, end: Point) -> Vec<Point> {
    let mut chain: Vec<Point> = edge
        .chain
        .iter()
        .map(|node_id| ir.nodes[*node_id].center())
        .collect();
    if edge.reversed {
        chain.reverse();
    }
    let mut points = vec![start];
    points.extend(chain);
    points.push(end);
    let deduped = dedup_points(points);
    if deduped.len() <= 2 {
        return Vec::new();
    }
    let simplified = simplify_orthogonal_points(deduped);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn orthogonal_path(
    ir: &LayeredIr,
    edge_index: usize,
    start: Point,
    end: Point,
    options: &LayoutOptions,
) -> Vec<Point> {
    let direction = options.layered.direction;
    let segment_spacing = options.layered.spacing.segment_spacing.max(12.0);
    let edge = &ir.edges[edge_index];
    let points = chain_points(ir, edge, start, end);
    let segments = edge_segments_for_edge(ir, edge_index);
    let mut bends = Vec::new();

    for (segment_index, window) in points.windows(2).enumerate() {
        let lane = segments
            .get(segment_index)
            .map(|segment| alternating_lane_offset(segment.lane, segment_spacing))
            .unwrap_or_default();
        let mut segment_bends = orthogonal_segment(window[0], window[1], lane, direction);
        bends.append(&mut segment_bends);
    }

    let mut points = vec![start];
    points.extend(bends);
    points.push(end);
    let simplified = simplify_orthogonal_points(points);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn chain_points(ir: &LayeredIr, edge: &IrEdge, start: Point, end: Point) -> Vec<Point> {
    let mut points = vec![start];
    let mut chain: Vec<Point> = edge
        .chain
        .iter()
        .map(|node_id| ir.nodes[*node_id].center())
        .collect();
    if edge.reversed {
        chain.reverse();
    }
    points.extend(chain);
    points.push(end);
    points
}

fn edge_segments_for_edge(ir: &LayeredIr, edge_index: usize) -> Vec<&NormalizedEdge> {
    let mut segments: Vec<_> = ir
        .normalized_edges
        .iter()
        .filter(|segment| segment.edge_index == edge_index)
        .collect();
    segments.sort_by_key(|segment| segment.segment_order);
    if ir.edges[edge_index].reversed {
        segments.reverse();
    }
    segments
}

fn orthogonal_segment(
    start: Point,
    end: Point,
    lane_offset: f32,
    direction: LayoutDirection,
) -> Vec<Point> {
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Vec::new();
    }

    let horizontal_major = matches!(
        direction,
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft
    );

    if lane_offset.abs() <= f32::EPSILON {
        let simple = if horizontal_major {
            vec![Point::new(end.x, start.y)]
        } else {
            vec![Point::new(start.x, end.y)]
        };
        return dedup_points(simple);
    }

    let mut bends = Vec::new();
    if horizontal_major {
        let preferred_minor = (start.y + end.y) / 2.0 + lane_offset;
        bends.push(Point::new(start.x, preferred_minor));
        bends.push(Point::new(end.x, preferred_minor));
    } else {
        let preferred_minor = (start.x + end.x) / 2.0 + lane_offset;
        bends.push(Point::new(preferred_minor, start.y));
        bends.push(Point::new(preferred_minor, end.y));
    }

    dedup_points(bends)
}

fn alternating_lane_offset(lane: i32, spacing: f32) -> f32 {
    if lane == 0 {
        return 0.0;
    }

    let normalized = lane.unsigned_abs() as usize;
    let magnitude = normalized.div_ceil(2) as f32 * spacing;
    if normalized % 2 == 0 {
        -magnitude
    } else {
        magnitude
    }
}

fn join_with_endpoint_stubs(
    start: Point,
    routed_start: Point,
    bends: Vec<Point>,
    routed_end: Point,
    end: Point,
) -> Vec<Point> {
    let mut points = vec![start];
    if !same_point(start, routed_start) {
        points.push(routed_start);
    }
    points.extend(bends);
    if !same_point(routed_end, end) {
        points.push(routed_end);
    }
    points.push(end);
    let simplified = simplify_orthogonal_points(dedup_points(points));
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn obstacle_aware_bends(
    graph: &Graph,
    edge: &IrEdge,
    start: Point,
    end: Point,
    bends: Vec<Point>,
    options: &LayoutOptions,
) -> Vec<Point> {
    let mut points = vec![start];
    points.extend(bends.iter().copied());
    points.push(end);

    for (segment_index, window) in points.windows(2).enumerate() {
        for node in &graph.nodes {
            if node.id == edge.routed_source.node
                || node.id == edge.routed_target.node
                || node.id == edge.source.node
                || node.id == edge.target.node
                || shares_endpoint_hierarchy(graph, node.id, edge)
            {
                continue;
            }
            if segment_intersects_rect(
                window[0],
                window[1],
                inflate_rect(node.bounds, options.layered.spacing.edge_spacing / 2.0),
            ) {
                return splice_detour(
                    &points,
                    segment_index,
                    detour_around_rect(window[0], window[1], node.bounds, options),
                );
            }
        }

        for label in &graph.labels {
            let rect = Rect::new(label.position, label.size);
            if segment_intersects_rect(
                window[0],
                window[1],
                inflate_rect(rect, options.layered.spacing.label_clearance),
            ) {
                let detour_point = Point::new(
                    rect.max_x() + options.layered.spacing.label_clearance,
                    rect.max_y() + options.layered.spacing.label_clearance,
                );
                return splice_detour(
                    &points,
                    segment_index,
                    dedup_points(vec![
                        Point::new(detour_point.x, window[0].y),
                        Point::new(detour_point.x, detour_point.y),
                        Point::new(window[1].x, detour_point.y),
                    ]),
                );
            }
        }
    }

    let mut points = vec![start];
    points.extend(bends);
    points.push(end);
    let simplified = simplify_orthogonal_points(points);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn shares_endpoint_hierarchy(graph: &Graph, candidate: NodeId, edge: &IrEdge) -> bool {
    [
        edge.source.node,
        edge.target.node,
        edge.routed_source.node,
        edge.routed_target.node,
    ]
    .into_iter()
    .any(|endpoint| {
        is_ancestor(graph, candidate, endpoint) || is_ancestor(graph, endpoint, candidate)
    })
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

fn splice_detour(points: &[Point], segment_index: usize, detour: Vec<Point>) -> Vec<Point> {
    let mut rerouted = Vec::with_capacity(points.len() + detour.len());
    rerouted.extend_from_slice(&points[..=segment_index]);
    rerouted.extend(detour);
    rerouted.extend_from_slice(&points[segment_index + 1..]);
    let rerouted = simplify_orthogonal_points(rerouted);
    if rerouted.len() <= 2 {
        Vec::new()
    } else {
        rerouted[1..rerouted.len() - 1].to_vec()
    }
}

fn detour_around_rect(start: Point, end: Point, rect: Rect, options: &LayoutOptions) -> Vec<Point> {
    let gap = options.layered.spacing.edge_spacing.max(20.0);
    let direction = options.layered.direction;

    if matches!(
        direction,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop
    ) {
        let detour_y = if end.y <= rect.origin.y {
            rect.origin.y - gap
        } else {
            rect.max_y() + gap
        };
        let candidates = [
            vec![
                Point::new(rect.origin.x - gap, start.y),
                Point::new(rect.origin.x - gap, detour_y),
                Point::new(end.x, detour_y),
            ],
            vec![
                Point::new(rect.max_x() + gap, start.y),
                Point::new(rect.max_x() + gap, detour_y),
                Point::new(end.x, detour_y),
            ],
        ];
        return best_detour(candidates);
    }

    let detour_x = if end.x <= rect.origin.x {
        rect.origin.x - gap
    } else {
        rect.max_x() + gap
    };
    let candidates = [
        vec![
            Point::new(start.x, rect.origin.y - gap),
            Point::new(detour_x, rect.origin.y - gap),
            Point::new(detour_x, end.y),
        ],
        vec![
            Point::new(start.x, rect.max_y() + gap),
            Point::new(detour_x, rect.max_y() + gap),
            Point::new(detour_x, end.y),
        ],
    ];
    best_detour(candidates)
}

fn best_detour<const N: usize>(candidates: [Vec<Point>; N]) -> Vec<Point> {
    candidates
        .into_iter()
        .map(dedup_points)
        .min_by(|left, right| {
            route_length(left)
                .partial_cmp(&route_length(right))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or_default()
}

fn route_length(points: &[Point]) -> f32 {
    points
        .windows(2)
        .map(|window| (window[1].x - window[0].x).abs() + (window[1].y - window[0].y).abs())
        .sum()
}

fn place_edge_labels(
    graph: &mut Graph,
    edge: &IrEdge,
    section: &EdgeSection,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    if edge.label_ids.is_empty() {
        return;
    }

    let (base, horizontal) = best_label_anchor(section);
    for (index, label_id) in edge.label_ids.iter().enumerate() {
        let placement = graph.labels[label_id.index()]
            .layout
            .edge_label_placement
            .unwrap_or(EdgeLabelPlacement::Center);
        let spacing = options.layered.spacing.label_spacing;
        let clearance = options.layered.spacing.label_clearance;
        let size = graph.labels[label_id.index()].size;
        let stacked = index as f32 * (size.height + 4.0);
        let mut position = match placement {
            EdgeLabelPlacement::Head => Point::new(
                section.start.x + spacing,
                section.start.y - size.height - spacing,
            ),
            EdgeLabelPlacement::Tail => Point::new(
                section.end.x - size.width - spacing,
                section.end.y + spacing,
            ),
            EdgeLabelPlacement::Center => {
                if horizontal {
                    Point::new(
                        base.x - size.width / 2.0,
                        base.y - size.height - spacing - stacked,
                    )
                } else {
                    Point::new(base.x + spacing, base.y - size.height / 2.0 + stacked)
                }
            }
        };

        for _ in 0..10 {
            let rect = inflate_rect(Rect::new(position, size), clearance);
            let node_clear = graph.nodes.iter().all(|node| !rect.intersects(node.bounds));
            let label_clear = graph
                .labels
                .iter()
                .filter(|other| other.id != *label_id)
                .all(|other| !rect.intersects(Rect::new(other.position, other.size)));
            if node_clear && label_clear {
                break;
            }
            position.y -= size.height + clearance;
            position.x += if horizontal { 0.0 } else { clearance / 2.0 };
        }

        graph.labels[label_id.index()].position = position;
        stats.label_displacements += distance(position, base);
    }
}

fn best_label_anchor(section: &EdgeSection) -> (Point, bool) {
    let mut points = vec![section.start];
    points.extend(section.bend_points.iter().copied());
    points.push(section.end);

    let mut best = (
        0.0f32,
        Point::new(
            (section.start.x + section.end.x) / 2.0,
            (section.start.y + section.end.y) / 2.0,
        ),
        true,
    );
    for window in points.windows(2) {
        let dx = (window[0].x - window[1].x).abs();
        let dy = (window[0].y - window[1].y).abs();
        let length = dx + dy;
        if length > best.0 {
            best = (
                length,
                Point::new(
                    (window[0].x + window[1].x) / 2.0,
                    (window[0].y + window[1].y) / 2.0,
                ),
                dx >= dy,
            );
        }
    }
    (best.1, best.2)
}

fn translate_descendants(graph: &mut Graph, parent: NodeId, delta: Point) {
    let children = graph.children_of(parent).to_vec();
    for child in children {
        let (ports, labels) = {
            let node = graph.node_mut(child);
            node.bounds.origin.x += delta.x;
            node.bounds.origin.y += delta.y;
            (node.ports.clone(), node.labels.clone())
        };
        for port_id in ports {
            let label_ids = {
                let port = graph.port_mut(port_id);
                port.bounds.origin.x += delta.x;
                port.bounds.origin.y += delta.y;
                port.labels.clone()
            };
            for label_id in label_ids {
                graph.labels[label_id.index()].position.x += delta.x;
                graph.labels[label_id.index()].position.y += delta.y;
            }
        }
        for label_id in labels {
            graph.labels[label_id.index()].position.x += delta.x;
            graph.labels[label_id.index()].position.y += delta.y;
        }
        translate_descendants(graph, child, delta);
    }
}

fn layout_ports(
    graph: &mut Graph,
    node_id: NodeId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let bounds = graph.node(node_id).bounds;
    let port_ids = graph.node(node_id).ports.clone();
    let node_layout = graph
        .node(node_id)
        .layout
        .clone()
        .inherit_from(&graph.layout);
    let respect_port_order = node_layout
        .respect_port_order
        .unwrap_or(options.layered.respect_port_order);
    let mut grouped: BTreeMap<PortSide, Vec<PortId>> = BTreeMap::new();
    for port_id in port_ids {
        let side = graph.port(port_id).side;
        grouped.entry(side).or_default().push(port_id);
    }

    for (side, mut ports) in grouped {
        if !respect_port_order {
            ports.sort_by_key(|port_id| {
                graph
                    .port(*port_id)
                    .layout
                    .model_order
                    .unwrap_or(port_id.index())
            });
        }
        let count = ports.len().max(1) as f32;
        for (index, port_id) in ports.into_iter().enumerate() {
            let port_layout = graph
                .port(port_id)
                .layout
                .clone()
                .inherit_from(&node_layout);
            if port_layout.port_constraint == Some(PortConstraint::FixedPosition) {
                layout_port_labels(graph, port_id, options, stats);
                continue;
            }
            let fraction = (index as f32 + 1.0) / (count + 1.0);
            let port = graph.port_mut(port_id);
            let size = port.bounds.size;
            port.bounds.origin = match side {
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
            layout_port_labels(graph, port_id, options, stats);
        }
    }
}

fn layout_node_labels(
    graph: &mut Graph,
    node_id: NodeId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let bounds = graph.node(node_id).bounds;
    let node_layout = graph
        .node(node_id)
        .layout
        .clone()
        .inherit_from(&graph.layout);
    let spacing = node_layout
        .spacing
        .unwrap_or(options.layered.spacing)
        .label_spacing;
    let labels = graph.node(node_id).labels.clone();
    if labels.is_empty() {
        return;
    }

    let total_height: f32 = labels
        .iter()
        .map(|label_id| graph.labels[label_id.index()].size.height)
        .sum();
    let content_alignment = node_layout
        .content_alignment
        .unwrap_or(ContentAlignment::Center);
    let inside_start_y = match content_alignment {
        ContentAlignment::Start => bounds.origin.y + spacing,
        ContentAlignment::Center => bounds.origin.y + (bounds.size.height - total_height) / 2.0,
        ContentAlignment::End => bounds.max_y() - total_height - spacing,
    };

    let mut cursor_y = 0.0;
    for label_id in labels {
        let placement = graph.labels[label_id.index()]
            .layout
            .node_label_placement
            .or(node_layout.node_label_placement)
            .unwrap_or(NodeLabelPlacement::OutsideTopCenter);
        let label = &mut graph.labels[label_id.index()];
        let anchor = Point::new(bounds.origin.x + bounds.size.width / 2.0, bounds.origin.y);
        label.position = match placement {
            NodeLabelPlacement::InsideTopLeft => Point::new(
                bounds.origin.x + spacing,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::InsideTopCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::InsideTopRight => Point::new(
                bounds.max_x() - label.size.width - spacing,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::OutsideBottomCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.max_y() + spacing + cursor_y,
            ),
            NodeLabelPlacement::OutsideTopCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.origin.y - label.size.height - spacing - cursor_y,
            ),
        };
        if matches!(
            placement,
            NodeLabelPlacement::InsideTopLeft
                | NodeLabelPlacement::InsideTopCenter
                | NodeLabelPlacement::InsideTopRight
        ) {
            label.position.y = inside_start_y + cursor_y;
        }
        stats.label_displacements += distance(label.position, anchor);
        cursor_y += label.size.height + spacing;
    }
}

fn layout_port_labels(
    graph: &mut Graph,
    port_id: PortId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let port = graph.port(port_id).clone();
    let labels = port.labels.clone();
    if labels.is_empty() {
        return;
    }
    let spacing = options.layered.spacing.port_label_spacing;
    let mut cursor = 0.0;
    for label_id in labels {
        let placement = graph.labels[label_id.index()]
            .layout
            .port_label_placement
            .or(port.layout.port_label_placement)
            .unwrap_or(PortLabelPlacement::Outside);
        let label = &mut graph.labels[label_id.index()];
        let anchor = port.bounds.center();
        label.position = match placement {
            PortLabelPlacement::Inside => Point::new(
                port.bounds.origin.x + (port.bounds.size.width - label.size.width) / 2.0,
                port.bounds.origin.y + (port.bounds.size.height - label.size.height) / 2.0 + cursor,
            ),
            PortLabelPlacement::NextToPortIfPossible | PortLabelPlacement::Outside => {
                match port.side {
                    PortSide::North => Point::new(
                        port.bounds.origin.x,
                        port.bounds.origin.y - label.size.height - spacing - cursor,
                    ),
                    PortSide::South => {
                        Point::new(port.bounds.origin.x, port.bounds.max_y() + spacing + cursor)
                    }
                    PortSide::East => {
                        Point::new(port.bounds.max_x() + spacing, port.bounds.origin.y + cursor)
                    }
                    PortSide::West => Point::new(
                        port.bounds.origin.x - label.size.width - spacing,
                        port.bounds.origin.y + cursor,
                    ),
                }
            }
        };
        stats.label_displacements += distance(label.position, anchor);
        cursor += label.size.height + spacing;
    }
}

fn spread_anchor_point_towards(
    graph: &Graph,
    ir: &LayeredIr,
    edge_index: usize,
    endpoint: EdgeEndpoint,
    toward: Point,
    profile: ViewProfile,
) -> Point {
    if let Some(port_id) = endpoint.port {
        return graph.port(port_id).bounds.center();
    }
    let bounds = endpoint_node_bounds(graph, endpoint.node);
    let side = choose_anchor_side(bounds, toward, profile);
    let offset = incident_anchor_offset(graph, ir, edge_index, endpoint.node, side);
    base_anchor_point(bounds, side, offset)
}

fn endpoint_anchor_side(
    graph: &Graph,
    endpoint: EdgeEndpoint,
    toward: Point,
    profile: ViewProfile,
) -> Option<PortSide> {
    if endpoint.port.is_some() {
        return None;
    }
    Some(choose_anchor_side(
        endpoint_node_bounds(graph, endpoint.node),
        toward,
        profile,
    ))
}

fn choose_anchor_side(bounds: Rect, toward: Point, profile: ViewProfile) -> PortSide {
    let center = bounds.center();
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;

    if matches!(profile, ViewProfile::InterconnectionView) {
        if dx.abs() >= dy.abs() {
            if dx >= 0.0 {
                PortSide::East
            } else {
                PortSide::West
            }
        } else if dy >= 0.0 {
            PortSide::South
        } else {
            PortSide::North
        }
    } else if dy >= 0.0 {
        PortSide::South
    } else {
        PortSide::North
    }
}

fn incident_anchor_offset(
    graph: &Graph,
    ir: &LayeredIr,
    edge_index: usize,
    node_id: NodeId,
    side: PortSide,
) -> f32 {
    let bounds = endpoint_node_bounds(graph, node_id);
    let span = if matches!(side, PortSide::East | PortSide::West) {
        bounds.size.height
    } else {
        bounds.size.width
    };
    let spacing = 12.0f32;

    let mut incident: Vec<(usize, f32, Option<u32>)> = ir
        .edges
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let endpoint = if edge.source.node == node_id {
                Some(edge.target)
            } else if edge.target.node == node_id {
                Some(edge.source)
            } else {
                None
            }?;
            if endpoint.port.is_some() {
                return None;
            }
            let toward = endpoint_center(graph, endpoint);
            let candidate_side = choose_anchor_side(bounds, toward, ViewProfile::InterconnectionView);
            if candidate_side != side {
                return None;
            }
            let sort_key = if matches!(side, PortSide::East | PortSide::West) {
                toward.y
            } else {
                toward.x
            };
            Some((index, sort_key, edge.bundle_key))
        })
        .collect();

    if incident.len() <= 1 {
        return 0.0;
    }

    incident.sort_by(|left, right| {
        let left_group = (left.2, left.2.map(|_| 0).unwrap_or(left.0));
        let right_group = (right.2, right.2.map(|_| 0).unwrap_or(right.0));
        left_group
            .cmp(&right_group)
            .then_with(|| {
                left.1
                    .partial_cmp(&right.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| left.0.cmp(&right.0))
    });

    let mut group_of_edge: Option<i32> = None;
    let mut current_group_key: (Option<u32>, usize) = (None, usize::MAX);
    let mut group_index: i32 = -1;
    for (idx, _sort_key, bundle_key) in &incident {
        let group_key = (*bundle_key, bundle_key.map(|_| 0).unwrap_or(*idx));
        if current_group_key != group_key {
            current_group_key = group_key;
            group_index += 1;
        }
        if *idx == edge_index {
            group_of_edge = Some(group_index);
            break;
        }
    }
    let Some(edge_group) = group_of_edge else {
        return 0.0;
    };
    let total_groups = group_index + 1;
    let center_group = (total_groups as f32 - 1.0) / 2.0;
    let raw = (edge_group as f32 - center_group) * spacing;
    let limit = (span / 2.0 - 10.0).max(spacing);
    raw.clamp(-limit, limit)
}

fn base_anchor_point(bounds: Rect, side: PortSide, offset: f32) -> Point {
    let center = bounds.center();
    match side {
        PortSide::East => Point::new(
            bounds.max_x(),
            clamp(bounds.origin.y, bounds.max_y(), center.y + offset),
        ),
        PortSide::West => Point::new(
            bounds.origin.x,
            clamp(bounds.origin.y, bounds.max_y(), center.y + offset),
        ),
        PortSide::North => Point::new(
            clamp(bounds.origin.x, bounds.max_x(), center.x + offset),
            bounds.origin.y,
        ),
        PortSide::South => Point::new(
            clamp(bounds.origin.x, bounds.max_x(), center.x + offset),
            bounds.max_y(),
        ),
    }
}

fn extend_from_side(point: Point, side: PortSide, distance: f32) -> Point {
    match side {
        PortSide::East => Point::new(point.x + distance, point.y),
        PortSide::West => Point::new(point.x - distance, point.y),
        PortSide::North => Point::new(point.x, point.y - distance),
        PortSide::South => Point::new(point.x, point.y + distance),
    }
}

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    value.max(min).min(max)
}

fn same_point(left: Point, right: Point) -> bool {
    (left.x - right.x).abs() <= 0.1 && (left.y - right.y).abs() <= 0.1
}

fn endpoint_node_bounds(graph: &Graph, node_id: NodeId) -> Rect {
    let node = graph.node(node_id);
    let origin = if let Some(parent) = node.parent {
        let parent_bounds = endpoint_node_bounds(graph, parent);
        let local_origin = node.preferred_position.unwrap_or(node.bounds.origin);
        Point::new(
            parent_bounds.origin.x + local_origin.x,
            parent_bounds.origin.y + local_origin.y,
        )
    } else {
        node.bounds.origin
    };
    Rect::new(origin, node.bounds.size)
}

fn endpoint_center(graph: &Graph, endpoint: EdgeEndpoint) -> Point {
    if let Some(port_id) = endpoint.port {
        graph.port(port_id).bounds.center()
    } else {
        endpoint_node_bounds(graph, endpoint.node).center()
    }
}

fn route_self_loop(graph: &mut Graph, edge: &IrEdge, options: &LayoutOptions) {
    let node_bounds = graph.node(edge.routed_source.node).bounds;
    let start = anchor_point_towards(
        graph,
        edge.routed_source,
        Point::new(node_bounds.max_x() + 1.0, node_bounds.origin.y),
        options.view_profile,
    );
    let end = anchor_point_towards(
        graph,
        edge.routed_target,
        Point::new(node_bounds.origin.x, node_bounds.origin.y - 1.0),
        options.view_profile,
    );
    let gap = options.layered.spacing.edge_spacing.max(18.0);
    let loop_right = Point::new(
        node_bounds.max_x() + gap,
        node_bounds.origin.y + node_bounds.size.height / 4.0,
    );
    let loop_top = Point::new(node_bounds.max_x() + gap, node_bounds.origin.y - gap);
    let loop_left = Point::new(
        node_bounds.origin.x + node_bounds.size.width / 4.0,
        node_bounds.origin.y - gap,
    );
    graph.edge_mut(edge.original_edge).sections = vec![EdgeSection {
        start,
        bend_points: vec![loop_right, loop_top, loop_left],
        end,
    }];
}

fn anchor_point_towards(graph: &Graph, endpoint: EdgeEndpoint, toward: Point, profile: ViewProfile) -> Point {
    if let Some(port_id) = endpoint.port {
        return graph.port(port_id).bounds.center();
    }
    let bounds = endpoint_node_bounds(graph, endpoint.node);
    let side = choose_anchor_side(bounds, toward, profile);
    base_anchor_point(bounds, side, 0.0)
}

fn inflate_rect(rect: Rect, amount: f32) -> Rect {
    Rect::new(
        Point::new(rect.origin.x - amount, rect.origin.y - amount),
        Size::new(
            rect.size.width + amount * 2.0,
            rect.size.height + amount * 2.0,
        ),
    )
}

/// Bounding rect of all nodes in the graph so edge detours stay on-canvas.
fn graph_bounds(graph: &Graph) -> Rect {
    let (min_x, min_y, max_x, max_y) = graph.nodes.iter().fold(
        (f32::MAX, f32::MAX, f32::NEG_INFINITY, f32::NEG_INFINITY),
        |(min_x, min_y, max_x, max_y), node| {
            let r = node.bounds;
            (
                min_x.min(r.origin.x),
                min_y.min(r.origin.y),
                max_x.max(r.max_x()),
                max_y.max(r.max_y()),
            )
        },
    );
    if min_x <= max_x && min_y <= max_y {
        Rect::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    } else {
        Rect::new(Point::new(0.0, 0.0), Size::new(1000.0, 1000.0))
    }
}

/// Clamp a point to stay inside the rect, inset by margin (keeps edges on-canvas).
/// Uses at most half the rect size per axis so min <= max when rect is small.
fn clamp_point_to_rect(p: Point, rect: Rect, margin: f32) -> Point {
    let margin_x = margin.min(rect.size.width / 2.0).max(0.0);
    let margin_y = margin.min(rect.size.height / 2.0).max(0.0);
    let min_x = rect.origin.x + margin_x;
    let max_x = rect.max_x() - margin_x;
    let min_y = rect.origin.y + margin_y;
    let max_y = rect.max_y() - margin_y;
    Point::new(
        p.x.clamp(min_x, max_x),
        p.y.clamp(min_y, max_y),
    )
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
        x >= rect.origin.x && x <= rect.max_x() && max_y >= rect.origin.y && min_y <= rect.max_y()
    } else if (start.y - end.y).abs() <= f32::EPSILON {
        let y = start.y;
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        y >= rect.origin.y && y <= rect.max_y() && max_x >= rect.origin.x && min_x <= rect.max_x()
    } else {
        false
    }
}
