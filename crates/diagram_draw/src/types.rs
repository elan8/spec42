use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::sysml_node::Compartments;

#[derive(Debug, Clone, Copy, Deserialize, serde::Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct EdgeSection {
    #[serde(rename = "startPoint")]
    pub start_point: Option<Point>,
    #[serde(rename = "bendPoints", default)]
    pub bend_points: Vec<Point>,
    #[serde(rename = "endPoint")]
    pub end_point: Option<Point>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct EdgeLayout {
    #[serde(default)]
    pub sections: Vec<EdgeSection>,
    #[serde(rename = "edgeOwnerOffset", default)]
    pub edge_owner_offset: Option<Point>,
    #[serde(rename = "lcaOffset", default)]
    pub lca_offset: Option<Point>,
}

pub const IBD_NODE_WIDTH: f64 = 280.0;
pub const IBD_NODE_HEIGHT: f64 = 140.0;

#[derive(Debug, Clone, Deserialize)]
pub struct LaidOutNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    /// The layout pass's own `collectCompartments(node)` snapshot (`reshapeGeneralLayoutResult` in
    /// `render/layout.ts`). `drawNodes` prefers this over recomputing from `attributes` --
    /// `d.compartments ?? collectCompartments(d)` -- so this must be too.
    #[serde(default)]
    pub compartments: Option<Compartments>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LaidOutEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    #[serde(rename = "edgeKind")]
    pub edge_kind: Option<String>,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
    pub layout: Option<EdgeLayout>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralViewGraph {
    pub title: String,
    #[serde(default)]
    pub meta: Value,
    pub nodes: Vec<LaidOutNode>,
    pub edges: Vec<LaidOutEdge>,
}

/// Port of `InterconnectionLayoutPortAnchor` (`prepare/types.ts:102-113`). `label` is node-relative
/// pre-measured text placement, present only when the layout pass resolved a label box for this
/// port.
#[derive(Debug, Deserialize)]
pub struct InterconnectionLayoutPortLabel {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct InterconnectionLayoutPortAnchor {
    pub x: f64,
    pub y: f64,
    pub side: String,
    #[serde(default)]
    pub label: Option<InterconnectionLayoutPortLabel>,
}

#[derive(Debug, Default, Deserialize)]
pub struct InterconnectionLayoutPortDrawOrder {
    #[serde(default)]
    pub west: Vec<String>,
    #[serde(default)]
    pub east: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct InterconnectionLayoutNodeDto {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(rename = "portAnchors", default)]
    pub port_anchors: BTreeMap<String, InterconnectionLayoutPortAnchor>,
    #[serde(rename = "portDrawOrder", default)]
    pub port_draw_order: Option<InterconnectionLayoutPortDrawOrder>,
}

#[derive(Debug, Deserialize)]
pub struct InterconnectionLayoutEdgeDto {
    #[allow(dead_code)]
    pub id: String,
    #[serde(rename = "routePoints", default)]
    pub route_points: Vec<Point>,
}

#[derive(Debug, Deserialize)]
pub struct InterconnectionLayoutContainerDto {
    #[allow(dead_code)]
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Port of `InterconnectionLayoutDto` (`prepare/types.ts:146-151`) -- the already-laid-out
/// interconnection graph: absolute node boxes, node-relative port anchors/draw order, edge route
/// points, and containers. `render/layout.ts`'s `layoutInterconnectionPrepared` omits this field
/// entirely on ELK failure, so it stays optional on `InterconnectionViewGraph`.
#[derive(Debug, Default, Deserialize)]
pub struct InterconnectionLayoutDto {
    #[serde(default)]
    pub nodes: Vec<InterconnectionLayoutNodeDto>,
    #[serde(default)]
    pub edges: Vec<InterconnectionLayoutEdgeDto>,
    #[serde(default)]
    pub containers: Vec<InterconnectionLayoutContainerDto>,
}

#[derive(Debug, Deserialize)]
pub struct InterconnectionViewGraph {
    pub title: String,
    #[serde(default)]
    pub meta: Value,
    pub nodes: Vec<LaidOutNode>,
    pub edges: Vec<LaidOutEdge>,
    #[serde(rename = "interconnectionLayout", default)]
    pub interconnection_layout: Option<InterconnectionLayoutDto>,
}

pub fn attr_str(attributes: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    match attributes.get(key) {
        Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        _ => None,
    }
}

/// Port of `text(value)` in `render/diagram-tooltip.ts`: `String(value ?? "").trim()`.
pub fn attr_text(attributes: &BTreeMap<String, Value>, key: &str) -> String {
    match attributes.get(key) {
        Some(Value::String(s)) => s.trim().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

pub fn attr_bool(attributes: &BTreeMap<String, Value>, key: &str) -> bool {
    matches!(attributes.get(key), Some(Value::Bool(true)))
}

pub fn attr_u32(attributes: &BTreeMap<String, Value>, key: &str) -> Option<u32> {
    match attributes.get(key) {
        Some(Value::Number(n)) => n.as_u64().map(|value| value as u32),
        _ => None,
    }
}
