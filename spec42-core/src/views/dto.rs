//! DTOs and conversion helpers for sysml/model and related responses.

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::Range;

use crate::views::diagram_types::{Bounds, HitRegionKind, RenderedDiagram};
use crate::views::extracted_model as model;
use crate::views::ibd;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct SysmlElementDto {
    #[serde(rename = "type")]
    pub element_type: String,
    pub name: String,
    pub range: RangeDto,
    pub children: Vec<SysmlElementDto>,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
    pub relationships: Vec<RelationshipDto>,
    pub errors: Option<Vec<String>>,
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
pub struct DiagramBoundsDto {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HitRegionDto {
    pub id: String,
    pub kind: String,
    pub element_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    pub bounds: DiagramBoundsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramMetricsDto {
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramViewStateDto {
    pub view: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderedDiagramDto {
    pub svg: String,
    pub hit_map: Vec<HitRegionDto>,
    pub bounds: DiagramBoundsDto,
    pub metrics: DiagramMetricsDto,
    pub warnings: Vec<String>,
    pub view_state: DiagramViewStateDto,
}

/// Map of diagram id (e.g. "generalView", "interconnectionView") to rendered diagram.
/// Serializes as a flat JSON object so clients get renderedDiagrams: { "generalView": {...}, ... }.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct RenderedDiagramsDto(pub std::collections::HashMap<String, RenderedDiagramDto>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlModelResultDto {
    pub version: u32,
    pub graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_view_graph: Option<SysmlGraphDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_diagrams: Option<Vec<model::ActivityDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_diagrams: Option<Vec<model::SequenceDiagramDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibd: Option<ibd::IbdDataDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_diagrams: Option<RenderedDiagramsDto>,
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

pub fn bounds_to_dto(bounds: Bounds) -> DiagramBoundsDto {
    DiagramBoundsDto {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width,
        height: bounds.height,
    }
}

pub fn rendered_diagram_to_dto(diagram: RenderedDiagram) -> RenderedDiagramDto {
    RenderedDiagramDto {
        svg: diagram.svg,
        hit_map: diagram
            .hit_map
            .into_iter()
            .map(|hit| HitRegionDto {
                id: hit.id,
                kind: match hit.kind {
                    HitRegionKind::Node => "node".to_string(),
                    HitRegionKind::Port => "port".to_string(),
                    HitRegionKind::EdgeLabel => "label".to_string(),
                },
                element_id: hit.element_id,
                qualified_name: hit.qualified_name,
                bounds: bounds_to_dto(hit.bounds),
            })
            .collect(),
        bounds: bounds_to_dto(diagram.bounds),
        metrics: DiagramMetricsDto {
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
        view_state: DiagramViewStateDto {
            view: diagram.view_state.view,
            selection: diagram.view_state.selection,
        },
    }
}
