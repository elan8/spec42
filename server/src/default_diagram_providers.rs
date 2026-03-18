//! Default diagram providers that use sysml-diagrams for general view and interconnection view.

use std::sync::Arc;

use spec42_core::config::{DiagramContext, DiagramProvider};
use spec42_core::dto::{GraphEdgeDto, GraphNodeDto};
use spec42_core::diagram_types::{Bounds, HitRegion, HitRegionKind, LayoutMetrics, RenderedDiagram, ViewState};
use spec42_core::ibd::IbdDataDto;

fn sysml_to_core_rendered(d: sysml_diagrams::RenderedDiagram) -> RenderedDiagram {
    RenderedDiagram {
        svg: d.svg,
        hit_map: d
            .hit_map
            .into_iter()
            .map(|hit| HitRegion {
                id: hit.id,
                kind: match hit.kind {
                    sysml_diagrams::HitRegionKind::Node => HitRegionKind::Node,
                    sysml_diagrams::HitRegionKind::Port => HitRegionKind::Port,
                    sysml_diagrams::HitRegionKind::EdgeLabel => HitRegionKind::EdgeLabel,
                },
                element_id: hit.element_id,
                qualified_name: hit.qualified_name,
                bounds: Bounds {
                    x: hit.bounds.x,
                    y: hit.bounds.y,
                    width: hit.bounds.width,
                    height: hit.bounds.height,
                },
            })
            .collect(),
        bounds: Bounds {
            x: d.bounds.x,
            y: d.bounds.y,
            width: d.bounds.width,
            height: d.bounds.height,
        },
        metrics: LayoutMetrics {
            node_count: d.metrics.node_count,
            edge_count: d.metrics.edge_count,
            overlap_count: d.metrics.overlap_count,
            overlap_area: d.metrics.overlap_area,
            edge_crossing_count: d.metrics.edge_crossing_count,
            edge_node_intrusion_count: d.metrics.edge_node_intrusion_count,
            total_edge_length: d.metrics.total_edge_length,
            bend_count: d.metrics.bend_count,
            orthogonal_violation_count: d.metrics.orthogonal_violation_count,
            minimum_node_clearance: d.metrics.minimum_node_clearance,
            canvas_area: d.metrics.canvas_area,
            aspect_ratio: d.metrics.aspect_ratio,
            compactness: d.metrics.compactness,
        },
        warnings: d.warnings,
        view_state: ViewState {
            view: d.view_state.view,
            selection: d.view_state.selection,
        },
    }
}

fn graph_node_input(node: &GraphNodeDto) -> sysml_diagrams::GraphNodeInput {
    sysml_diagrams::GraphNodeInput {
        id: node.id.clone(),
        element_type: node.element_type.clone(),
        name: node.name.clone(),
        parent_id: node.parent_id.clone(),
        range: sysml_diagrams::RangeInput {
            start_line: node.range.start.line,
            start_character: node.range.start.character,
            end_line: node.range.end.line,
            end_character: node.range.end.character,
        },
        attributes: node
            .attributes
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    serde_json::to_string(value).unwrap_or_else(|_| String::new()),
                )
            })
            .collect(),
    }
}

fn graph_edge_input(edge: &GraphEdgeDto) -> sysml_diagrams::GraphEdgeInput {
    sysml_diagrams::GraphEdgeInput {
        source: edge.source.clone(),
        target: edge.target.clone(),
        rel_type: edge.rel_type.clone(),
        name: edge.name.clone(),
    }
}

fn ibd_input(ibd: &IbdDataDto) -> sysml_diagrams::IbdInput {
    sysml_diagrams::IbdInput {
        parts: ibd
            .parts
            .iter()
            .map(|part| sysml_diagrams::IbdPartInput {
                id: part.id.clone(),
                name: part.name.clone(),
                qualified_name: part.qualified_name.clone(),
                container_id: part.container_id.clone(),
                element_type: part.element_type.clone(),
                attributes: part
                    .attributes
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            serde_json::to_string(value).unwrap_or_else(|_| String::new()),
                        )
                    })
                    .collect(),
            })
            .collect(),
        ports: ibd
            .ports
            .iter()
            .map(|port| sysml_diagrams::IbdPortInput {
                id: port.id.clone(),
                name: port.name.clone(),
                parent_id: port.parent_id.clone(),
                direction: port.direction.clone(),
                port_type: port.port_type.clone(),
                port_side: port.port_side.clone(),
            })
            .collect(),
        connectors: ibd
            .connectors
            .iter()
            .map(|connector| sysml_diagrams::IbdConnectorInput {
                source: connector.source.clone(),
                target: connector.target.clone(),
                source_id: connector.source_id.clone(),
                target_id: connector.target_id.clone(),
                rel_type: connector.rel_type.clone(),
            })
            .collect(),
        root_candidates: ibd.root_candidates.clone(),
        default_root: ibd.default_root.clone(),
    }
}

/// General view diagram provider (graph of nodes and edges).
#[derive(Debug, Default)]
pub struct GeneralViewProvider;

impl DiagramProvider for GeneralViewProvider {
    fn diagram_id(&self) -> &str {
        "generalView"
    }

    fn render(&self, context: &DiagramContext<'_>) -> Option<RenderedDiagram> {
        let graph = context.graph?;
        let nodes: Vec<sysml_diagrams::GraphNodeInput> = graph
            .nodes
            .iter()
            .map(graph_node_input)
            .collect();
        let edges: Vec<sysml_diagrams::GraphEdgeInput> = graph
            .edges
            .iter()
            .map(graph_edge_input)
            .collect();
        sysml_diagrams::general_view::render(&nodes, &edges)
            .ok()
            .map(sysml_to_core_rendered)
    }
}

/// Interconnection view (IBD) diagram provider.
#[derive(Debug, Default)]
pub struct InterconnectionViewProvider;

impl DiagramProvider for InterconnectionViewProvider {
    fn diagram_id(&self) -> &str {
        "interconnectionView"
    }

    fn render(&self, context: &DiagramContext<'_>) -> Option<RenderedDiagram> {
        let ibd = context.ibd?;
        sysml_diagrams::interconnection_view::render(&ibd_input(ibd))
            .ok()
            .map(sysml_to_core_rendered)
    }
}

/// Builds the default Spec42 config: default semantic checks and default diagram providers.
pub fn default_config() -> spec42_core::Spec42Config {
    spec42_core::Spec42Config::new()
        .with_check_provider(Arc::new(spec42_core::DefaultSemanticChecks))
        .with_diagram_provider(Arc::new(GeneralViewProvider))
        .with_diagram_provider(Arc::new(InterconnectionViewProvider))
}
