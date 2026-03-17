use std::collections::{BTreeMap, BTreeSet};

use elk_core::NodeId;

use crate::ir::{IrNodeKind, LayeredIr};

pub(crate) fn break_cycles(ir: &mut LayeredIr) -> usize {
    let real_nodes: Vec<NodeId> = ir
        .nodes
        .iter()
        .filter_map(|node| match node.kind {
            IrNodeKind::Real(node_id) => Some(node_id),
            _ => None,
        })
        .collect();
    let mut remaining: BTreeSet<NodeId> = real_nodes.iter().copied().collect();
    let mut left = Vec::new();
    let mut right = Vec::new();

    while !remaining.is_empty() {
        let sinks: Vec<NodeId> = remaining
            .iter()
            .copied()
            .filter(|node| out_degree(ir, *node, &remaining) == 0)
            .collect();
        if !sinks.is_empty() {
            for node in sinks {
                remaining.remove(&node);
                right.push(node);
            }
            continue;
        }

        let sources: Vec<NodeId> = remaining
            .iter()
            .copied()
            .filter(|node| in_degree(ir, *node, &remaining) == 0)
            .collect();
        if !sources.is_empty() {
            for node in sources {
                remaining.remove(&node);
                left.push(node);
            }
            continue;
        }

        let selected = remaining
            .iter()
            .copied()
            .max_by(|left_node, right_node| {
                let left_score = out_degree(ir, *left_node, &remaining) as isize
                    - in_degree(ir, *left_node, &remaining) as isize;
                let right_score = out_degree(ir, *right_node, &remaining) as isize
                    - in_degree(ir, *right_node, &remaining) as isize;
                let left_model = ir.nodes[ir.real_to_ir[left_node]].model_order;
                let right_model = ir.nodes[ir.real_to_ir[right_node]].model_order;
                left_score
                    .cmp(&right_score)
                    .then_with(|| right_model.cmp(&left_model))
                    .then_with(|| right_node.index().cmp(&left_node.index()))
            })
            .expect("remaining set should not be empty");
        remaining.remove(&selected);
        left.push(selected);
    }

    left.extend(right.into_iter().rev());
    let order: BTreeMap<NodeId, usize> = left
        .iter()
        .copied()
        .enumerate()
        .map(|(index, node)| (node, index))
        .collect();

    let mut reversed = 0usize;
    for edge in &mut ir.edges {
        if edge.self_loop {
            continue;
        }

        let source_rank = order
            .get(&edge.effective_source)
            .copied()
            .unwrap_or_default();
        let target_rank = order
            .get(&edge.effective_target)
            .copied()
            .unwrap_or_default();
        let prefers_forward = source_rank < target_rank
            || (source_rank == target_rank && edge.model_order <= edge.original_edge.index());
        edge.reversed = !prefers_forward;
        if edge.reversed {
            std::mem::swap(&mut edge.effective_source, &mut edge.effective_target);
            std::mem::swap(&mut edge.routed_source, &mut edge.routed_target);
            reversed += 1;
        }
    }

    reversed
}

fn in_degree(ir: &LayeredIr, node: NodeId, remaining: &BTreeSet<NodeId>) -> usize {
    ir.edges
        .iter()
        .filter(|edge| {
            !edge.self_loop
                && remaining.contains(&edge.effective_source)
                && remaining.contains(&edge.effective_target)
                && edge.effective_target == node
        })
        .count()
}

fn out_degree(ir: &LayeredIr, node: NodeId, remaining: &BTreeSet<NodeId>) -> usize {
    ir.edges
        .iter()
        .filter(|edge| {
            !edge.self_loop
                && remaining.contains(&edge.effective_source)
                && remaining.contains(&edge.effective_target)
                && edge.effective_source == node
        })
        .count()
}
