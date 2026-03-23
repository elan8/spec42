use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutOptions, NodeAlignment, Point, Rect, Size};
use elk_graph::{EdgeEndpoint, PortId};

use crate::ir::{IrNodeId, IrNodeKind, LayeredIr};
use crate::pipeline::orthogonal_routing_generator::{HyperEdgeSegment, assign_routing_slots};
use crate::pipeline::util::{
    major_size, minor_size, node_minor_center, node_minor_start, placeholder_padding,
    set_node_minor_start,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BkVDirection {
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BkHDirection {
    Right,
    Left,
}

#[derive(Clone)]
struct BkLayout {
    starts: Vec<f32>,
    x_center: Vec<f32>,
    min_start: f32,
    max_end: f32,
    width: f32,
}

fn bk_assign_minor_positions(ir: &mut LayeredIr, options: &LayoutOptions) {
    // Brandes–Köpf-inspired coordinate assignment (minor axis) for dense diagrams.
    // This is not a full ELK port yet (no type-1 conflict marking / port inside-block shifting),
    // but it follows the same overall structure: build alignments then compact blocks.
    if !options.layered.minor_axis_bk {
        return;
    }

    let n = ir.nodes.len();
    if n == 0 || ir.layers.len() <= 1 {
        return;
    }
    let spacing = options.layered.spacing.node_spacing.max(12.0);
    let direction = options.layered.direction;

    // Precompute immediate-layer neighbors (pred/succ) by layer adjacency.
    let mut preds: Vec<Vec<IrNodeId>> = vec![Vec::new(); n];
    let mut succs: Vec<Vec<IrNodeId>> = vec![Vec::new(); n];
    for e in &ir.normalized_edges {
        let from_layer = ir.nodes[e.from].layer;
        let to_layer = ir.nodes[e.to].layer;
        if to_layer == from_layer + 1 {
            preds[e.to].push(e.from);
            succs[e.from].push(e.to);
        }
    }
    for v in 0..n {
        preds[v].sort_by_key(|nid| ir.nodes[*nid].order);
        preds[v].dedup();
        succs[v].sort_by_key(|nid| ir.nodes[*nid].order);
        succs[v].dedup();
    }

    let mut half_width: Vec<f32> = Vec::with_capacity(n);
    for v in 0..n {
        half_width.push(
            minor_size(ir.nodes[v].size, direction) / 2.0
                + placeholder_padding(ir, v) / 2.0,
        );
    }
    let mut layouts: Vec<BkLayout> = Vec::with_capacity(4);
    for (vdir, hdir) in [
        (BkVDirection::Down, BkHDirection::Right),
        (BkVDirection::Down, BkHDirection::Left),
        (BkVDirection::Up, BkHDirection::Right),
        (BkVDirection::Up, BkHDirection::Left),
    ] {
        // --- Vertical alignment (build blocks) ---
        let mut root: Vec<IrNodeId> = (0..n).collect();
        let mut align: Vec<IrNodeId> = (0..n).collect();
        let mut pos_in_layer: Vec<usize> = vec![0; n];
        for layer in &ir.layers {
            for (i, &v) in layer.iter().enumerate() {
                pos_in_layer[v] = i;
            }
        }

        let layer_range: Box<dyn Iterator<Item = usize>> = match hdir {
            BkHDirection::Right => Box::new(0..ir.layers.len()),
            BkHDirection::Left => Box::new((0..ir.layers.len()).rev()),
        };

        for li in layer_range {
            let layer = ir.layers[li].clone();
            let iter: Box<dyn Iterator<Item = IrNodeId>> = match vdir {
                BkVDirection::Down => Box::new(layer.into_iter()),
                BkVDirection::Up => Box::new(layer.into_iter().rev()),
            };
            let mut r = match vdir {
                BkVDirection::Down => -1,
                BkVDirection::Up => i32::MAX,
            };

            for v in iter {
                let neigh = match hdir {
                    BkHDirection::Right => &preds[v],
                    BkHDirection::Left => &succs[v],
                };
                if neigh.is_empty() {
                    continue;
                }
                // median neighbor by layer order (ELK uses neighborhood info + conflicts; we skip conflicts).
                let d = neigh.len();
                // Align if median is “close” in order to reduce extreme widening.
                let low = (((d + 1) as f32 / 2.0).floor() as usize).saturating_sub(1);
                let high = (((d + 1) as f32 / 2.0).ceil() as usize).saturating_sub(1);
                match vdir {
                    BkVDirection::Up => {
                        for m in (low..=high).rev() {
                            if align[v] != v {
                                break;
                            }
                            let u = neigh[m];
                            let u_pos = pos_in_layer[u] as i32;
                            if r > u_pos {
                                align[u] = v;
                                root[v] = root[u];
                                align[v] = root[v];
                                r = u_pos;
                            }
                        }
                    }
                    BkVDirection::Down => {
                        for &u in &neigh[low..=high] {
                            if align[v] != v {
                                break;
                            }
                            let u_pos = pos_in_layer[u] as i32;
                            if r < u_pos {
                                align[u] = v;
                                root[v] = root[u];
                                align[v] = root[v];
                                r = u_pos;
                            }
                        }
                    }
                }
            }
        }

        let inner_shift = compute_block_inner_shifts(ir, &align, &root, &half_width, direction);
        let block_size = compute_block_sizes(ir, &root, &inner_shift, direction);
        let starts = bk_horizontal_compaction(
            ir,
            &root,
            &align,
            &inner_shift,
            &pos_in_layer,
            spacing,
            vdir,
            hdir,
            direction,
        );

        let mut x_center = vec![0.0f32; n];
        let mut min_start = f32::MAX;
        let mut max_end = 0.0f32;
        for v in 0..n {
            let start = starts[v];
            let end = start + minor_size(ir.nodes[v].size, direction) + placeholder_padding(ir, v);
            min_start = min_start.min(start);
            max_end = max_end.max(end);
            x_center[v] = start + half_width[v];
        }
        for v in 0..n {
            x_center[v] -= min_start;
        }
        let width = (max_end - min_start).max(0.0);
        layouts.push(BkLayout {
            starts,
            x_center,
            min_start,
            max_end,
            width,
        });
    }

    let produce_balanced_layout =
        options.layered.node_alignment == NodeAlignment::Balanced
            && !options.layered.prioritize_straight_edges;

    let chosen_layout = if produce_balanced_layout {
        let balanced = create_balanced_layout(ir, &layouts, direction);
        if check_layout_order_constraint(ir, &balanced, direction) {
            balanced
        } else {
            choose_smallest_feasible_layout(ir, layouts, direction)
        }
    } else {
        choose_smallest_feasible_layout(ir, layouts, direction)
    };

    // Apply chosen layout to node minor starts.
    for v in 0..n {
        let start = chosen_layout.starts[v];
        set_node_minor_start(ir, v, direction, start.max(0.0));
    }
}

fn choose_smallest_feasible_layout(
    ir: &LayeredIr,
    layouts: Vec<BkLayout>,
    direction: LayoutDirection,
) -> BkLayout {
    let mut chosen: Option<usize> = None;
    for (index, layout) in layouts.iter().enumerate() {
        if check_layout_order_constraint(ir, layout, direction)
            && chosen
                .map(|current| layouts[current].width > layout.width)
                .unwrap_or(true)
        {
            chosen = Some(index);
        }
    }
    chosen
        .map(|index| layouts[index].clone())
        .unwrap_or_else(|| layouts.into_iter().next().expect("at least one BK layout"))
}

fn create_balanced_layout(
    ir: &LayeredIr,
    layouts: &[BkLayout],
    direction: LayoutDirection,
) -> BkLayout {
    let mut min_width_layout = 0usize;
    for i in 1..layouts.len() {
        if layouts[i].width < layouts[min_width_layout].width {
            min_width_layout = i;
        }
    }

    let mut shifts = vec![0.0f32; layouts.len()];
    for (i, layout) in layouts.iter().enumerate() {
        shifts[i] = layouts[min_width_layout].min_start - layout.min_start;
    }

    let mut starts = vec![0.0f32; ir.nodes.len()];
    for v in 0..ir.nodes.len() {
        let mut positions = layouts
            .iter()
            .enumerate()
            .map(|(i, layout)| layout.starts[v] + shifts[i])
            .collect::<Vec<_>>();
        positions.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        starts[v] = if positions.len() >= 4 {
            (positions[1] + positions[2]) * 0.5
        } else {
            positions[positions.len() / 2]
        };
    }

    let mut min_start = f32::MAX;
    let mut max_end = f32::NEG_INFINITY;
    let mut x_center = vec![0.0; ir.nodes.len()];
    for v in 0..ir.nodes.len() {
        let start = starts[v];
        let end = start + minor_size(ir.nodes[v].size, direction) + placeholder_padding(ir, v);
        min_start = min_start.min(start);
        max_end = max_end.max(end);
        x_center[v] = start + minor_size(ir.nodes[v].size, direction) * 0.5
            + placeholder_padding(ir, v) * 0.5;
    }
    let width = (max_end - min_start).max(0.0);
    BkLayout {
        starts,
        x_center,
        min_start,
        max_end,
        width,
    }
}

fn check_layout_order_constraint(
    ir: &LayeredIr,
    layout: &BkLayout,
    direction: LayoutDirection,
) -> bool {
    for layer in &ir.layers {
        let mut pos = f32::NEG_INFINITY;
        for &node_id in layer {
            let start = layout.starts[node_id];
            let end = start + minor_size(ir.nodes[node_id].size, direction) + placeholder_padding(ir, node_id);
            if start > pos && end > pos {
                pos = end;
            } else {
                return false;
            }
        }
    }
    true
}

fn compute_block_inner_shifts(
    ir: &LayeredIr,
    align: &[IrNodeId],
    root: &[IrNodeId],
    half_width: &[f32],
    direction: LayoutDirection,
) -> Vec<f32> {
    let mut inner_shift = vec![0.0; ir.nodes.len()];
    let mut seen_roots = vec![false; ir.nodes.len()];

    for root_id in 0..ir.nodes.len() {
        if root[root_id] != root_id || seen_roots[root_id] {
            continue;
        }
        seen_roots[root_id] = true;
        inner_shift[root_id] = 0.0;

        let mut current = root_id;
        let mut steps = 0usize;
        let max_steps = ir.nodes.len().saturating_add(2);
        let mut min_start = -half_width[root_id];

        loop {
            let next = align[current];
            if next == root_id || next == current {
                break;
            }

            steps += 1;
            if steps > max_steps {
                break;
            }

            let center_delta =
                aligned_pair_center_delta(ir, current, next, direction).unwrap_or_default();
            inner_shift[next] = inner_shift[current] + center_delta;
            min_start = min_start.min(inner_shift[next] - half_width[next]);
            current = next;
        }

        if min_start < 0.0 {
            let block_offset = -min_start;
            let mut current = root_id;
            inner_shift[current] += block_offset;
            let mut steps = 0usize;
            loop {
                let next = align[current];
                if next == root_id || next == current {
                    break;
                }
                steps += 1;
                if steps > max_steps {
                    break;
                }
                inner_shift[next] += block_offset;
                current = next;
            }
        }
    }

    inner_shift
}

fn compute_block_sizes(
    ir: &LayeredIr,
    root: &[IrNodeId],
    inner_shift: &[f32],
    direction: LayoutDirection,
) -> Vec<f32> {
    let mut block_size = vec![0.0f32; ir.nodes.len()];
    for v in 0..ir.nodes.len() {
        let rv = root[v];
        block_size[rv] = block_size[rv].max(
            inner_shift[v] + minor_size(ir.nodes[v].size, direction) + placeholder_padding(ir, v),
        );
    }
    block_size
}

fn bk_horizontal_compaction(
    ir: &LayeredIr,
    root: &[IrNodeId],
    align: &[IrNodeId],
    inner_shift: &[f32],
    pos_in_layer: &[usize],
    spacing: f32,
    vdir: BkVDirection,
    hdir: BkHDirection,
    direction: LayoutDirection,
) -> Vec<f32> {
    let n = ir.nodes.len();
    let mut sink: Vec<IrNodeId> = (0..n).collect();
    let mut shift: Vec<f32> = vec![
        match vdir {
            BkVDirection::Up => f32::NEG_INFINITY,
            BkVDirection::Down => f32::INFINITY,
        };
        n
    ];
    let mut y: Vec<Option<f32>> = vec![None; n];
    let mut class_outgoing: Vec<Vec<(IrNodeId, f32)>> = vec![Vec::new(); n];
    let mut class_indegree: Vec<usize> = vec![0; n];

    let layers: Vec<Vec<IrNodeId>> = match hdir {
        BkHDirection::Right => ir.layers.clone(),
        BkHDirection::Left => ir.layers.iter().rev().cloned().collect(),
    };

    for layer in &layers {
        let nodes: Vec<IrNodeId> = match vdir {
            BkVDirection::Down => layer.clone(),
            BkVDirection::Up => layer.iter().rev().copied().collect(),
        };
        for v in nodes {
            if root[v] == v {
                bk_place_block(
                    v,
                    ir,
                    root,
                    align,
                    inner_shift,
                    &mut sink,
                    &mut y,
                    &mut class_outgoing,
                    &mut class_indegree,
                    pos_in_layer,
                    spacing,
                    vdir,
                    direction,
                );
            }
        }
    }

    let mut queue = std::collections::VecDeque::new();
    for class_root in 0..n {
        if class_indegree[class_root] == 0 && class_outgoing[class_root].len() + 1 > 0 {
            queue.push_back(class_root);
        }
    }
    while let Some(class_root) = queue.pop_front() {
        if shift[class_root].is_infinite() {
            shift[class_root] = 0.0;
        }
        for &(target, separation) in &class_outgoing[class_root] {
            let candidate = shift[class_root] + separation;
            if shift[target].is_infinite() {
                shift[target] = candidate;
            } else {
                shift[target] = match vdir {
                    BkVDirection::Down => shift[target].min(candidate),
                    BkVDirection::Up => shift[target].max(candidate),
                };
            }
            class_indegree[target] = class_indegree[target].saturating_sub(1);
            if class_indegree[target] == 0 {
                queue.push_back(target);
            }
        }
    }

    let mut starts = vec![0.0; n];
    for v in 0..n {
        let rv = root[v];
        let class_root = sink[rv];
        let class_shift = if shift[class_root].is_infinite() {
            0.0
        } else {
            shift[class_root]
        };
        let root_start = y[rv].unwrap_or(0.0) + class_shift;
        starts[v] = root_start + inner_shift[v];
    }
    starts
}

#[allow(clippy::too_many_arguments)]
fn bk_place_block(
    block_root: IrNodeId,
    ir: &LayeredIr,
    root: &[IrNodeId],
    align: &[IrNodeId],
    inner_shift: &[f32],
    sink: &mut [IrNodeId],
    y: &mut [Option<f32>],
    class_outgoing: &mut [Vec<(IrNodeId, f32)>],
    class_indegree: &mut [usize],
    pos_in_layer: &[usize],
    spacing: f32,
    vdir: BkVDirection,
    direction: LayoutDirection,
) {
    if y[block_root].is_some() {
        return;
    }

    let mut is_initial_assignment = true;
    y[block_root] = Some(0.0);
    let mut current = block_root;
    let max_steps = ir.nodes.len().saturating_add(2);
    let mut steps = 0usize;

    loop {
        steps += 1;
        if steps > max_steps {
            break;
        }
        let layer_nodes = &ir.layers[ir.nodes[current].layer];
        let current_index = pos_in_layer[current];
        let has_neighbor = match vdir {
            BkVDirection::Down => current_index > 0,
            BkVDirection::Up => current_index + 1 < layer_nodes.len(),
        };

        if has_neighbor {
            let neighbor = match vdir {
                BkVDirection::Down => layer_nodes[current_index - 1],
                BkVDirection::Up => layer_nodes[current_index + 1],
            };
            let neighbor_root = root[neighbor];
            bk_place_block(
                neighbor_root,
                ir,
                root,
                align,
                inner_shift,
                sink,
                y,
                class_outgoing,
                class_indegree,
                pos_in_layer,
                spacing,
                vdir,
                direction,
            );

            if sink[block_root] == block_root {
                sink[block_root] = sink[neighbor_root];
            }

            if sink[block_root] == sink[neighbor_root] {
                let new_position = match vdir {
                    BkVDirection::Up => {
                        y[neighbor_root].unwrap_or(0.0)
                            + inner_shift[neighbor]
                            - spacing
                            - minor_size(ir.nodes[current].size, direction)
                            - placeholder_padding(ir, current)
                            - inner_shift[current]
                    }
                    BkVDirection::Down => {
                        y[neighbor_root].unwrap_or(0.0)
                            + inner_shift[neighbor]
                            + minor_size(ir.nodes[neighbor].size, direction)
                            + placeholder_padding(ir, neighbor)
                            + spacing
                            - inner_shift[current]
                    }
                };
                y[block_root] = Some(if is_initial_assignment {
                    is_initial_assignment = false;
                    new_position
                } else {
                    match vdir {
                        BkVDirection::Up => y[block_root].unwrap_or(0.0).min(new_position),
                        BkVDirection::Down => y[block_root].unwrap_or(0.0).max(new_position),
                    }
                });
            } else {
                let required_space = match vdir {
                    BkVDirection::Up => {
                        y[block_root].unwrap_or(0.0)
                            + inner_shift[current]
                            + minor_size(ir.nodes[current].size, direction)
                            + placeholder_padding(ir, current)
                            + spacing
                            - (y[neighbor_root].unwrap_or(0.0) + inner_shift[neighbor])
                    }
                    BkVDirection::Down => {
                        y[block_root].unwrap_or(0.0)
                            + inner_shift[current]
                            - (y[neighbor_root].unwrap_or(0.0)
                                + inner_shift[neighbor]
                                + minor_size(ir.nodes[neighbor].size, direction)
                                + placeholder_padding(ir, neighbor)
                                + spacing)
                    }
                };
                let source_class = sink[block_root];
                let target_class = sink[neighbor_root];
                class_outgoing[source_class].push((target_class, required_space));
                class_indegree[target_class] += 1;
            }
        }

        current = align[current];
        if current == block_root || current == align[current] {
            break;
        }
    }
}

fn aligned_pair_center_delta(
    ir: &LayeredIr,
    current: IrNodeId,
    next: IrNodeId,
    direction: LayoutDirection,
) -> Option<f32> {
    let segment = ir
        .normalized_edges
        .iter()
        .find(|edge| {
            (edge.from == current && edge.to == next) || (edge.from == next && edge.to == current)
        })?;
    Some(
        normalized_segment_endpoint_center_offset(ir, segment, current, direction)
            - normalized_segment_endpoint_center_offset(ir, segment, next, direction),
    )
}

fn normalized_segment_endpoint_center_offset(
    ir: &LayeredIr,
    segment: &crate::ir::NormalizedEdge,
    node_id: IrNodeId,
    direction: LayoutDirection,
) -> f32 {
    let node = &ir.nodes[node_id];
    let half_size = minor_size(node.size, direction) / 2.0;
    let edge = &ir.edges[segment.edge_index];

    match node.kind {
        IrNodeKind::Real(real_node)
            if real_node == edge.effective_source && segment.from == node_id =>
        {
            endpoint_minor_anchor_offset(ir, edge.routed_source, direction)
                .map(|offset| offset - half_size)
                .unwrap_or_default()
        }
        IrNodeKind::Real(real_node)
            if real_node == edge.effective_target && segment.to == node_id =>
        {
            endpoint_minor_anchor_offset(ir, edge.routed_target, direction)
                .map(|offset| offset - half_size)
                .unwrap_or_default()
        }
        _ => 0.0,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct PlacementSummary {
    pub bounds: Rect,
    pub aligned_nodes: usize,
    pub compacted_layers: usize,
    pub straight_segments: usize,
}

pub(crate) fn place_nodes(ir: &mut LayeredIr, options: &LayoutOptions) -> PlacementSummary {
    let spacing = options.layered.spacing;
    let padding = options.layered.padding;
    let direction = options.layered.direction;
    let major_horizontal = matches!(
        direction,
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft
    );

    initialize_minor_positions(ir, options);
    bk_assign_minor_positions(ir, options);
    if options.layered.minor_axis_bk {
        for node_id in 0..ir.nodes.len() {
            let center = node_minor_center(ir, node_id, direction);
            ir.nodes[node_id].desired_minor = center;
            ir.nodes[node_id].aligned = true;
        }
    } else {
        for _ in 0..4 {
            for layer_index in 0..ir.layers.len() {
                compact_layer(ir, layer_index, options, true);
            }
            for layer_index in (0..ir.layers.len()).rev() {
                compact_layer(ir, layer_index, options, false);
            }
        }
    }

    assign_lanes(ir, options);

    let max_minor_span = ir
        .layers
        .iter()
        .map(|layer| layer_minor_span(ir, layer, direction, spacing.node_spacing))
        .fold(0.0, f32::max);

    let layer_extents: Vec<f32> = ir
        .layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|node_id| major_size(ir.nodes[*node_id].size, direction))
                .fold(0.0, f32::max)
        })
        .collect();
    let interlayer_routing_widths = compute_interlayer_routing_widths(ir, options);
    let total_major = layer_extents.iter().sum::<f32>()
        + interlayer_routing_widths.iter().sum::<f32>();
    let total_minor = max_minor_span;
    let bounds = if major_horizontal {
        Rect::new(
            Point::new(0.0, 0.0),
            Size::new(
                padding.left + total_major + padding.right,
                padding.top + total_minor + padding.bottom,
            ),
        )
    } else {
        Rect::new(
            Point::new(0.0, 0.0),
            Size::new(
                padding.left + total_minor + padding.right,
                padding.top + total_major + padding.bottom,
            ),
        )
    };

    let mut cursor_major = match direction {
        LayoutDirection::LeftToRight => padding.left,
        LayoutDirection::RightToLeft => bounds.size.width - padding.right,
        LayoutDirection::TopToBottom => padding.top,
        LayoutDirection::BottomToTop => bounds.size.height - padding.bottom,
    };

    let mut compacted_layers = 0usize;
    for (layer_index, layer) in ir.layers.clone().into_iter().enumerate() {
        let layer_extent = layer_extents[layer_index];
        let layer_span = layer_minor_span(ir, &layer, direction, spacing.node_spacing);
        let align_offset =
            alignment_offset(options.layered.node_alignment, max_minor_span, layer_span);
        if layer_span <= max_minor_span * (1.0 - (options.layered.compactness * 0.15)) {
            compacted_layers += 1;
        }

        let layer_major_start = match direction {
            LayoutDirection::LeftToRight => {
                let current = cursor_major;
                cursor_major += layer_extent
                    + interlayer_routing_widths
                        .get(layer_index)
                        .copied()
                        .unwrap_or(0.0);
                current
            }
            LayoutDirection::RightToLeft => {
                cursor_major -= layer_extent;
                let current = cursor_major;
                cursor_major -= interlayer_routing_widths
                    .get(layer_index.saturating_sub(1))
                    .copied()
                    .unwrap_or(0.0);
                current
            }
            LayoutDirection::TopToBottom => {
                let current = cursor_major;
                cursor_major += layer_extent
                    + interlayer_routing_widths
                        .get(layer_index)
                        .copied()
                        .unwrap_or(0.0);
                current
            }
            LayoutDirection::BottomToTop => {
                cursor_major -= layer_extent;
                let current = cursor_major;
                cursor_major -= interlayer_routing_widths
                    .get(layer_index.saturating_sub(1))
                    .copied()
                    .unwrap_or(0.0);
                current
            }
        };

        for node_id in layer {
            let minor = node_minor_start(ir, node_id, direction) + align_offset;
            ir.nodes[node_id].position = match direction {
                LayoutDirection::LeftToRight => Point::new(layer_major_start, padding.top + minor),
                LayoutDirection::RightToLeft => Point::new(layer_major_start, padding.top + minor),
                LayoutDirection::TopToBottom => Point::new(padding.left + minor, layer_major_start),
                LayoutDirection::BottomToTop => Point::new(padding.left + minor, layer_major_start),
            };
        }
    }

    let aligned_nodes = ir.nodes.iter().filter(|node| node.aligned).count();
    let straight_segments = count_straight_segments(ir, direction, spacing.segment_spacing);

    PlacementSummary {
        bounds,
        aligned_nodes,
        compacted_layers,
        straight_segments,
    }
}

fn compute_interlayer_routing_widths(ir: &LayeredIr, options: &LayoutOptions) -> Vec<f32> {
    if ir.layers.len() <= 1 {
        return Vec::new();
    }

    let edge_edge_spacing = options.layered.spacing.edge_spacing.max(1.0);
    let edge_node_spacing = options.layered.spacing.segment_spacing.max(1.0);
    let node_node_spacing = options.layered.spacing.layer_spacing;
    let mut widths = vec![0.0; ir.layers.len().saturating_sub(1)];

    for layer_index in 0..ir.layers.len().saturating_sub(1) {
        let mut min_lane = i32::MAX;
        let mut max_lane = i32::MIN;
        let mut has_non_straight = false;

        for edge in &ir.normalized_edges {
            let from_layer = ir.nodes[edge.from].layer;
            let to_layer = ir.nodes[edge.to].layer;
            if from_layer.min(to_layer) != layer_index || from_layer == to_layer {
                continue;
            }

            let from_minor = node_minor_center(ir, edge.from, options.layered.direction);
            let to_minor = node_minor_center(ir, edge.to, options.layered.direction);
            if (from_minor - to_minor).abs() <= 1e-3 {
                continue;
            }

            has_non_straight = true;
            min_lane = min_lane.min(edge.lane);
            max_lane = max_lane.max(edge.lane);
        }

        widths[layer_index] = if has_non_straight {
            let slots_count = (max_lane - min_lane + 1).max(1) as f32;
            let mut routing_width = (slots_count - 1.0) * edge_edge_spacing + edge_node_spacing * 2.0;
            routing_width = routing_width.max(node_node_spacing);
            routing_width
        } else {
            node_node_spacing
        };
    }

    widths
}

pub(crate) fn assign_lanes(ir: &mut LayeredIr, options: &LayoutOptions) {
    let mut layer_groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, edge) in ir.normalized_edges.iter().enumerate() {
        let layer = ir.nodes[edge.from].layer.min(ir.nodes[edge.to].layer);
        layer_groups.entry(layer).or_default().push(index);
    }

    for indices in layer_groups.values() {
        // Group by (from, to) segment then by bundle_key so same-type edges share lane 0
        let mut segment_bundles: BTreeMap<(IrNodeId, IrNodeId), Vec<Vec<usize>>> = BTreeMap::new();
        for &idx in indices {
            let ne = &ir.normalized_edges[idx];
            let bundle_key = ir.edges[ne.edge_index].bundle_key;
            let key = (ne.from, ne.to);
            let list = segment_bundles.entry(key).or_default();
            let found = match bundle_key {
                None => None,
                Some(k) => list.iter_mut().find(|bundle| {
                    bundle
                        .first()
                        .and_then(|&i| ir.edges[ir.normalized_edges[i].edge_index].bundle_key)
                        == Some(k)
                }),
            };
            if let Some(bundle) = found {
                bundle.push(idx);
            } else {
                list.push(vec![idx]);
            }
        }

        for bundles in segment_bundles.values_mut() {
            bundles.sort_by_key(|bundle| {
                let first = bundle[0];
                let ne = &ir.normalized_edges[first];
                (
                    ir.edges[ne.edge_index].bundle_key,
                    ir.nodes[ne.from].order,
                    ir.nodes[ne.to].order,
                    ne.segment_order,
                    ne.original_edge.index(),
                )
            });
            let preferred = options.layered.preferred_connector_lanes.max(1) as i32;
            let center = (bundles.len() as i32 - 1) / 2;
            for (slot, bundle) in bundles.iter().enumerate() {
                let relative = slot as i32 - center;
                let lane = if relative == 0 {
                    0
                } else {
                    let band = (relative.abs() - 1) / preferred + 1;
                    relative.signum() * band
                };
                let lane = if options.layered.prioritize_straight_edges {
                    lane
                } else {
                    lane * 2
                };
                for &edge_index in bundle {
                    ir.normalized_edges[edge_index].lane = lane;
                }
            }
        }
    }

    // Java-like refinement for orthogonal routing: use slot assignment with
    // conflict/crossing heuristics and deterministic cycle handling.
    refine_lanes_with_orthogonal_slots(ir, options);
}

fn refine_lanes_with_orthogonal_slots(ir: &mut LayeredIr, options: &LayoutOptions) {
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (idx, edge) in ir.normalized_edges.iter().enumerate() {
        let from_layer = ir.nodes[edge.from].layer;
        let to_layer = ir.nodes[edge.to].layer;
        if from_layer == to_layer {
            continue;
        }
        groups.entry(from_layer.min(to_layer)).or_default().push(idx);
    }

    for indices in groups.values() {
        if indices.len() <= 1 {
            continue;
        }
        let mut segments = Vec::with_capacity(indices.len());
        for (seg_id, &edge_idx) in indices.iter().enumerate() {
            let edge = &ir.normalized_edges[edge_idx];
            let (from_coord, to_coord) =
                normalized_segment_connection_coordinates(ir, edge_idx, options.layered.direction);
            let mut incoming = vec![from_coord];
            let mut outgoing = vec![to_coord];
            incoming.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            outgoing.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            segments.push(HyperEdgeSegment {
                id: seg_id,
                start_coordinate: from_coord,
                end_coordinate: to_coord,
                incoming_connection_coordinates: incoming,
                outgoing_connection_coordinates: outgoing,
                routing_slot: 0,
                in_weight: 0,
                out_weight: 0,
                incoming: Vec::new(),
                outgoing: Vec::new(),
                split_partner: None,
                split_by: None,
                mark: 0,
            });
        }
        let slots = assign_routing_slots(segments, options.layered.spacing.edge_spacing.max(1.0));
        for (seg_id, &edge_idx) in indices.iter().enumerate() {
            if let Some(slot) = slots.get(seg_id) {
                ir.normalized_edges[edge_idx].lane = *slot;
            }
        }

        // Degenerate case: identical geometry can collapse all slots to one value.
        // Spread lanes deterministically so parallel connectors remain visually distinct.
        let mut unique = indices
            .iter()
            .map(|&idx| ir.normalized_edges[idx].lane)
            .collect::<Vec<_>>();
        unique.sort_unstable();
        unique.dedup();
        if unique.len() <= 1 && indices.len() > 1 {
            let mut ordered = indices
                .iter()
                .copied()
                .collect::<Vec<_>>();
            ordered.sort_by_key(|&idx| {
                let ne = &ir.normalized_edges[idx];
                let edge = &ir.edges[ne.edge_index];
                (edge.bundle_key, edge.model_order, ne.segment_order, ne.original_edge.index())
            });
            let center = (ordered.len() as i32 - 1) / 2;
            for (i, idx) in ordered.into_iter().enumerate() {
                ir.normalized_edges[idx].lane = i as i32 - center;
            }
        }
    }
}

fn normalized_segment_connection_coordinates(
    ir: &LayeredIr,
    normalized_edge_index: usize,
    direction: LayoutDirection,
) -> (f32, f32) {
    let normalized = &ir.normalized_edges[normalized_edge_index];
    let edge = &ir.edges[normalized.edge_index];

    let from_coord = if normalized.segment_order == 0 {
        endpoint_minor_anchor_position(ir, edge.routed_source, direction)
    } else {
        node_minor_center(ir, normalized.from, direction)
    };

    let last_segment_order = edge.chain.len();
    let to_coord = if normalized.segment_order == last_segment_order {
        endpoint_minor_anchor_position(ir, edge.routed_target, direction)
    } else {
        node_minor_center(ir, normalized.to, direction)
    };

    (from_coord, to_coord)
}

fn initialize_minor_positions(ir: &mut LayeredIr, options: &LayoutOptions) {
    let direction = options.layered.direction;
    let spacing = options.layered.spacing;
    for layer in ir.layers.clone() {
        let mut cursor = 0.0f32;
        for node_id in layer {
            set_node_minor_start(ir, node_id, direction, cursor);
            cursor += minor_size(ir.nodes[node_id].size, direction)
                + spacing.node_spacing
                + placeholder_padding(ir, node_id);
        }
    }
}

fn compact_layer(ir: &mut LayeredIr, layer_index: usize, options: &LayoutOptions, incoming: bool) {
    let direction = options.layered.direction;
    let spacing = options.layered.spacing;
    let compactness = options.layered.compactness.clamp(0.0, 1.0);
    let layer = ir.layers[layer_index].clone();
    if layer.is_empty() {
        return;
    }

    let mut desired_starts = Vec::new();
    let mut cursor = 0.0f32;
    for node_id in &layer {
        let preferred_center = preferred_minor_center(ir, *node_id, incoming, options);
        let size = minor_size(ir.nodes[*node_id].size, direction);
        ir.nodes[*node_id].desired_minor = preferred_center;
        desired_starts.push((
            *node_id,
            (preferred_center - size / 2.0).max(0.0),
            size + placeholder_padding(ir, *node_id),
        ));
    }

    let mut forward = BTreeMap::new();
    for (node_id, desired_start, footprint) in &desired_starts {
        let placed = desired_start.max(cursor);
        forward.insert(*node_id, placed);
        cursor = placed + *footprint + spacing.node_spacing * (0.4 + compactness * 0.6);
    }

    let mut backward = BTreeMap::new();
    let mut tail = cursor;
    for (node_id, desired_start, footprint) in desired_starts.iter().rev() {
        let placed = desired_start.min((tail - *footprint).max(0.0));
        backward.insert(*node_id, placed);
        tail = (placed - spacing.node_spacing * compactness.max(0.2)).max(0.0);
    }

    for node_id in &layer {
        let blended = forward[node_id] * compactness + backward[node_id] * (1.0 - compactness);
        set_node_minor_start(ir, *node_id, direction, blended.max(0.0));
    }

    // Overlap-resolution pass: blending can reorder nodes; enforce minimum separation.
    let mut resolved: Vec<(IrNodeId, f32, f32)> = layer
        .iter()
        .map(|&node_id| {
            let start = node_minor_start(ir, node_id, direction);
            let footprint =
                minor_size(ir.nodes[node_id].size, direction) + placeholder_padding(ir, node_id);
            (node_id, start, footprint)
        })
        .collect();
    resolved.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut prev_end = 0.0f32;
    for (node_id, start, footprint) in &mut resolved {
        let min_start = prev_end;
        let actual = (*start).max(min_start);
        *start = actual;
        set_node_minor_start(ir, *node_id, direction, actual);
        prev_end = actual + *footprint + spacing.node_spacing;
    }

    for node_id in &layer {
        let center = node_minor_center(ir, *node_id, direction);
        ir.nodes[*node_id].aligned = (center - ir.nodes[*node_id].desired_minor).abs()
            <= options.layered.spacing.segment_spacing.max(12.0);
    }
}

fn preferred_minor_center(
    ir: &LayeredIr,
    node_id: IrNodeId,
    incoming: bool,
    options: &LayoutOptions,
) -> f32 {
    let direction = options.layered.direction;
    let mut neighbors = preferred_minor_centers_from_ports(ir, node_id, incoming, direction);
    if neighbors.is_empty() {
        for edge in &ir.normalized_edges {
            if incoming && edge.to == node_id {
                neighbors.push(
                    node_minor_center(ir, edge.from, direction)
                        + (edge.lane as f32 * options.layered.spacing.segment_spacing),
                );
            } else if !incoming && edge.from == node_id {
                neighbors.push(
                    node_minor_center(ir, edge.to, direction)
                        - (edge.lane as f32 * options.layered.spacing.segment_spacing),
                );
            }
        }
    }

    if neighbors.is_empty() {
        return node_minor_center(ir, node_id, direction);
    }

    let average = neighbors.iter().sum::<f32>() / neighbors.len() as f32;
    let _ = options;
    average
}

fn preferred_minor_centers_from_ports(
    ir: &LayeredIr,
    node_id: IrNodeId,
    incoming: bool,
    direction: LayoutDirection,
) -> Vec<f32> {
    let crate::ir::IrNodeKind::Real(real_node) = ir.nodes[node_id].kind else {
        return Vec::new();
    };

    let mut neighbors = Vec::new();
    for edge in &ir.edges {
        let (neighbor_endpoint, own_endpoint, matches_direction) = if incoming {
            (
                edge.routed_source,
                edge.routed_target,
                edge.effective_target == real_node,
            )
        } else {
            (
                edge.routed_target,
                edge.routed_source,
                edge.effective_source == real_node,
            )
        };
        if !matches_direction {
            continue;
        }

        let Some(own_offset) = endpoint_minor_anchor_offset(ir, own_endpoint, direction) else {
            continue;
        };
        let neighbor_anchor = endpoint_minor_anchor_position(ir, neighbor_endpoint, direction);
        neighbors.push(
            neighbor_anchor - own_offset + minor_size(ir.nodes[node_id].size, direction) / 2.0,
        );
    }

    neighbors
}

fn endpoint_minor_anchor_position(
    ir: &LayeredIr,
    endpoint: EdgeEndpoint,
    direction: LayoutDirection,
) -> f32 {
    if let Some(offset) = endpoint_minor_anchor_offset(ir, endpoint, direction) {
        if let Some(node_id) = ir.real_to_ir.get(&endpoint.node).copied() {
            return node_minor_start(ir, node_id, direction) + offset;
        }
    }
    ir.real_to_ir
        .get(&endpoint.node)
        .copied()
        .map(|node_id| node_minor_center(ir, node_id, direction))
        .unwrap_or_default()
}

fn endpoint_minor_anchor_offset(
    ir: &LayeredIr,
    endpoint: EdgeEndpoint,
    direction: LayoutDirection,
) -> Option<f32> {
    let port_id = endpoint.port?;
    let node_id = ir.real_to_ir.get(&endpoint.node).copied()?;
    port_minor_anchor_offset(&ir.nodes[node_id], port_id, direction)
}

fn port_minor_anchor_offset(
    node: &crate::ir::IrNode,
    port_id: PortId,
    direction: LayoutDirection,
) -> Option<f32> {
    let port = node.ports.iter().find(|port| port.port_id == port_id)?;
    let mut same_side = node
        .ports
        .iter()
        .filter(|candidate| candidate.side == port.side)
        .collect::<Vec<_>>();
    same_side.sort_by_key(|candidate| (candidate.order, candidate.port_id.index()));
    let rank = same_side
        .iter()
        .position(|candidate| candidate.port_id == port_id)?;
    let count = same_side.len().max(1) as f32;
    let fraction = (rank as f32 + 1.0) / (count + 1.0);

    Some(match direction {
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => match port.side {
            elk_core::PortSide::East | elk_core::PortSide::West => node.size.height * fraction,
            elk_core::PortSide::North => 0.0,
            elk_core::PortSide::South => node.size.height,
        },
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => match port.side {
            elk_core::PortSide::North | elk_core::PortSide::South => node.size.width * fraction,
            elk_core::PortSide::West => 0.0,
            elk_core::PortSide::East => node.size.width,
        },
    })
}

fn alignment_offset(alignment: NodeAlignment, max_span: f32, layer_span: f32) -> f32 {
    let slack = (max_span - layer_span).max(0.0);
    match alignment {
        NodeAlignment::Start => 0.0,
        NodeAlignment::Center => slack / 2.0,
        NodeAlignment::End => slack,
        NodeAlignment::Balanced => slack / 2.0,
    }
}

fn layer_minor_span(
    ir: &LayeredIr,
    layer: &[IrNodeId],
    direction: LayoutDirection,
    spacing: f32,
) -> f32 {
    if layer.is_empty() {
        return 0.0;
    }
    let start = layer
        .iter()
        .map(|node_id| node_minor_start(ir, *node_id, direction))
        .fold(f32::MAX, f32::min);
    let end = layer
        .iter()
        .map(|node_id| {
            node_minor_start(ir, *node_id, direction)
                + minor_size(ir.nodes[*node_id].size, direction)
                + placeholder_padding(ir, *node_id)
        })
        .fold(0.0, f32::max);
    (end - start + spacing).max(0.0)
}

fn count_straight_segments(ir: &LayeredIr, direction: LayoutDirection, tolerance: f32) -> usize {
    ir.normalized_edges
        .iter()
        .filter(|edge| {
            let from = node_minor_center(ir, edge.from, direction);
            let to = node_minor_center(ir, edge.to, direction);
            (from - to).abs() <= tolerance
        })
        .count()
}
