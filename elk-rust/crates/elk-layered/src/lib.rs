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
        // Allow graph-level overrides coming from interchange formats (e.g. ELK Graph JSON's
        // `layoutOptions`). This mirrors ELK's behavior where the root node carries global defaults.
        if let Some(direction) = graph.layout.direction {
            effective_options.layered.direction = direction;
        }
        if let Some(edge_routing) = graph.layout.edge_routing {
            effective_options.layered.edge_routing = edge_routing;
        }
        if let Some(hierarchy_handling) = graph.layout.hierarchy_handling {
            effective_options.layered.hierarchy_handling = hierarchy_handling;
        }
        if let Some(respect_port_order) = graph.layout.respect_port_order {
            effective_options.layered.respect_port_order = respect_port_order;
        }
        if let Some(node_alignment) = graph.layout.node_alignment {
            effective_options.layered.node_alignment = node_alignment;
        }
        if let Some(padding) = graph.layout.padding {
            effective_options.layered.padding = padding;
        }
        if let Some(spacing) = graph.layout.spacing {
            effective_options.layered.spacing = spacing;
        }

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

/// Compatibility entry point: run layered layout on an `elk-graph` model.
///
/// This uses the `elk-graph -> elk-core` bridge so we can migrate incrementally without rewriting
/// the full pipeline in one step.
pub fn layout_elk_graph(
    graph: &mut elk_graph::ElkGraph,
    options: &LayoutOptions,
) -> Result<LayoutReport, LayoutError> {
    let mut bridge = elk_graph::to_core_graph(graph);
    let report = LayeredLayoutEngine::new().layout(&mut bridge.core, options)?;
    bridge.apply_core_layout_to_elk_graph(graph);
    Ok(report)
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
