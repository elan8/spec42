use std::collections::{BTreeMap, HashMap};

use serde_json::Value;

use crate::graph_normalization::normalize_edge_kind;
use crate::svg::{format_number as n, Element};
use crate::theme::{stroke_color_for_edge, Theme};
use crate::types::{EdgeSection, LaidOutEdge, LaidOutNode, Point};

/// General-View branch of `applyEdgeMarker` in `render/drawing.ts` (the interconnection/IBD
/// branch is out of scope for this spike -- General View is the only view being ported).
struct MarkerStyle {
    stroke_dasharray: Option<&'static str>,
    marker_start: Option<&'static str>,
    marker_end: Option<&'static str>,
    stroke_width: Option<&'static str>,
    unsupported: bool,
}

fn apply_edge_marker(edge_kind: &str) -> MarkerStyle {
    let none = MarkerStyle {
        stroke_dasharray: None,
        marker_start: None,
        marker_end: None,
        stroke_width: None,
        unsupported: false,
    };
    match edge_kind {
        "specializes" => MarkerStyle {
            marker_end: Some("general-d3-specializes"),
            stroke_width: Some("1.7px"),
            ..none
        },
        "subsetting" => MarkerStyle {
            marker_end: Some("general-d3-arrow-open"),
            stroke_dasharray: Some("6,3"),
            ..none
        },
        "typing" => MarkerStyle {
            marker_end: Some("general-d3-arrow-open"),
            stroke_dasharray: Some("5,3"),
            ..none
        },
        "hierarchy" => MarkerStyle {
            marker_start: Some("general-d3-diamond"),
            marker_end: Some("none"),
            ..none
        },
        "bind" => MarkerStyle {
            stroke_dasharray: Some("2,2"),
            marker_end: Some("none"),
            ..none
        },
        "allocate" => MarkerStyle {
            marker_end: Some("general-d3-arrow"),
            stroke_dasharray: Some("8,4"),
            ..none
        },
        "dependency" | "usage" => MarkerStyle {
            marker_end: Some("general-d3-arrow-open"),
            stroke_dasharray: Some("4,4"),
            ..none
        },
        "redefinition" => MarkerStyle {
            marker_end: Some("general-d3-specializes"),
            stroke_dasharray: Some("5,3"),
            ..none
        },
        "composition" => MarkerStyle {
            marker_start: Some("general-d3-diamond"),
            marker_end: Some("none"),
            stroke_dasharray: Some("6,3"),
            ..none
        },
        "connection" => MarkerStyle {
            marker_start: Some("none"),
            marker_end: Some("none"),
            ..none
        },
        "satisfy" | "verify" | "derivation" => MarkerStyle {
            marker_end: Some("general-d3-arrow-open"),
            stroke_dasharray: Some("7,4"),
            ..none
        },
        _ => MarkerStyle {
            stroke_dasharray: Some("2,4"),
            marker_start: Some("none"),
            marker_end: Some("none"),
            unsupported: true,
            ..none
        },
    }
}

fn points_to_path_d(points: &[Point]) -> String {
    if points.len() < 2 {
        return String::new();
    }
    points
        .iter()
        .enumerate()
        .map(|(index, p)| {
            format!(
                "{}{},{}",
                if index == 0 { "M" } else { "L" },
                n(p.x),
                n(p.y)
            )
        })
        .collect()
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

fn path_from_simple_section(section: Option<&EdgeSection>) -> Option<String> {
    let section = section?;
    let points = section_points(section);
    if points.len() < 2 {
        return None;
    }
    Some(points_to_path_d(&points))
}

fn attr_string(attributes: &BTreeMap<String, Value>, key: &str) -> String {
    match attributes.get(key) {
        Some(Value::String(s)) => s.trim().to_string(),
        _ => String::new(),
    }
}

/// Port of `generalEdgeDisplayLabel` in `render/drawing.ts`.
fn general_edge_display_label(edge: &LaidOutEdge, edge_kind: &str) -> String {
    const GENERIC: &[&str] = &[
        "",
        "relationship",
        "edge",
        "connect",
        "connection",
        "dependency",
        "specializes",
        "specialization",
        "typing",
        "defined_by",
        "defined by",
        "definition",
        "hierarchy",
        "contains",
        "owns",
        "ownership",
        "containment",
        "allocate",
        "allocation",
        "satisfy",
        "verify",
        "bind",
        "binding",
    ];
    let label = edge.label.trim();
    let relation_type = attr_string(&edge.attributes, "relationType");
    let lower_label = label.to_lowercase();
    if GENERIC.contains(&lower_label.as_str()) {
        return String::new();
    }
    if lower_label == relation_type.to_lowercase() || lower_label == edge_kind.to_lowercase() {
        return String::new();
    }
    label.to_string()
}

fn truncate(value: &str, max: usize) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() > max {
        format!(
            "{}...",
            chars[..max.saturating_sub(1)].iter().collect::<String>()
        )
    } else {
        value.to_string()
    }
}

struct LabelAnchor {
    x: f64,
    y: f64,
    text_anchor: &'static str,
    dy: &'static str,
}

/// Port of the non-interconnection branch of `edgeLabelAnchor` in `render/drawing.ts`.
fn edge_label_anchor(
    edge: &LaidOutEdge,
    source: Option<&LaidOutNode>,
    target: Option<&LaidOutNode>,
) -> LabelAnchor {
    if let Some(section) = edge.layout.as_ref().and_then(|l| l.sections.first()) {
        let points = section_points(section);
        if !points.is_empty() {
            let index = (points.len() - 1) / 2;
            let point = points[index];
            return LabelAnchor {
                x: point.x,
                y: point.y,
                text_anchor: "middle",
                dy: "-0.35em",
            };
        }
    }
    if let (Some(source), Some(target)) = (source, target) {
        const NODE_WIDTH: f64 = 200.0;
        const NODE_HEIGHT: f64 = 70.0;
        return LabelAnchor {
            x: (source.x + target.x + NODE_WIDTH) / 2.0,
            y: (source.y + target.y + NODE_HEIGHT) / 2.0,
            text_anchor: "middle",
            dy: "-0.35em",
        };
    }
    LabelAnchor {
        x: 0.0,
        y: 0.0,
        text_anchor: "middle",
        dy: "-0.35em",
    }
}

/// Port of `drawEdges`'s General-View branch in `render/drawing.ts` (the `isInterconnectionView`
/// branches -- `pathForIbdEdge`, `ibdEdgeDisplayLabel`, IBD marker styling -- are out of scope for
/// this spike).
pub fn draw_edges(
    edges: &[LaidOutEdge],
    nodes_by_id: &HashMap<&str, &LaidOutNode>,
    theme: &Theme,
) -> Vec<Element> {
    let mut edge_layer = Element::new("g").attr("class", "viz-edges");
    struct PendingLabel {
        edge_kind: String,
        display_label: String,
        anchor: LabelAnchor,
    }
    let mut labels = Vec::new();

    for edge in edges {
        let Some(&source) = nodes_by_id.get(edge.source.as_str()) else {
            continue;
        };
        let Some(&target) = nodes_by_id.get(edge.target.as_str()) else {
            continue;
        };
        let Some(path) =
            path_from_simple_section(edge.layout.as_ref().and_then(|l| l.sections.first()))
        else {
            continue;
        };
        let edge_kind = edge
            .edge_kind
            .clone()
            .unwrap_or_else(|| normalize_edge_kind(&edge.label));
        let display_label = general_edge_display_label(edge, &edge_kind);
        let stroke = stroke_color_for_edge(theme);
        // `edgeKind === "hierarchy" ? 1.4 : isInterconnectionView ? 2 : 1.8` in `drawEdges` --
        // General View is never `isInterconnectionView`, so that branch collapses to `1.8`.
        let stroke_width = if edge_kind == "hierarchy" { 1.4 } else { 1.8 };
        let data_type = {
            let relation_type = attr_string(&edge.attributes, "relationType");
            if !relation_type.is_empty() {
                relation_type
            } else {
                edge_kind.clone()
            }
        };

        let mut path_el = Element::new("path")
            .attr(
                "class",
                format!("general-connector viz-edge viz-edge--{edge_kind}"),
            )
            .attr("d", path.clone())
            .attr("data-connector-id", edge.id.clone())
            .attr("data-source", edge.source.clone())
            .attr("data-target", edge.target.clone())
            .attr("data-type", data_type)
            .attr("data-edge-id", edge.id.clone())
            .attr("data-base-stroke-width", n(stroke_width))
            .style("fill", "none")
            .style("stroke", stroke)
            .style("stroke-width", n(stroke_width))
            .style("opacity", "0.9");

        let marker = apply_edge_marker(&edge_kind);
        if marker.unsupported {
            path_el = path_el.attr("data-notation-status", "unsupported");
        }
        if let Some(dash) = marker.stroke_dasharray {
            path_el = path_el.style("stroke-dasharray", dash);
        }
        if let Some(width) = marker.stroke_width {
            path_el = path_el.attr("stroke", stroke).style("stroke-width", width);
        }
        if let Some(marker_start) = marker.marker_start {
            path_el = path_el.style(
                "marker-start",
                if marker_start == "none" {
                    "none".to_string()
                } else {
                    format!("url(#{marker_start})")
                },
            );
        }
        if let Some(marker_end) = marker.marker_end {
            path_el = path_el.style(
                "marker-end",
                if marker_end == "none" {
                    "none".to_string()
                } else {
                    format!("url(#{marker_end})")
                },
            );
        }
        edge_layer = edge_layer.child(path_el);

        edge_layer = edge_layer.child(
            Element::new("path")
                .attr("class", "viz-edge-hit-target")
                .attr("data-tooltip-kind", "edge")
                .attr("data-tooltip-id", edge.id.clone())
                .attr("d", path)
                .style("fill", "none")
                .style("stroke", "transparent")
                .style("stroke-width", "12px")
                .style("pointer-events", "stroke"),
        );

        if !display_label.is_empty() {
            labels.push(PendingLabel {
                edge_kind,
                display_label,
                anchor: edge_label_anchor(edge, Some(source), Some(target)),
            });
        }
    }

    if labels.is_empty() {
        return vec![edge_layer];
    }

    let mut label_layer = Element::new("g")
        .attr("class", "viz-edge-labels")
        .style("pointer-events", "none");
    for label in labels {
        label_layer = label_layer.child(
            Element::new("text")
                .attr(
                    "class",
                    format!("viz-edge-label viz-edge-label--{}", label.edge_kind),
                )
                .attr_f("x", label.anchor.x)
                .attr_f("y", label.anchor.y)
                .attr("text-anchor", label.anchor.text_anchor)
                .attr("dy", label.anchor.dy)
                .attr("fill", theme.text_primary)
                .attr("font-size", "11")
                .attr("paint-order", "stroke fill")
                .attr("stroke", theme.canvas_background)
                .attr("stroke-width", "4")
                .attr("stroke-linejoin", "round")
                .text(truncate(&label.display_label, 18)),
        );
    }

    vec![edge_layer, label_layer]
}
