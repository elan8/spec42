//! DTOs and conversion helpers for sysml/model and related responses.

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::Range;

use crate::views::extracted_model as model;
use crate::views::ibd;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
pub struct SysmlDiagramParamsDto {
    pub text_document: TextDocumentIdentifierDto,
    pub kind: String,
    #[serde(default)]
    pub options: Option<SysmlDiagramOptionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlDiagramOptionsDto {
    #[serde(default)]
    pub workspace_visualization: Option<bool>,
    #[serde(default)]
    pub root: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramPointDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramBoundsDto {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramNodeCompartmentsDto {
    pub stereotype: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_by_name: Option<String>,
    pub attributes: Vec<String>,
    pub parts: Vec<String>,
    pub ports: Vec<String>,
    pub other: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralDiagramNodeDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub category: String,
    pub is_definition: bool,
    pub compartments: DiagramNodeCompartmentsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralDiagramEdgeDto {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub points: Vec<DiagramPointDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralDiagramSceneDto {
    pub nodes: Vec<GeneralDiagramNodeDto>,
    pub edges: Vec<GeneralDiagramEdgeDto>,
    pub bounds: DiagramBoundsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdScenePartDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_container: bool,
    pub depth: u32,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdScenePortDto {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub x: f32,
    pub y: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_side: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdSceneConnectorDto {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_id: String,
    pub target_id: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    pub points: Vec<DiagramPointDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdSceneRootDto {
    pub name: String,
    pub parts: Vec<IbdScenePartDto>,
    pub ports: Vec<IbdScenePortDto>,
    pub connectors: Vec<IbdSceneConnectorDto>,
    pub bounds: DiagramBoundsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdDiagramSceneDto {
    pub root_candidates: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_root: Option<String>,
    pub roots: std::collections::HashMap<String, IbdSceneRootDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramSceneDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view: Option<GeneralDiagramSceneDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnection_view: Option<IbdDiagramSceneDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlDiagramStatsDto {
    pub node_count: u32,
    pub edge_count: u32,
    pub build_time_ms: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlDiagramResultDto {
    pub version: u32,
    pub kind: String,
    pub source_uri: String,
    pub scene: DiagramSceneDto,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<SysmlDiagramStatsDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlModelResultDto {
    pub version: u32,
    pub graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view_graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_model: Option<WorkspaceModelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_diagrams: Option<Vec<model::ActivityDiagramDto>>,
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
