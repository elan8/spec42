use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutOptions, NodeAlignment, Point, Rect, Size};

use crate::ir::{IrNodeId, IrNodeKind, LayeredIr};
use crate::pipeline::util::{
    major_size, minor_size, node_minor_center, node_minor_start, placeholder_padding,
    set_node_minor_start,
};

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

    for node_id in layer {
        let blended = forward[&node_id] * compactness + backward[&node_id] * (1.0 - compactness);
        set_node_minor_start(ir, node_id, direction, blended.max(0.0));
        let center = node_minor_center(ir, node_id, direction);
        ir.nodes[node_id].aligned = (center - ir.nodes[node_id].desired_minor).abs()
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
