#![forbid(unsafe_code)]
#![doc = "Layered / Sugiyama-style layout engine for elk-rust."]

mod ir;
mod pipeline;

use std::collections::BTreeSet;

use elk_core::{LayoutError, LayoutOptions, LayoutReport};
use elk_graph::{ElkGraph, NodeId};
use pipeline::compound::{postprocess_cross_hierarchy_edges, preprocess_cross_hierarchy_edges};
use pipeline::layout_subgraph;
use pipeline::decode_layout_from_props;

#[derive(Debug, Default, Clone, Copy)]
pub struct LayeredLayoutEngine;

impl LayeredLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        validate_graph(graph)?;

        let mut effective_options = options.clone();
        effective_options.apply_view_profile_defaults();
        // Allow graph-level overrides coming from interchange formats (e.g. ELK Graph JSON's
        // `layoutOptions`). This mirrors ELK's behavior where the root node carries global defaults.
        let root_overrides = decode_layout_from_props(&graph.properties);
        if let Some(direction) = root_overrides.direction {
            effective_options.layered.direction = direction;
        }
        if let Some(edge_routing) = root_overrides.edge_routing {
            effective_options.layered.edge_routing = edge_routing;
        }
        if let Some(respect_port_order) = root_overrides.respect_port_order {
            effective_options.layered.respect_port_order = respect_port_order;
        }
        if let Some(node_alignment) = root_overrides.node_alignment {
            effective_options.layered.node_alignment = node_alignment;
        }
        if let Some(padding) = root_overrides.padding {
            effective_options.layered.padding = padding;
        }
        if let Some(spacing) = root_overrides.spacing {
            effective_options.layered.spacing = spacing;
        }

        let mut report = LayoutReport::default();
        let top_level = elk_graph_top_level_nodes(graph);
        let top_level_set: BTreeSet<NodeId> = top_level.iter().copied().collect();
        let compound_map =
            preprocess_cross_hierarchy_edges(graph, &top_level_set, &effective_options);
        let bounds = layout_subgraph(graph, &top_level, &effective_options, &mut report)?;
        postprocess_cross_hierarchy_edges(graph, &compound_map);
        // Store graph bounds on the synthetic root node geometry.
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
    LayeredLayoutEngine::new().layout(graph, options)
}

fn elk_graph_top_level_nodes(graph: &ElkGraph) -> Vec<NodeId> {
    graph
        .nodes
        .iter()
        // In `ElkGraph`, real top-level nodes are children of the synthetic root.
        .filter(|n| n.parent == Some(graph.root) && n.id != graph.root)
        .map(|n| n.id)
        .collect()
}

fn validate_graph(graph: &ElkGraph) -> Result<(), LayoutError> {
    for edge in &graph.edges {
        for endpoint in edge.sources.iter().chain(edge.targets.iter()) {
            if endpoint.node.index() >= graph.nodes.len() {
                return Err(LayoutError::Validation(format!(
                    "edge {:?} references an unknown node",
                    edge.id
                )));
            }
            if let Some(port_id) = endpoint.port {
                if port_id.index() >= graph.ports.len() {
                    return Err(LayoutError::Validation(format!(
                        "edge {:?} references an unknown port",
                        edge.id
                    )));
                }
            }
        }
    }
    Ok(())
}
