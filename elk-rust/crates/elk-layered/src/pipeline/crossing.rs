use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::ir::{IrNodeId, IrNodeKind, LayeredIr};

pub(crate) fn minimize_crossings(ir: &mut LayeredIr, crossing_sweeps: &mut Vec<usize>) {
    refresh_orders(ir);
    for _ in 0..4 {
        for layer_index in 1..ir.layers.len() {
            reorder_layer(ir, layer_index, true);
        }
        for layer_index in (0..ir.layers.len().saturating_sub(1)).rev() {
            reorder_layer(ir, layer_index, false);
        }
        crossing_sweeps.push(count_crossings(ir));
    }
}

pub(crate) fn count_crossings(ir: &LayeredIr) -> usize {
    let mut total = 0usize;
    for window in ir.layers.windows(2) {
        let left_layer: BTreeSet<_> = window[0].iter().copied().collect();
        let right_layer: BTreeSet<_> = window[1].iter().copied().collect();
        let mut edges = Vec::new();

        for edge in &ir.normalized_edges {
            if left_layer.contains(&edge.from) && right_layer.contains(&edge.to) {
                edges.push((
                    ir.nodes[edge.from].order,
                    ir.nodes[edge.to].order,
                    edge.original_edge.index(),
                    edge.segment_order,
                ));
            }
        }

        for index in 0..edges.len() {
            for other in (index + 1)..edges.len() {
                let (a1, b1, edge1, segment1) = edges[index];
                let (a2, b2, edge2, segment2) = edges[other];
                if edge1 == edge2 && segment1.abs_diff(segment2) <= 1 {
                    continue;
                }
                if (a1 < a2 && b1 > b2) || (a1 > a2 && b1 < b2) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn reorder_layer(ir: &mut LayeredIr, layer_index: usize, incoming: bool) {
    let current = ir.layers[layer_index].clone();
    let mut weights = Vec::new();
    for node_id in &current {
        weights.push((*node_id, barycenter(ir, *node_id, incoming)));
    }
    weights.sort_by(|(left_node, left_weight), (right_node, right_weight)| {
        left_weight
            .partial_cmp(right_weight)
            .unwrap_or(Ordering::Equal)
            .then_with(|| node_sort_key(ir, *left_node).cmp(&node_sort_key(ir, *right_node)))
    });
    ir.layers[layer_index] = weights.into_iter().map(|(node_id, _)| node_id).collect();
    refresh_orders(ir);
}

fn node_sort_key(ir: &LayeredIr, node_id: IrNodeId) -> (usize, usize, usize) {
    let node = &ir.nodes[node_id];
    match node.kind {
        IrNodeKind::Real(real) => (
            0usize,
            node.model_order,
            real.index().saturating_add(ir.nodes[node_id].ports.len()),
        ),
        IrNodeKind::LabelPlaceholder { edge_index } => (1usize, node.model_order, edge_index),
        IrNodeKind::Dummy {
            edge_index,
            segment_index,
        } => (
            2usize,
            node.model_order,
            edge_index.saturating_mul(1024) + segment_index,
        ),
    }
}

fn barycenter(ir: &LayeredIr, node_id: IrNodeId, incoming: bool) -> f32 {
    let mut neighbors = Vec::new();
    for edge in &ir.normalized_edges {
        if incoming && edge.to == node_id {
            neighbors.push(ir.nodes[edge.from].order as f32);
        } else if !incoming && edge.from == node_id {
            neighbors.push(ir.nodes[edge.to].order as f32);
        }
    }
    if neighbors.is_empty() {
        return ir.nodes[node_id].model_order as f32;
    }
    neighbors.iter().sum::<f32>() / neighbors.len() as f32
}

fn refresh_orders(ir: &mut LayeredIr) {
    for layer in &ir.layers {
        for (order, node_id) in layer.iter().copied().enumerate() {
            ir.nodes[node_id].order = order;
        }
    }
}
