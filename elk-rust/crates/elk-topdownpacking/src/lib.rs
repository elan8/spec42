#![forbid(unsafe_code)]
#![doc = "Top-down hierarchy-aware packing (ELK alg.topdownpacking baseline)."]

use std::collections::{BTreeMap, VecDeque};

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, Size};
use elk_graph::{ElkGraph, NodeId};

#[derive(Debug, Default, Clone, Copy)]
pub struct TopDownPackingLayoutEngine;

impl TopDownPackingLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;
        let mut effective = options.clone();
        effective.apply_view_profile_defaults();

        let spacing = effective.layered.spacing.component_spacing.max(0.0);
        let pad = effective.layered.padding.top
            .max(effective.layered.padding.right)
            .max(effective.layered.padding.bottom)
            .max(effective.layered.padding.left);

        // Post-order: children before parents.
        let post_order = post_order_nodes(graph, graph.root);

        // Pass 1: size compounds and assign relative positions to children.
        for &node_id in &post_order {
            let children = graph.children_of(node_id).to_vec();
            if children.is_empty() {
                continue;
            }
            let sizes: Vec<(NodeId, Size)> = children
                .iter()
                .map(|&n| (n, node_size(graph, n)))
                .collect();
            let placements = pack_rects(&sizes, spacing, pad);
            let mut min_x = 0.0f32;
            let mut min_y = 0.0f32;
            let mut max_x = 0.0f32;
            let mut max_y = 0.0f32;
            for (n, pt) in &placements {
                let s = node_size(graph, *n);
                graph.nodes[n.index()].geometry.x = pt.x;
                graph.nodes[n.index()].geometry.y = pt.y;
                min_x = min_x.min(pt.x);
                min_y = min_y.min(pt.y);
                max_x = max_x.max(pt.x + s.width);
                max_y = max_y.max(pt.y + s.height);
            }
            let w = (max_x - min_x + 2.0 * pad).max(1.0);
            let h = (max_y - min_y + 2.0 * pad).max(1.0);
            graph.nodes[node_id.index()].geometry.width = w;
            graph.nodes[node_id.index()].geometry.height = h;
            // Shift children so container origin is at (0,0) for this compound.
            for (n, pt) in &placements {
                graph.nodes[n.index()].geometry.x = pt.x - min_x + pad;
                graph.nodes[n.index()].geometry.y = pt.y - min_y + pad;
            }
        }

        // Pass 2: absolute positions from root down.
        graph.nodes[graph.root.index()].geometry.x = 0.0;
        graph.nodes[graph.root.index()].geometry.y = 0.0;
        let mut queue = VecDeque::new();
        queue.push_back(graph.root);
        while let Some(p) = queue.pop_front() {
            let px = graph.nodes[p.index()].geometry.x;
            let py = graph.nodes[p.index()].geometry.y;
            let children: Vec<NodeId> = graph.children_of(p).to_vec();
            for c in children {
                graph.nodes[c.index()].geometry.x += px;
                graph.nodes[c.index()].geometry.y += py;
                queue.push_back(c);
            }
        }

        // Root bounds = bounding box of top-level nodes + padding; translate whole graph so root is at (0,0).
        let top = graph.children_of(graph.root).to_vec();
        if top.is_empty() {
            graph.nodes[graph.root.index()].geometry.x = 0.0;
            graph.nodes[graph.root.index()].geometry.y = 0.0;
            graph.nodes[graph.root.index()].geometry.width = 2.0 * pad;
            graph.nodes[graph.root.index()].geometry.height = 2.0 * pad;
        } else {
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;
            for &n in &top {
                let g = &graph.nodes[n.index()].geometry;
                min_x = min_x.min(g.x);
                min_y = min_y.min(g.y);
                max_x = max_x.max(g.x + g.width);
                max_y = max_y.max(g.y + g.height);
            }
            let dx = pad - min_x;
            let dy = pad - min_y;
            for node in &mut graph.nodes {
                if node.id != graph.root {
                    node.geometry.x += dx;
                    node.geometry.y += dy;
                }
            }
            graph.nodes[graph.root.index()].geometry.x = 0.0;
            graph.nodes[graph.root.index()].geometry.y = 0.0;
            graph.nodes[graph.root.index()].geometry.width = max_x - min_x + 2.0 * pad;
            graph.nodes[graph.root.index()].geometry.height = max_y - min_y + 2.0 * pad;
        }

        Ok(LayoutReport::default())
    }
}

pub fn layout(graph: &mut ElkGraph, options: &LayoutOptions) -> Result<LayoutReport, LayoutError> {
    TopDownPackingLayoutEngine::new().layout(graph, options)
}

fn post_order_nodes(graph: &ElkGraph, root: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    fn visit(graph: &ElkGraph, n: NodeId, out: &mut Vec<NodeId>) {
        for &c in graph.children_of(n) {
            visit(graph, c, out);
        }
        out.push(n);
    }
    visit(graph, root, &mut out);
    out
}

fn node_size(graph: &ElkGraph, n: NodeId) -> Size {
    let g = graph.nodes[n.index()].geometry;
    Size::new(g.width.max(1.0), g.height.max(1.0))
}

/// Deterministic shelf packing; returns (x,y) per node.
fn pack_rects(sizes: &[(NodeId, Size)], spacing: f32, pad: f32) -> BTreeMap<NodeId, Point> {
    let mut out = BTreeMap::new();
    if sizes.is_empty() {
        return out;
    }
    let total_area: f32 = sizes.iter().map(|(_, s)| s.width * s.height).sum();
    let target_row_width = total_area.max(1.0).sqrt() * 1.2;

    let mut ordered: Vec<(NodeId, Size)> = sizes.to_vec();
    ordered.sort_by(|a, b| {
        b.1.height
            .partial_cmp(&a.1.height)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.1.width.partial_cmp(&a.1.width).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut cursor_x = pad;
    let mut cursor_y = pad;
    let mut row_height = 0.0;

    for (n, s) in ordered {
        if cursor_x > pad && cursor_x + s.width > pad + target_row_width {
            cursor_x = pad;
            cursor_y += row_height + spacing;
            row_height = 0.0;
        }
        out.insert(n, Point::new(cursor_x, cursor_y));
        cursor_x += s.width + spacing;
        row_height = row_height.max(s.height);
    }
    out
}
