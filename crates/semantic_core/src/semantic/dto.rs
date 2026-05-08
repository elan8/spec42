//! Shared DTO primitives for semantic/model and visualization payloads.

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::Range;

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
