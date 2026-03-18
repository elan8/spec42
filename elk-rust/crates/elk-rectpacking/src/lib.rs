#![forbid(unsafe_code)]
#![doc = "Rectangle packing layout (ELK alg.rectpacking baseline)."]

use std::collections::BTreeMap;

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, Rect, Size};
use elk_graph::{ElkGraph, NodeId};

#[derive(Debug, Default, Clone, Copy)]
pub struct RectPackingLayoutEngine;

impl RectPackingLayoutEngine {
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

        let nodes = top_level_nodes(graph);
        let bounds = pack_nodes(graph, &nodes, &effective)?;

        if let Some(root) = graph.nodes.get_mut(graph.root.index()) {
            root.geometry.x = bounds.origin.x;
            root.geometry.y = bounds.origin.y;
            root.geometry.width = bounds.size.width;
            root.geometry.height = bounds.size.height;
        }

        Ok(LayoutReport::default())
    }
}

pub fn layout(graph: &mut ElkGraph, options: &LayoutOptions) -> Result<LayoutReport, LayoutError> {
    RectPackingLayoutEngine::new().layout(graph, options)
}

fn top_level_nodes(graph: &ElkGraph) -> Vec<NodeId> {
    let mut out = graph
        .nodes
        .iter()
        .filter(|n| n.parent == Some(graph.root) && n.id != graph.root)
        .map(|n| n.id)
        .collect::<Vec<_>>();
    out.sort();
    out
}

fn node_size(graph: &ElkGraph, n: NodeId) -> Size {
    let g = graph.nodes[n.index()].geometry;
    Size::new(g.width.max(1.0), g.height.max(1.0))
}

/// Deterministic shelf/row packing.
fn pack_nodes(graph: &mut ElkGraph, nodes: &[NodeId], options: &LayoutOptions) -> Result<Rect, LayoutError> {
    let spacing = options.layered.spacing.component_spacing.max(0.0);
    let padding = options.layered.padding;
    let pad = padding.top.max(padding.right).max(padding.bottom).max(padding.left);

    if nodes.is_empty() {
        return Ok(Rect::new(Point::default(), Size::new(pad * 2.0, pad * 2.0)));
    }

    // Sort by (height desc, width desc, id) for better packing while deterministic.
    let mut ordered = nodes.to_vec();
    ordered.sort_by(|a, b| {
        let sa = node_size(graph, *a);
        let sb = node_size(graph, *b);
        sb.height
            .partial_cmp(&sa.height)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| sb.width.partial_cmp(&sa.width).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| a.cmp(b))
    });

    // Target row width heuristic: sqrt(total_area) * aspect_ratio(=1.2).
    let total_area: f32 = ordered
        .iter()
        .map(|n| {
            let s = node_size(graph, *n);
            s.width * s.height
        })
        .sum();
    let target_row_width = total_area.max(1.0).sqrt() * 1.2;

    let mut cursor_x = pad;
    let mut cursor_y = pad;
    let mut row_height = 0.0;

    let mut min = Point::new(f32::MAX, f32::MAX);
    let mut max = Point::new(f32::MIN, f32::MIN);

    // For stability, keep per-node placed positions (useful if we add a second pass later).
    let mut placed: BTreeMap<NodeId, Point> = BTreeMap::new();

    for n in ordered {
        let s = node_size(graph, n);
        if cursor_x > pad && cursor_x + s.width > pad + target_row_width {
            cursor_x = pad;
            cursor_y += row_height + spacing;
            row_height = 0.0;
        }

        placed.insert(n, Point::new(cursor_x, cursor_y));
        cursor_x += s.width + spacing;
        row_height = row_height.max(s.height);
    }

    for (n, p) in placed {
        let s = node_size(graph, n);
        let geom = &mut graph.nodes[n.index()].geometry;
        geom.x = p.x;
        geom.y = p.y;

        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x + s.width);
        max.y = max.y.max(p.y + s.height);
    }

    min.x -= pad;
    min.y -= pad;
    max.x += pad;
    max.y += pad;
    Ok(Rect::new(min, Size::new(max.x - min.x, max.y - min.y)))
}

