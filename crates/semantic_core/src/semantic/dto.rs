//! Shared DTO primitives for semantic/model and visualization payloads.

use crate::semantic::text_span::TextRange;
use serde::{Deserialize, Serialize};

use crate::semantic::extracted_model::{ActivityDiagramDto, SequenceDiagramDto, StateMachineDto};
use crate::semantic::ibd::IbdDataDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct RelationshipDto {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub source: String,
    pub target: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphNodeDto {
    pub id: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<String>,
    pub range: RangeDto,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphEdgeDto {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysmlGraphDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysmlElementDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
    pub children: Vec<SysmlElementDto>,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
    pub relationships: Vec<RelationshipDto>,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFileModelDto {
    pub uri: String,
    pub elements: Vec<SysmlElementDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceModelSummaryDto {
    pub scanned_files: usize,
    pub loaded_files: usize,
    pub failures: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceModelDto {
    pub files: Vec<WorkspaceFileModelDto>,
    pub semantic: Vec<SysmlElementDto>,
    pub summary: WorkspaceModelSummaryDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysmlModelStatsDto {
    #[serde(rename = "totalElements")]
    pub total_elements: u32,
    #[serde(rename = "resolvedElements")]
    pub resolved_elements: u32,
    #[serde(rename = "unresolvedElements")]
    pub unresolved_elements: u32,
    #[serde(rename = "parseTimeMs")]
    pub parse_time_ms: u32,
    #[serde(rename = "modelBuildTimeMs")]
    pub model_build_time_ms: u32,
    #[serde(rename = "parseCached")]
    pub parse_cached: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationViewCandidateDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer_view: Option<String>,
    pub supported: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationPackageCandidateDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationGroupDto {
    pub id: String,
    pub label: String,
    pub depth: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationResultDto {
    pub version: u32,
    pub view: String,
    pub workspace_root_uri: String,
    /// When false, clients must not render diagram geometry (model still indexing or refreshing).
    #[serde(default = "default_model_ready")]
    pub model_ready: bool,
    pub view_candidates: Vec<SysmlVisualizationViewCandidateDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_state_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_groups: Option<Vec<SysmlVisualizationGroupDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view_graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_model: Option<WorkspaceModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_diagrams: Option<Vec<ActivityDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_diagrams: Option<Vec<SequenceDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machines: Option<Vec<StateMachineDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibd: Option<IbdDataDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<SysmlModelStatsDto>,
}

#[allow(dead_code)]
fn default_model_ready() -> bool {
    true
}

pub fn visualization_model_not_ready(
    workspace_root_uri: &str,
    view: &str,
    message: &str,
) -> SysmlVisualizationResultDto {
    let empty_graph = SysmlGraphDto {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    SysmlVisualizationResultDto {
        version: 0,
        model_ready: false,
        view: view.to_string(),
        workspace_root_uri: workspace_root_uri.to_string(),
        view_candidates: Vec::new(),
        selected_view: None,
        selected_view_name: None,
        empty_state_message: Some(message.to_string()),
        package_groups: Some(Vec::new()),
        graph: Some(empty_graph.clone()),
        general_view_graph: Some(empty_graph),
        workspace_model: None,
        activity_diagrams: None,
        sequence_diagrams: None,
        state_machines: None,
        ibd: None,
        stats: None,
    }
}

pub fn range_to_dto(r: TextRange) -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: r.start.line,
            character: r.start.character,
        },
        end: PositionDto {
            line: r.end.line,
            character: r.end.character,
        },
    }
}
