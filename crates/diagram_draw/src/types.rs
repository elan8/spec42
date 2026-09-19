use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct EdgeSection {
    #[serde(rename = "startPoint")]
    pub start_point: Option<Point>,
    #[serde(rename = "bendPoints", default)]
    pub bend_points: Vec<Point>,
    #[serde(rename = "endPoint")]
    pub end_point: Option<Point>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EdgeLayout {
    #[serde(default)]
    pub sections: Vec<EdgeSection>,
}

#[derive(Debug, Deserialize)]
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
}

#[derive(Debug, Deserialize)]
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

pub fn attr_str(attributes: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    match attributes.get(key) {
        Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        _ => None,
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
