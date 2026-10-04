//! Port of the drawing-relevant half of `views/behavior-common.ts`, shared by `action_flow.rs`,
//! `state_transition.rs`, and (for `PreparedView`/`PreparedNode`/`PreparedEdge` only) `sequence.rs`.
//! Layout (`buildBehaviorElkGraphInput`/`layoutBehaviorGraph`) lives in the crate's private
//! `layout` module.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::svg::format_number as n;
use crate::types::{EdgeSection, Point};

#[derive(Debug, Deserialize)]
pub struct PreparedNode {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct PreparedEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    #[serde(rename = "edgeKind", default)]
    pub edge_kind: Option<String>,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct PreparedView {
    pub title: String,
    pub view: String,
    pub nodes: Vec<PreparedNode>,
    pub edges: Vec<PreparedEdge>,
    #[serde(default)]
    pub meta: Value,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct LaidOutRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct ElkLabelBox {
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct BehaviorLayoutResult {
    pub positions: BTreeMap<String, LaidOutRect>,
    #[serde(rename = "edgeSectionsById", default)]
    pub edge_sections_by_id: BTreeMap<String, Vec<EdgeSection>>,
    #[serde(rename = "edgeLabelsById", default)]
    pub edge_labels_by_id: BTreeMap<String, Vec<ElkLabelBox>>,
}

/// Port of `nodeKind`: `String(node.kind || "action").toLowerCase()`.
pub fn node_kind(node: &PreparedNode) -> String {
    if node.kind.is_empty() {
        "action".to_string()
    } else {
        node.kind.to_lowercase()
    }
}

fn section_points(section: &EdgeSection) -> Vec<Point> {
    let mut points = Vec::new();
    if let Some(start) = section.start_point {
        points.push(start);
    }
    points.extend(section.bend_points.iter().copied());
    if let Some(end) = section.end_point {
        points.push(end);
    }
    points
}

/// Port of `pathFromSections`.
pub fn path_from_sections(sections: Option<&[EdgeSection]>) -> Option<String> {
    let sections = sections?;
    if sections.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    for section in sections {
        let (Some(start), Some(end)) = (section.start_point, section.end_point) else {
            continue;
        };
        parts.push(format!("M{},{}", n(start.x), n(start.y)));
        for point in &section.bend_points {
            parts.push(format!("L{},{}", n(point.x), n(point.y)));
        }
        parts.push(format!("L{},{}", n(end.x), n(end.y)));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

pub struct FallbackPath {
    pub path: String,
    pub label_x: f64,
    pub label_y: f64,
}

/// Port of `buildSelfLoopPath`.
pub fn build_self_loop_path(node: &LaidOutRect) -> FallbackPath {
    let start_x = node.x + node.width;
    let start_y = node.y + node.height / 2.0 - 8.0;
    let loop_radius = 28.0;
    FallbackPath {
        path: format!(
            "M{},{} C{},{} {},{} {},{}",
            n(start_x),
            n(start_y),
            n(start_x + loop_radius),
            n(start_y - loop_radius),
            n(start_x + loop_radius),
            n(start_y + loop_radius),
            n(start_x),
            n(start_y + 18.0)
        ),
        label_x: start_x + loop_radius + 8.0,
        label_y: start_y,
    }
}

/// Port of `fallbackEdgePath`.
pub fn fallback_edge_path(
    source: &LaidOutRect,
    target: &LaidOutRect,
    horizontal: bool,
) -> FallbackPath {
    if source.x == target.x
        && source.y == target.y
        && source.width == target.width
        && source.height == target.height
    {
        return build_self_loop_path(source);
    }
    if horizontal {
        let start_x = source.x + source.width;
        let start_y = source.y + source.height / 2.0;
        let end_x = target.x;
        let end_y = target.y + target.height / 2.0;
        let mid_x = (start_x + end_x) / 2.0;
        return FallbackPath {
            path: format!(
                "M{},{} L{},{} L{},{} L{},{}",
                n(start_x),
                n(start_y),
                n(mid_x),
                n(start_y),
                n(mid_x),
                n(end_y),
                n(end_x),
                n(end_y)
            ),
            label_x: mid_x,
            label_y: (start_y + end_y) / 2.0 - 6.0,
        };
    }
    let start_x = source.x + source.width / 2.0;
    let start_y = source.y + source.height;
    let end_x = target.x + target.width / 2.0;
    let end_y = target.y;
    let mid_y = (start_y + end_y) / 2.0;
    FallbackPath {
        path: format!(
            "M{},{} L{},{} L{},{} L{},{}",
            n(start_x),
            n(start_y),
            n(start_x),
            n(mid_y),
            n(end_x),
            n(mid_y),
            n(end_x),
            n(end_y)
        ),
        label_x: (start_x + end_x) / 2.0,
        label_y: mid_y - 6.0,
    }
}

/// Port of `truncateLabel`. JS `String.length`/`slice` are UTF-16 code units, not Unicode scalar
/// values -- match with `encode_utf16` rather than `chars()` (unlike General View's
/// `truncate_to_chars`, which never needed non-BMP parity).
pub fn truncate_label(text: &str, max: usize) -> String {
    let trimmed = text.trim();
    let units: Vec<u16> = trimmed.encode_utf16().collect();
    if units.len() > max {
        let truncated = String::from_utf16_lossy(&units[..max.saturating_sub(2)]);
        format!("{truncated}..")
    } else {
        trimmed.to_string()
    }
}

/// Port of `edgeLabelPositionFromSections`.
pub fn edge_label_position_from_sections(sections: Option<&[EdgeSection]>) -> Option<(f64, f64)> {
    let section = sections?.first()?;
    if section.start_point.is_none() || section.end_point.is_none() {
        return None;
    }
    let points = section_points(section);
    let mid_index = (points.len() - 1) / 2;
    let start = points[mid_index];
    let end = points.get(mid_index + 1).copied().unwrap_or(start);
    Some(((start.x + end.x) / 2.0, (start.y + end.y) / 2.0 - 6.0))
}
