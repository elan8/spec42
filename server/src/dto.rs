//! Re-export DTOs from spec42-core and convert sysml_diagrams output to core types for LSP response.

pub use spec42_core::dto::*;

use spec42_core::diagram_types::{Bounds, HitRegion, HitRegionKind, LayoutMetrics, RenderedDiagram, ViewState};

/// Converts sysml_diagrams::RenderedDiagram to spec42_core::RenderedDiagram and then to RenderedDiagramDto.
pub fn rendered_diagram_to_dto(diagram: sysml_diagrams::RenderedDiagram) -> RenderedDiagramDto {
    let core_diagram = RenderedDiagram {
        svg: diagram.svg,
        hit_map: diagram
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
            x: diagram.bounds.x,
            y: diagram.bounds.y,
            width: diagram.bounds.width,
            height: diagram.bounds.height,
        },
        metrics: LayoutMetrics {
            node_count: diagram.metrics.node_count,
            edge_count: diagram.metrics.edge_count,
            overlap_count: diagram.metrics.overlap_count,
            overlap_area: diagram.metrics.overlap_area,
            edge_crossing_count: diagram.metrics.edge_crossing_count,
            edge_node_intrusion_count: diagram.metrics.edge_node_intrusion_count,
            total_edge_length: diagram.metrics.total_edge_length,
            bend_count: diagram.metrics.bend_count,
            orthogonal_violation_count: diagram.metrics.orthogonal_violation_count,
            minimum_node_clearance: diagram.metrics.minimum_node_clearance,
            canvas_area: diagram.metrics.canvas_area,
            aspect_ratio: diagram.metrics.aspect_ratio,
            compactness: diagram.metrics.compactness,
        },
        warnings: diagram.warnings,
        view_state: ViewState {
            view: diagram.view_state.view,
            selection: diagram.view_state.selection,
        },
    };
    spec42_core::dto::rendered_diagram_to_dto(core_diagram)
}
