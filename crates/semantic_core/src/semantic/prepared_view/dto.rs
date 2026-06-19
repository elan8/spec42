//! Render-ready diagram payloads consumed by ELK layout and SVG drawing.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::semantic::dto::RangeDto;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreparedNodeDto {
    pub id: String,
    pub label: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreparedEdgeDto {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreparedViewDto {
    pub title: String,
    pub view: String,
    pub nodes: Vec<PreparedNodeDto>,
    pub edges: Vec<PreparedEdgeDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}
