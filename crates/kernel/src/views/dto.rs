//! DTOs and conversion helpers for sysml/model and related responses.

use serde::{Deserialize, Serialize};

use crate::views::extracted_model as model;
use crate::views::ibd;
pub use semantic_core::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, RelationshipDto,
    SysmlElementDto, SysmlGraphDto, SysmlModelStatsDto, SysmlVisualizationGroupDto,
    SysmlVisualizationPackageCandidateDto, SysmlVisualizationResultDto,
    SysmlVisualizationViewCandidateDto, WorkspaceFileModelDto, WorkspaceModelDto,
    WorkspaceModelSummaryDto,
};

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlVisualizationParamsDto {
    pub workspace_root_uri: String,
    pub view: String,
    #[serde(default)]
    pub selected_view: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlModelResultDto {
    pub version: u32,
    pub graph: Option<SysmlGraphDto>,
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
