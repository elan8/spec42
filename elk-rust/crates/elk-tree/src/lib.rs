#![forbid(unsafe_code)]
#![doc = "Tree layout engine (ELK alg.mrtree subset)."]

mod build_tree;
mod layout;
mod routing;

use elk_core::{LayoutError, LayoutOptions, LayoutReport};
use elk_graph::ElkGraph;

#[derive(Debug, Default, Clone, Copy)]
pub struct TreeLayoutEngine;

impl TreeLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        // Smoke-level: validate ids before layout.
        graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;

        let mut effective = options.clone();
        effective.apply_view_profile_defaults();

        let tree = build_tree::build_tree(graph, &effective)?;
        let mut report = LayoutReport::default();
        let bounds = layout::place_tree(graph, &tree, &effective, &mut report)?;
        routing::route_tree_edges(graph, &tree, &effective, &mut report)?;

        // Store bounds on synthetic root node geometry.
        if let Some(root) = graph.nodes.get_mut(graph.root.index()) {
            root.geometry.x = bounds.origin.x;
            root.geometry.y = bounds.origin.y;
            root.geometry.width = bounds.size.width;
            root.geometry.height = bounds.size.height;
        }
        Ok(report)
    }
}

pub fn layout(graph: &mut ElkGraph, options: &LayoutOptions) -> Result<LayoutReport, LayoutError> {
    TreeLayoutEngine::new().layout(graph, options)
}

