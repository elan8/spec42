//! DTOs and conversion helpers for sysml/model and related responses.

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::Range;

use crate::views::extracted_model as model;
use crate::views::ibd;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceAnchorDto {
    pub file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareComponentDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub crate_name: String,
    pub module_path: String,
    pub anchors: Vec<SourceAnchorDto>,
    pub is_external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareDependencyDto {
    pub from: String,
    pub to: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_anchor: Option<SourceAnchorDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareArchitectureModelDto {
    pub components: Vec<SoftwareComponentDto>,
    pub dependencies: Vec<SoftwareDependencyDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareAnalysisSummaryDto {
    pub crate_count: usize,
    pub module_count: usize,
    pub dependency_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareWorkspaceModelDto {
    pub workspace_root: String,
    pub architecture: SoftwareArchitectureModelDto,
    pub summary: SoftwareAnalysisSummaryDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysmlGraphDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifierDto {
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorParamsDto {
    pub text_document: TextDocumentIdentifierDto,
    pub position: PositionDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorElementRefDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub uri: String,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorResolutionDto {
    pub status: String,
    pub targets: Vec<SysmlFeatureInspectorElementRefDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorRelationshipDto {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub peer: SysmlFeatureInspectorElementRefDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorElementDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub uri: String,
    pub range: RangeDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<SysmlFeatureInspectorElementRefDto>,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
    pub typing: SysmlFeatureInspectorResolutionDto,
    pub specialization: SysmlFeatureInspectorResolutionDto,
    pub incoming_relationships: Vec<SysmlFeatureInspectorRelationshipDto>,
    pub outgoing_relationships: Vec<SysmlFeatureInspectorRelationshipDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorResultDto {
    pub version: u32,
    pub source_uri: String,
    pub requested_position: PositionDto,
    pub element: Option<SysmlFeatureInspectorElementDto>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationParamsDto {
    pub workspace_root_uri: String,
    pub view: String,
    #[serde(default)]
    pub selected_view: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareVisualizationParamsDto {
    pub workspace_root_uri: String,
    pub view: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareAnalyzeWorkspaceParamsDto {
    pub workspace_root_uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareProjectViewParamsDto {
    pub workspace_root_uri: String,
    pub view: String,
    pub workspace_model: SoftwareWorkspaceModelDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareVisualizationViewCandidateDto {
    pub id: String,
    pub name: String,
    pub supported: bool,
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
    pub software_architecture: Option<SoftwareArchitectureModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view_graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_model: Option<WorkspaceModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_diagrams: Option<Vec<model::ActivityDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_diagrams: Option<Vec<model::SequenceDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibd: Option<ibd::IbdDataDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<SysmlModelStatsDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareVisualizationResultDto {
    pub version: u32,
    pub view: String,
    pub workspace_root_uri: String,
    pub views: Vec<SoftwareVisualizationViewCandidateDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_state_message: Option<String>,
    pub graph: SysmlGraphDto,
    pub software_architecture: SoftwareArchitectureModelDto,
    pub workspace_model: WorkspaceModelDto,
    pub stats: SysmlModelStatsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareAnalyzeWorkspaceResultDto {
    pub version: u32,
    pub workspace_model: SoftwareWorkspaceModelDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlModelResultDto {
    pub version: u32,
    pub graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_architecture: Option<SoftwareArchitectureModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_groups: Option<Vec<SysmlVisualizationGroupDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view_graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_model: Option<WorkspaceModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_diagrams: Option<Vec<model::ActivityDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_diagrams: Option<Vec<model::SequenceDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibd: Option<ibd::IbdDataDto>,
    pub stats: Option<SysmlModelStatsDto>,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerStatsDto {
    pub uptime: u64,
    pub memory: SysmlServerMemoryDto,
    pub caches: SysmlServerCachesDto,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerMemoryDto {
    pub rss: u64,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerCachesDto {
    pub documents: usize,
    #[serde(rename = "symbolTables")]
    pub symbol_tables: usize,
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct SysmlClearCacheResultDto {
    pub documents: usize,
    #[serde(rename = "symbolTables")]
    pub symbol_tables: usize,
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchParamsDto {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchItemDto {
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    pub uri: String,
    pub range: RangeDto,
    pub score: i64,
    pub source: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchPackageDto {
    pub name: String,
    pub path: String,
    pub source: String,
    pub symbols: Vec<SysmlLibrarySearchItemDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchSourceDto {
    pub source: String,
    pub packages: Vec<SysmlLibrarySearchPackageDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchResultDto {
    pub sources: Vec<SysmlLibrarySearchSourceDto>,
    pub symbol_total: usize,
    pub total: usize,
}

pub fn range_to_dto(r: Range) -> RangeDto {
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
