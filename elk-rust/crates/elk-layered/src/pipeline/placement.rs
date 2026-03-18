use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutOptions, NodeAlignment, Point, Rect, Size};

use crate::ir::{IrNodeId, IrNodeKind, LayeredIr};
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

fn bk_assign_minor_positions(ir: &mut LayeredIr, options: &LayoutOptions) {
    // Brandes–Köpf-inspired coordinate assignment (minor axis) for dense diagrams.
    // This is not a full ELK port yet (no type-1 conflict marking / port inside-block shifting),
    // but it follows the same overall structure: build alignments then compact blocks.
    if !options.layered.minor_axis_bk {
        return;
    }
    if options.layered.direction != LayoutDirection::TopToBottom {
        return;
    }

    let n = ir.nodes.len();
    if n == 0 || ir.layers.len() <= 1 {
        return;
    }
    let spacing = options.layered.spacing.node_spacing.max(12.0);

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
            minor_size(ir.nodes[v].size, LayoutDirection::TopToBottom) / 2.0
                + placeholder_padding(ir, v) / 2.0,
        );
    }
    let sep = |u: IrNodeId, v: IrNodeId| -> f32 { half_width[u] + half_width[v] + spacing };

    #[derive(Clone)]
    struct Layout {
        x_center: Vec<f32>,
        width: f32,
    }

    let mut layouts: Vec<Layout> = Vec::with_capacity(4);
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

        let layer_range: Box<dyn Iterator<Item = usize>> = match vdir {
            BkVDirection::Down => Box::new(1..ir.layers.len()),
            BkVDirection::Up => Box::new((0..ir.layers.len() - 1).rev()),
        };

        for li in layer_range {
            let layer = ir.layers[li].clone();
            let iter: Box<dyn Iterator<Item = IrNodeId>> = match hdir {
                BkHDirection::Right => Box::new(layer.into_iter()),
                BkHDirection::Left => Box::new(layer.into_iter().rev()),
            };

            for v in iter {
                let neigh = match vdir {
                    BkVDirection::Down => &preds[v],
                    BkVDirection::Up => &succs[v],
                };
                if neigh.is_empty() {
                    continue;
                }
                // median neighbor by layer order (ELK uses neighborhood info + conflicts; we skip conflicts).
                let median = neigh[neigh.len() / 2];
                // Align if median is “close” in order to reduce extreme widening.
                let dv = pos_in_layer[v] as i32 - pos_in_layer[median] as i32;
                if dv.abs() > 6 {
                    continue;
                }
                align[v] = median;
                root[v] = root[median];
                align[median] = v;
            }
        }

        // --- Horizontal compaction (place blocks) ---
        let mut sink: Vec<IrNodeId> = (0..n).collect();
        let mut shift: Vec<f32> = vec![f32::INFINITY; n];
        let mut x: Vec<f32> = vec![0.0; n]; // block/root coordinates
        let mut placed: Vec<bool> = vec![false; n];

        fn place_block(
            v: IrNodeId,
            ir: &LayeredIr,
            hdir: BkHDirection,
            vdir: BkVDirection,
            preds: &Vec<Vec<IrNodeId>>,
            succs: &Vec<Vec<IrNodeId>>,
            align: &Vec<IrNodeId>,
            root: &Vec<IrNodeId>,
            sink: &mut Vec<IrNodeId>,
            shift: &mut Vec<f32>,
            x: &mut Vec<f32>,
            placed: &mut Vec<bool>,
            sep: &dyn Fn(IrNodeId, IrNodeId) -> f32,
            pos_in_layer: &Vec<usize>,
        ) {
            let rv = root[v];
            if placed[rv] {
                return;
            }
            placed[rv] = true;
            x[rv] = 0.0;

            // Walk through the block via align pointers.
            let mut w = v;
            let mut steps = 0usize;
            let max_steps = ir.nodes.len().saturating_add(2);
            loop {
                steps += 1;
                if steps > max_steps {
                    // Defensive: alignment pointers should form a simple cycle back to `v`,
                    // but if they don't, break to avoid infinite loops.
                    break;
                }
                // Determine neighbor in same layer that constrains w (predecessor/successor by hdir).
                let layer = ir.nodes[w].layer;
                let layer_nodes = &ir.layers[layer];
                let idx = pos_in_layer[w];
                let neighbor_same_layer: Option<IrNodeId> = match hdir {
                    BkHDirection::Right => idx.checked_sub(1).and_then(|i| layer_nodes.get(i).copied()),
                    BkHDirection::Left => layer_nodes.get(idx + 1).copied(),
                };

                if let Some(u) = neighbor_same_layer {
                    let ru = root[u];
                    place_block(
                        u, ir, hdir, vdir, preds, succs, align, root, sink, shift, x, placed, sep,
                        pos_in_layer,
                    );
                    let delta = match hdir {
                        BkHDirection::Right => sep(u, w),
                        BkHDirection::Left => sep(w, u),
                    };
                    if sink[rv] == rv {
                        sink[rv] = sink[ru];
                    }
                    if sink[rv] == sink[ru] {
                        x[rv] = x[rv].max(x[ru] + delta);
                    } else {
                        shift[sink[ru]] = shift[sink[ru]].min(x[rv] - (x[ru] + delta));
                    }
                }

                w = align[w];
                if w == v {
                    break;
                }
                // Avoid pathological cycles if alignment got weird.
                if align[w] == w {
                    break;
                }
            }
        }

        // Place blocks in traversal order.
        let layers_iter: Box<dyn Iterator<Item = usize>> = match vdir {
            BkVDirection::Down => Box::new(0..ir.layers.len()),
            BkVDirection::Up => Box::new((0..ir.layers.len()).rev()),
        };
        for li in layers_iter {
            let layer = ir.layers[li].clone();
            let iter: Box<dyn Iterator<Item = IrNodeId>> = match hdir {
                BkHDirection::Right => Box::new(layer.into_iter()),
                BkHDirection::Left => Box::new(layer.into_iter().rev()),
            };
            for v in iter {
                place_block(
                    v,
                    ir,
                    hdir,
                    vdir,
                    &preds,
                    &succs,
                    &align,
                    &root,
                    &mut sink,
                    &mut shift,
                    &mut x,
                    &mut placed,
                    &sep,
                    &pos_in_layer,
                );
            }
        }
        for i in 0..n {
            if shift[i].is_infinite() {
                shift[i] = 0.0;
            }
        }

        // Resolve final center x for each node.
        let mut x_center = vec![0.0f32; n];
        for v in 0..n {
            let rv = root[v];
            let s = sink[rv];
            x_center[v] = x[rv] + shift[s];
        }

        // Normalize to start at 0 and compute width.
        let mut min_start = f32::MAX;
        let mut max_end = 0.0f32;
        for v in 0..n {
            let start = x_center[v] - half_width[v];
            let end = x_center[v] + half_width[v];
            min_start = min_start.min(start);
            max_end = max_end.max(end);
        }
        for v in 0..n {
            x_center[v] -= min_start;
        }
        let width = (max_end - min_start).max(0.0);
        layouts.push(Layout { x_center, width });
    }

    // Choose tightest layout (ELK would consider errors + balanced median; we start with tightest).
    let mut best = 0usize;
    for i in 1..layouts.len() {
        if layouts[i].width < layouts[best].width {
            best = i;
        }
    }

    // Apply best layout to node minor starts.
    for v in 0..n {
        let start = layouts[best].x_center[v] - half_width[v];
        set_node_minor_start(ir, v, LayoutDirection::TopToBottom, start.max(0.0));
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
    for _ in 0..4 {
        for layer_index in 0..ir.layers.len() {
            compact_layer(ir, layer_index, options, true);
        }
        for layer_index in (0..ir.layers.len()).rev() {
            compact_layer(ir, layer_index, options, false);
        }
    }

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
    let total_major = layer_extents.iter().sum::<f32>()
        + spacing.layer_spacing * ir.layers.len().saturating_sub(1) as f32;
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
                cursor_major += layer_extent + spacing.layer_spacing;
                current
            }
            LayoutDirection::RightToLeft => {
                cursor_major -= layer_extent;
                let current = cursor_major;
                cursor_major -= spacing.layer_spacing;
                current
            }
            LayoutDirection::TopToBottom => {
                let current = cursor_major;
                cursor_major += layer_extent + spacing.layer_spacing;
                current
            }
            LayoutDirection::BottomToTop => {
                cursor_major -= layer_extent;
                let current = cursor_major;
                cursor_major -= spacing.layer_spacing;
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

    assign_lanes(ir, options);
    let aligned_nodes = ir.nodes.iter().filter(|node| node.aligned).count();
    let straight_segments = count_straight_segments(ir, direction, spacing.segment_spacing);

    PlacementSummary {
        bounds,
        aligned_nodes,
        compacted_layers,
        straight_segments,
    }
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
    let mut neighbors = Vec::new();
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

    if neighbors.is_empty() {
        return node_minor_center(ir, node_id, direction);
    }

    let average = neighbors.iter().sum::<f32>() / neighbors.len() as f32;
    if options.layered.prioritize_straight_edges
        && matches!(
            ir.nodes[node_id].kind,
            IrNodeKind::Dummy { .. } | IrNodeKind::LabelPlaceholder { .. }
        )
    {
        average
    } else {
        (average + node_minor_center(ir, node_id, direction)) / 2.0
    }
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
