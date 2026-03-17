use crate::layout::{Bounds, DiagramError, HitRegion, LayoutMetrics};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeInput {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNodeInput {
    pub id: String,
    pub element_type: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub range: RangeInput,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdgeInput {
    pub source: String,
    pub target: String,
    pub rel_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbdPartInput {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    pub container_id: Option<String>,
    pub element_type: String,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbdPortInput {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub direction: Option<String>,
    pub port_type: Option<String>,
    pub port_side: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbdConnectorInput {
    pub source: String,
    pub target: String,
    pub source_id: String,
    pub target_id: String,
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbdInput {
    pub parts: Vec<IbdPartInput>,
    pub ports: Vec<IbdPortInput>,
    pub connectors: Vec<IbdConnectorInput>,
    pub root_candidates: Vec<String>,
    pub default_root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub view: String,
    pub selection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedDiagram {
    pub svg: String,
    pub hit_map: Vec<HitRegion>,
    pub bounds: Bounds,
    pub metrics: LayoutMetrics,
    pub warnings: Vec<String>,
    pub view_state: ViewState,
}

#[derive(Debug, Error)]
pub enum DiagramBuildError {
    #[error(transparent)]
    Layout(#[from] DiagramError),
}

pub type Result<T> = std::result::Result<T, DiagramBuildError>;
