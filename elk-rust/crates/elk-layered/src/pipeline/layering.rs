use std::collections::BTreeMap;

use elk_core::{LayerConstraint, LayoutOptions, NodeId};

use crate::ir::{IrNodeKind, LayeredIr};

pub(crate) fn assign_layers(ir: &mut LayeredIr, options: &LayoutOptions) {
    let real_nodes: Vec<NodeId> = ir
        .nodes
        .iter()
        .filter_map(|node| match node.kind {
            IrNodeKind::Real(node_id) => Some(node_id),
            _ => None,
        })
        .collect();
    let mut indegree: BTreeMap<NodeId, usize> = real_nodes
        .iter()
        .copied()
        .map(|node| (node, 0usize))
        .collect();
    let mut outgoing: BTreeMap<NodeId, Vec<NodeId>> = real_nodes
        .iter()
        .copied()
        .map(|node| (node, Vec::new()))
        .collect();

    for edge in &ir.edges {
        if edge.self_loop {
            continue;
        }
        *indegree.entry(edge.effective_target).or_default() += 1;
        outgoing
            .entry(edge.effective_source)
            .or_default()
            .push(edge.effective_target);
    }

    let mut ready: Vec<NodeId> = indegree
        .iter()
        .filter_map(|(node, indegree)| (*indegree == 0).then_some(*node))
        .collect();
    ready.sort_by_key(|node| sort_key(ir, *node));

    let mut order = Vec::new();
    while let Some(node) = ready.first().copied() {
        ready.remove(0);
        order.push(node);
        if let Some(targets) = outgoing.get(&node) {
            for target in targets {
                if let Some(value) = indegree.get_mut(target) {
                    *value = value.saturating_sub(1);
                    if *value == 0 {
                        ready.push(*target);
                        ready.sort_by_key(|candidate| sort_key(ir, *candidate));
                    }
                }
            }
        }
    }

    let mut layer_of: BTreeMap<NodeId, usize> = BTreeMap::new();
    for node in &order {
        let layer = layer_of.get(node).copied().unwrap_or_default();
        if let Some(targets) = outgoing.get(node) {
            for target in targets {
                let next_layer = layer + 1;
                layer_of
                    .entry(*target)
                    .and_modify(|value| *value = (*value).max(next_layer))
                    .or_insert(next_layer);
            }
        }
    }

    for node in &real_nodes {
        layer_of.entry(*node).or_insert(0usize);
    }

    for node in &real_nodes {
        let ir_node = ir.real_to_ir[node];
        if ir.nodes[ir_node].layer_constraint == LayerConstraint::First {
            layer_of.insert(*node, 0);
        }
    }

    let mut max_layer = layer_of.values().copied().max().unwrap_or_default();
    for node in &real_nodes {
        let ir_node = ir.real_to_ir[node];
        if ir.nodes[ir_node].layer_constraint == LayerConstraint::Last {
            max_layer = max_layer.max(layer_of[node] + 1);
        }
    }
    for node in &real_nodes {
        let ir_node = ir.real_to_ir[node];
        if ir.nodes[ir_node].layer_constraint == LayerConstraint::Last {
            layer_of.insert(*node, max_layer);
        }
    }

    ir.layers.clear();
    let max_layer = layer_of.values().copied().max().unwrap_or_default();
    ir.layers.resize_with(max_layer + 1, Vec::new);

    for node_id in real_nodes {
        let ir_node = ir.real_to_ir[&node_id];
        let layer = layer_of[&node_id];
        ir.nodes[ir_node].layer = layer;
        ir.layers[layer].push(ir_node);
    }

    for layer_index in 0..ir.layers.len() {
        let mut layer = ir.layers[layer_index].clone();
        layer.sort_by_key(|node_id| sort_ir_key(ir, *node_id));
        for (order, node_id) in layer.iter().copied().enumerate() {
            ir.nodes[node_id].order = order;
        }
        ir.layers[layer_index] = layer;
    }

    if options.layered.merge_layers {
        merge_layers_for_interconnection_view(ir);
    }
}

/// For InterconnectionView, merge consecutive layers when no edge goes from layer i+1 to layer i,
/// so we get fewer layers and shorter vertical spans.
fn merge_layers_for_interconnection_view(ir: &mut LayeredIr) {
    loop {
        let mut merged = false;
        let num_layers = ir.layers.len();
        if num_layers < 2 {
            break;
        }
        for i in 0..num_layers - 1 {
            if has_back_edge(ir, i, i + 1) {
                continue;
            }
            let layer_i1: Vec<_> = ir.layers.remove(i + 1);
            for &ir_node in &layer_i1 {
                ir.nodes[ir_node].layer = i;
            }
            ir.layers[i].extend(layer_i1);
            for layer_index in (i + 1)..ir.layers.len() {
                for &ir_node in &ir.layers[layer_index] {
                    ir.nodes[ir_node].layer = layer_index;
                }
            }
            let mut layer = ir.layers[i].clone();
            layer.sort_by_key(|node_id| sort_ir_key(ir, *node_id));
            for (order, node_id) in layer.iter().copied().enumerate() {
                ir.nodes[node_id].order = order;
            }
            ir.layers[i] = layer;
            merged = true;
            break;
        }
        if !merged {
            break;
        }
    }
}

fn has_back_edge(ir: &LayeredIr, from_layer: usize, to_layer: usize) -> bool {
    ir.edges.iter().any(|e| {
        if e.self_loop {
            return false;
        }
        let Some(&src_ir) = ir.real_to_ir.get(&e.effective_source) else {
            return false;
        };
        let Some(&tgt_ir) = ir.real_to_ir.get(&e.effective_target) else {
            return false;
        };
        ir.nodes[src_ir].layer == from_layer && ir.nodes[tgt_ir].layer == to_layer
    })
}

fn sort_key(ir: &LayeredIr, node_id: NodeId) -> (usize, usize) {
    let ir_node = ir.real_to_ir[&node_id];
    let node = &ir.nodes[ir_node];
    let constraint_rank = match node.layer_constraint {
        LayerConstraint::First => 0usize,
        LayerConstraint::None => 1usize,
        LayerConstraint::Last => 2usize,
    };
    (constraint_rank, node.model_order)
}

fn sort_ir_key(ir: &LayeredIr, node_id: usize) -> (usize, usize, usize) {
    let node = &ir.nodes[node_id];
    let constraint_rank = match node.layer_constraint {
        LayerConstraint::First => 0usize,
        LayerConstraint::None => 1usize,
        LayerConstraint::Last => 2usize,
    };
    let real = match node.kind {
        IrNodeKind::Real(real) => real.index(),
        _ => usize::MAX,
    };
    (constraint_rank, node.model_order, real)
}
