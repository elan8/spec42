//! Diagram output types used by DiagramProvider. Core owns these so it does not depend on sysml-diagrams.
//! Binaries that use sysml-diagrams convert from sysml_diagrams::RenderedDiagram to these types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HitRegionKind {
    Node,
    Port,
    EdgeLabel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitRegion {
    pub id: String,
    pub kind: HitRegionKind,
    pub element_id: String,
    pub qualified_name: Option<String>,
    pub bounds: Bounds,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub overlap_count: usize,
    pub overlap_area: f32,
    pub edge_crossing_count: usize,
    pub edge_node_intrusion_count: usize,
    pub total_edge_length: f32,
    pub bend_count: usize,
    pub orthogonal_violation_count: usize,
    pub minimum_node_clearance: f32,
    pub canvas_area: f32,
    pub aspect_ratio: f32,
    pub compactness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub view: String,
    pub selection: Option<String>,
}

/// Rendered diagram output from a DiagramProvider. Serialized as RenderedDiagramDto in the LSP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedDiagram {
    pub svg: String,
    pub hit_map: Vec<HitRegion>,
    pub bounds: Bounds,
    pub metrics: LayoutMetrics,
    pub warnings: Vec<String>,
    pub view_state: ViewState,
}
