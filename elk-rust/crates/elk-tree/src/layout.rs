use std::collections::BTreeMap;

use elk_core::{LayoutDirection, LayoutError, LayoutOptions, LayoutReport, Point, Rect, Size};
use elk_graph::{ElkGraph, NodeId};

use crate::build_tree::TreeModel;

pub fn place_tree(
    graph: &mut ElkGraph,
    tree: &TreeModel,
    options: &LayoutOptions,
    _report: &mut LayoutReport,
) -> Result<Rect, LayoutError> {
    let direction = options.layered.direction;
    let spacing_sibling = options.layered.spacing.node_spacing;
    let spacing_depth = options.layered.spacing.layer_spacing;
    let padding = options.layered.padding;

    // Depth-first assign coordinates.
    let mut cursor_minor: BTreeMap<usize, f32> = BTreeMap::new();
    let mut depth_of: BTreeMap<NodeId, usize> = BTreeMap::new();
    depth_of.insert(tree.root, 0);

    fn major_is_x(dir: LayoutDirection) -> bool {
        matches!(dir, LayoutDirection::LeftToRight | LayoutDirection::RightToLeft)
    }

    fn node_size(graph: &ElkGraph, n: NodeId) -> Size {
        let g = graph.nodes[n.index()].geometry;
        Size::new(g.width, g.height)
    }

    fn dfs(
        graph: &mut ElkGraph,
        tree: &TreeModel,
        node: NodeId,
        depth: usize,
        direction: LayoutDirection,
        spacing_sibling: f32,
        spacing_depth: f32,
        cursor_minor: &mut BTreeMap<usize, f32>,
        depth_of: &mut BTreeMap<NodeId, usize>,
    ) {
        depth_of.insert(node, depth);
        let mut children = tree.children.get(&node).cloned().unwrap_or_default();
        children.sort();
        for c in children {
            dfs(
                graph,
                tree,
                c,
                depth + 1,
                direction,
                spacing_sibling,
                spacing_depth,
                cursor_minor,
                depth_of,
            );
        }

        let size = node_size(graph, node);
        let cur = cursor_minor.entry(depth).or_insert(0.0);
        // Major coordinate is based on depth.
        let major = depth as f32 * (spacing_depth + if major_is_x(direction) { size.width } else { size.height });
        let minor = *cur;
        *cur += (if major_is_x(direction) { size.height } else { size.width }) + spacing_sibling;

        let geom = &mut graph.nodes[node.index()].geometry;
        match direction {
            LayoutDirection::TopToBottom => {
                geom.x = minor;
                geom.y = major;
            }
            LayoutDirection::BottomToTop => {
                geom.x = minor;
                geom.y = -major;
            }
            LayoutDirection::LeftToRight => {
                geom.x = major;
                geom.y = minor;
            }
            LayoutDirection::RightToLeft => {
                geom.x = -major;
                geom.y = minor;
            }
        }
    }

    dfs(
        graph,
        tree,
        tree.root,
        0,
        direction,
        spacing_sibling,
        spacing_depth,
        &mut cursor_minor,
        &mut depth_of,
    );

    // Compute bounds over tree nodes.
    let mut min = Point::new(f32::MAX, f32::MAX);
    let mut max = Point::new(f32::MIN, f32::MIN);
    for n in &tree.nodes {
        let g = graph.nodes[n.index()].geometry;
        min.x = min.x.min(g.x);
        min.y = min.y.min(g.y);
        max.x = max.x.max(g.x + g.width);
        max.y = max.y.max(g.y + g.height);
    }

    // Apply padding.
    min.x -= padding.left;
    min.y -= padding.top;
    max.x += padding.right;
    max.y += padding.bottom;
    Ok(Rect::new(min, Size::new(max.x - min.x, max.y - min.y)))
}

