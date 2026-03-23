use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PortSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramPort {
    pub id: String,
    pub name: String,
    pub side: PortSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub width: f32,
    pub height: f32,
    pub parent_id: Option<String>,
    pub detail_lines: Vec<String>,
    pub ports: Vec<DiagramPort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramEdge {
    pub id: String,
    pub source_node: String,
    pub target_node: String,
    pub source_port: Option<String>,
    pub target_port: Option<String>,
    pub label: Option<String>,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagramGraph {
    pub nodes: Vec<DiagramNode>,
    pub edges: Vec<DiagramEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub node_gap_x: f32,
    pub node_gap_y: f32,
    pub container_padding: f32,
    pub container_header_height: f32,
    pub top_padding: f32,
    pub root_gap_x: f32,
    pub root_gap_y: f32,
    pub max_children_per_row: usize,
    pub root_layer_direction: LayerDirection,
    pub layer_direction: LayerDirection,
    pub view_profile: LayoutViewProfile,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            node_gap_x: 48.0,
            node_gap_y: 48.0,
            container_padding: 24.0,
            container_header_height: 44.0,
            top_padding: 28.0,
            root_gap_x: 72.0,
            root_gap_y: 72.0,
            max_children_per_row: 3,
            root_layer_direction: LayerDirection::VerticalColumns,
            layer_direction: LayerDirection::VerticalColumns,
            view_profile: LayoutViewProfile::Default,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LayerDirection {
    HorizontalRows,
    VerticalColumns,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LayoutViewProfile {
    #[default]
    Default,
    GeneralView,
    InterconnectionView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgRenderOptions {
    pub class_name: String,
    pub visual_edge_bridges: bool,
}

impl Default for SvgRenderOptions {
    fn default() -> Self {
        Self {
            class_name: "diagram-root".to_string(),
            visual_edge_bridges: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortLayout {
    pub id: String,
    pub name: String,
    pub node_id: String,
    pub side: PortSide,
    pub position: Point,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLayout {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub detail_lines: Vec<String>,
    pub bounds: Bounds,
    pub parent_id: Option<String>,
    pub ports: Vec<PortLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeLayout {
    pub id: String,
    pub source_node: String,
    pub target_node: String,
    pub kind: String,
    pub label: Option<String>,
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramLayout {
    pub width: f32,
    pub height: f32,
    pub nodes: Vec<NodeLayout>,
    pub edges: Vec<EdgeLayout>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedSvg {
    pub svg: String,
    pub hit_regions: Vec<HitRegion>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LayoutPhaseKind {
    Normalize,
    Measure,
    PlaceNodes,
    RouteEdges,
    Evaluate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPhaseReport {
    pub phase: LayoutPhaseKind,
    pub node_count: usize,
    pub edge_count: usize,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPipelineReport {
    pub phases: Vec<LayoutPhaseReport>,
    pub metrics: LayoutMetrics,
    pub warnings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum DiagramError {
    #[error("missing node `{0}`")]
    MissingNode(String),
    #[error("missing parent `{0}`")]
    MissingParent(String),
    #[error("missing port `{0}`")]
    MissingPort(String),
    #[error("layout failed: {0}")]
    LayoutFailure(String),
}

pub type Result<T> = std::result::Result<T, DiagramError>;
