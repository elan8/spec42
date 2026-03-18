use std::collections::{BTreeMap, BTreeSet, VecDeque};

use elk_core::{LayoutError, LayoutOptions};
use elk_graph::{EdgeEndpoint, EdgeId, ElkGraph, NodeId};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TreeModel {
    pub root: NodeId,
    /// parent -> children
    pub children: BTreeMap<NodeId, Vec<NodeId>>,
    /// child -> parent
    pub parent: BTreeMap<NodeId, NodeId>,
    /// edges used for parent->child relation (child -> edge)
    pub edge_to_child: BTreeMap<NodeId, EdgeId>,
    /// all edges that connect nodes in `nodes` (used for routing sanity)
    pub edges: Vec<EdgeId>,
    /// nodes participating (excluding synthetic root)
    pub nodes: Vec<NodeId>,
}

pub fn build_tree(graph: &ElkGraph, _options: &LayoutOptions) -> Result<TreeModel, LayoutError> {
    // Smoke-level: consider all real nodes (exclude synthetic root).
    let nodes = graph
        .nodes
        .iter()
        .filter(|n| n.id != graph.root)
        .map(|n| n.id)
        .collect::<Vec<_>>();

    if nodes.is_empty() {
        return Err(LayoutError::Validation(
            "tree layout: graph has no nodes".to_string(),
        ));
    }

    // Build directed adjacency from all edges (smoke-level: use first source/target endpoint).
    let mut indegree: BTreeMap<NodeId, usize> = nodes.iter().copied().map(|n| (n, 0)).collect();
    let mut out: BTreeMap<NodeId, Vec<(NodeId, EdgeId)>> = BTreeMap::new();
    let mut edges_in_tree = Vec::new();

    for edge in &graph.edges {
        let Some(src) = edge.sources.first().copied() else { continue };
        let Some(tgt) = edge.targets.first().copied() else { continue };
        if !indegree.contains_key(&src.node) || !indegree.contains_key(&tgt.node) {
            continue;
        }
        out.entry(src.node).or_default().push((tgt.node, edge.id));
        *indegree.entry(tgt.node).or_insert(0) += 1;
        edges_in_tree.push(edge.id);
    }

    // Root selection: prefer explicit zero-indegree node; tie-break by node id.
    let mut roots: Vec<NodeId> = indegree
        .iter()
        .filter_map(|(n, d)| (*d == 0).then_some(*n))
        .collect();
    roots.sort();
    let root = roots.first().copied().unwrap_or_else(|| nodes[0]);

    // Spanning tree BFS (deterministic): visit children in sorted order.
    let mut parent: BTreeMap<NodeId, NodeId> = BTreeMap::new();
    let mut children: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    let mut edge_to_child: BTreeMap<NodeId, EdgeId> = BTreeMap::new();
    let mut seen: BTreeSet<NodeId> = BTreeSet::new();
    let mut q = VecDeque::new();
    seen.insert(root);
    q.push_back(root);

    while let Some(cur) = q.pop_front() {
        let mut nexts = out.get(&cur).cloned().unwrap_or_default();
        nexts.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        for (child, edge_id) in nexts {
            if !seen.insert(child) {
                continue;
            }
            parent.insert(child, cur);
            children.entry(cur).or_default().push(child);
            edge_to_child.insert(child, edge_id);
            q.push_back(child);
        }
    }

    // Ensure all nodes are present in children map for iteration.
    for n in &nodes {
        children.entry(*n).or_default();
    }

    // If some nodes were unreachable, attach them as extra roots under chosen root (smoke-level).
    for n in &nodes {
        if *n == root {
            continue;
        }
        if !seen.contains(n) {
            parent.insert(*n, root);
            children.entry(root).or_default().push(*n);
        }
    }

    Ok(TreeModel {
        root,
        children,
        parent,
        edge_to_child,
        edges: edges_in_tree,
        nodes,
    })
}

#[allow(dead_code)]
fn endpoint_node(ep: EdgeEndpoint) -> NodeId {
    ep.node
}

