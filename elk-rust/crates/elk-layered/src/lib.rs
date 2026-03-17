#![forbid(unsafe_code)]
#![doc = "Layered / Sugiyama-style layout engine for elk-rust."]

mod ir;
mod pipeline;

use std::collections::BTreeSet;

use elk_core::{Graph, LayoutEngine, LayoutError, LayoutOptions, LayoutReport, NodeId};
use pipeline::compound::{postprocess_cross_hierarchy_edges, preprocess_cross_hierarchy_edges};
use pipeline::layout_subgraph;

#[derive(Debug, Default, Clone, Copy)]
pub struct LayeredLayoutEngine;

impl LayeredLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl LayoutEngine for LayeredLayoutEngine {
    fn layout(
        &self,
        graph: &mut Graph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        validate_graph(graph)?;

        let mut effective_options = options.clone();
        effective_options.apply_view_profile_defaults();

        let mut report = LayoutReport::default();
        let top_level = graph.top_level_nodes();
        let top_level_set: BTreeSet<NodeId> = top_level.iter().copied().collect();
        let compound_map =
            preprocess_cross_hierarchy_edges(graph, &top_level_set, &effective_options);
        let bounds = layout_subgraph(graph, &top_level, &effective_options, &mut report)?;
        postprocess_cross_hierarchy_edges(graph, &compound_map);
        graph.bounds = bounds;
        Ok(report)
    }
}

fn validate_graph(graph: &Graph) -> Result<(), LayoutError> {
    for edge in &graph.edges {
        if edge.source.node.index() >= graph.nodes.len()
            || edge.target.node.index() >= graph.nodes.len()
        {
            return Err(LayoutError::Validation(format!(
                "edge {} references an unknown node",
                edge.id
            )));
        }
        if let Some(port_id) = edge.source.port {
            if port_id.index() >= graph.ports.len() {
                return Err(LayoutError::Validation(format!(
                    "edge {} references an unknown source port",
                    edge.id
                )));
            }
        }
        if let Some(port_id) = edge.target.port {
            if port_id.index() >= graph.ports.len() {
                return Err(LayoutError::Validation(format!(
                    "edge {} references an unknown target port",
                    edge.id
                )));
            }
        }
    }

    Ok(())
}
