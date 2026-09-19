//! Port of the Interconnection-View branches of `drawEdges`/`applyEdgeMarker`/`pathForIbdEdge`/
//! `interconnectionEdgeLabelAnchor` in `render/drawing.ts` (the General-View branches are already
//! ported in `edges.rs`).

use std::collections::HashMap;

use crate::graph_normalization::normalize_edge_kind;
use crate::hit_target::{mark_visible_edge, path_edge_hit_target};
use crate::ibd_route::resolve_ibd_route_points;
use crate::svg::{format_number as n, Element};
use crate::theme::{stroke_color_for_edge, Theme};
use crate::tooltip::{edge_tooltip_descriptor_interconnection, tooltip_fallback_text};
use crate::types::{attr_text, InterconnectionLayoutEdgeDto, LaidOutEdge, LaidOutNode, Point};

/// Port of the `isInterconnectionView` branch of `applyEdgeMarker`.
struct MarkerStyle {
    unsupported: bool,
    /// `(attr_name, value)` pairs applied via `.attr()`, in call order (only `flow`/`reference`
    /// touch `stroke-width` as a plain attribute, matching the original exactly).
    attrs: Vec<(&'static str, &'static str)>,
    /// `.style()` pairs, in call order -- order matters, since they compose into one string.
    styles: Vec<(&'static str, String)>,
}

fn marker_url(id: &'static str) -> String {
    format!("url(#{id})")
}

fn apply_edge_marker_ibd(edge_kind: &str, theme: &Theme) -> MarkerStyle {
    let stroke = stroke_color_for_edge(theme);
    match edge_kind {
        "flow" => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke), ("stroke-width", "2.5")],
            styles: vec![("marker-end", marker_url("ibd-flow-arrow"))],
        },
        "interface" => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke)],
            styles: vec![
                ("stroke-dasharray", "8,4".to_string()),
                ("marker-end", marker_url("ibd-interface-arrow")),
            ],
        },
        "bind" | "binding" => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke)],
            styles: vec![
                ("stroke-dasharray", "6,4".to_string()),
                ("marker-start", marker_url("ibd-connection-dot")),
                ("marker-end", marker_url("ibd-connection-dot")),
            ],
        },
        "reference" => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke), ("stroke-width", "1.6")],
            styles: vec![
                ("stroke-dasharray", "4,4".to_string()),
                ("marker-start", marker_url("ibd-connection-dot")),
                ("marker-end", marker_url("ibd-connection-dot")),
            ],
        },
        "connection" | "relationship" => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke), ("stroke-width", "2")],
            styles: vec![
                ("marker-start", marker_url("ibd-connection-dot")),
                ("marker-end", marker_url("ibd-connection-dot")),
            ],
        },
        _ => MarkerStyle {
            unsupported: true,
            attrs: vec![],
            styles: vec![
                ("stroke-dasharray", "2,4".to_string()),
                ("marker-start", "none".to_string()),
                ("marker-end", "none".to_string()),
            ],
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

/// Port of `pathForIbdEdge`.
fn path_for_ibd_edge(
    edge: &LaidOutEdge,
    layout_edges_by_id: &HashMap<&str, &InterconnectionLayoutEdgeDto>,
) -> Option<String> {
    let layout_edge = layout_edges_by_id.get(edge.id.as_str());
    let points = match layout_edge {
        Some(layout_edge) if layout_edge.route_points.len() >= 2 => {
            layout_edge.route_points.clone()
        }
        _ => resolve_ibd_route_points(edge)?,
    };
    if points.len() < 2 {
        return None;
    }
    Some(points_to_path_d(&points))
}

fn route_points_for_edge(
    edge: &LaidOutEdge,
    layout_edges_by_id: &HashMap<&str, &InterconnectionLayoutEdgeDto>,
) -> Vec<Point> {
    match layout_edges_by_id.get(edge.id.as_str()) {
        Some(layout_edge) if !layout_edge.route_points.is_empty() => {
            layout_edge.route_points.clone()
        }
        _ => resolve_ibd_route_points(edge).unwrap_or_default(),
    }
}

struct LabelAnchor {
    x: f64,
    y: f64,
    text_anchor: &'static str,
    dy: &'static str,
}

/// Port of `interconnectionEdgeLabelAnchor`.
fn interconnection_edge_label_anchor(
    edge: &LaidOutEdge,
    layout_edges_by_id: &HashMap<&str, &InterconnectionLayoutEdgeDto>,
) -> Option<LabelAnchor> {
    let route_points = route_points_for_edge(edge, layout_edges_by_id);
    if route_points.len() < 2 {
        return None;
    }
    struct Segment {
        start: Point,
        end: Point,
        horizontal: bool,
        length: f64,
    }
    let segments: Vec<Segment> = route_points
        .windows(2)
        .map(|pair| {
            let (start, end) = (pair[0], pair[1]);
            Segment {
                start,
                end,
                horizontal: (start.y - end.y).abs() < 1e-6,
                length: (end.x - start.x).hypot(end.y - start.y),
            }
        })
        .collect();
    let longest_horizontal = segments
        .iter()
        .filter(|s| s.horizontal && s.length >= 24.0)
        .max_by(|a, b| a.length.total_cmp(&b.length));
    let segment = longest_horizontal
        .or_else(|| segments.iter().max_by(|a, b| a.length.total_cmp(&b.length)))?;
    let x = (segment.start.x + segment.end.x) / 2.0;
    let y = (segment.start.y + segment.end.y) / 2.0;
    Some(if segment.horizontal {
        LabelAnchor {
            x,
            y,
            text_anchor: "middle",
            dy: "-0.55em",
        }
    } else {
        LabelAnchor {
            x: x + 8.0,
            y,
            text_anchor: "start",
            dy: "0.35em",
        }
    })
}

/// Port of `ibdEdgeDisplayLabel`.
fn ibd_edge_display_label(edge: &LaidOutEdge, edge_kind: &str) -> String {
    let item_type = attr_text(&edge.attributes, "itemType");
    if edge_kind == "flow" && !item_type.is_empty() {
        return item_type;
    }
    let interface_name = attr_text(&edge.attributes, "interfaceName");
    if edge_kind == "interface" && !interface_name.is_empty() {
        return interface_name;
    }
    let label = edge.label.trim();
    let relation_type = attr_text(&edge.attributes, "relationType");
    const GENERIC: &[&str] = &[
        "",
        "connect",
        "connection",
        "flow",
        "interface",
        "binding",
        "bind",
        "reference",
        "ref",
        "relationship",
    ];
    if GENERIC.contains(&label.to_lowercase().as_str())
        || GENERIC.contains(&relation_type.to_lowercase().as_str())
    {
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

/// Port of the Interconnection-View branch of `drawEdges`.
pub fn draw_ibd_edges(
    edges: &[LaidOutEdge],
    nodes_by_id: &HashMap<&str, &LaidOutNode>,
    layout_edges: &[InterconnectionLayoutEdgeDto],
    theme: &Theme,
) -> Vec<Element> {
    let layout_edges_by_id: HashMap<&str, &InterconnectionLayoutEdgeDto> =
        layout_edges.iter().map(|e| (e.id.as_str(), e)).collect();

    let mut edge_layer = Element::new("g").attr("class", "viz-edges");
    struct PendingLabel {
        edge_id: String,
        edge_kind: String,
        display_label: String,
        anchor: LabelAnchor,
    }
    let mut labels = Vec::new();

    for edge in edges {
        let (Some(&source), Some(&target)) = (
            nodes_by_id.get(edge.source.as_str()),
            nodes_by_id.get(edge.target.as_str()),
        ) else {
            continue;
        };
        let Some(path) = path_for_ibd_edge(edge, &layout_edges_by_id) else {
            continue;
        };
        let edge_kind = edge
            .edge_kind
            .clone()
            .unwrap_or_else(|| normalize_edge_kind(&edge.label));
        let display_label = ibd_edge_display_label(edge, &edge_kind);
        let stroke = stroke_color_for_edge(theme);
        let stroke_width = if edge_kind == "hierarchy" { 1.4 } else { 2.0 };
        let data_type = {
            let relation_type = attr_text(&edge.attributes, "relationType");
            if !relation_type.is_empty() {
                relation_type
            } else {
                edge_kind.clone()
            }
        };

        let mut path_el = Element::new("path")
            .attr(
                "class",
                format!("ibd-connector viz-edge viz-edge--{edge_kind}"),
            )
            .attr("d", path.clone())
            .attr("data-connector-id", edge.id.clone())
            .attr("data-source", edge.source.clone())
            .attr("data-target", edge.target.clone())
            .attr("data-type", data_type)
            .style("fill", "none")
            .style("stroke", stroke)
            .style("stroke-width", n(stroke_width))
            .style("opacity", "0.9");
        path_el = mark_visible_edge(path_el, &edge.id, &n(stroke_width));

        let marker = apply_edge_marker_ibd(&edge_kind, theme);
        if marker.unsupported {
            path_el = path_el.attr("data-notation-status", "unsupported");
        }
        for (name, value) in marker.attrs {
            path_el = path_el.attr(name, value);
        }
        for (name, value) in marker.styles {
            path_el = path_el.style(name, value);
        }
        edge_layer = edge_layer.child(path_el);

        let tooltip_text =
            tooltip_fallback_text(&edge_tooltip_descriptor_interconnection(edge, nodes_by_id));
        edge_layer = edge_layer.child(path_edge_hit_target(&path, &edge.id, Some(&tooltip_text)));

        if !display_label.is_empty() {
            if let Some(anchor) = interconnection_edge_label_anchor(edge, &layout_edges_by_id) {
                labels.push(PendingLabel {
                    edge_id: edge.id.clone(),
                    edge_kind,
                    display_label,
                    anchor,
                });
            } else {
                // Fallback anchor: node-box midpoint (only reachable when route points are
                // unavailable -- `edgeLabelAnchor`'s final fallback in the TS source).
                let anchor = LabelAnchor {
                    x: (source.x + target.x + crate::types::IBD_NODE_WIDTH) / 2.0,
                    y: (source.y + target.y + crate::types::IBD_NODE_HEIGHT) / 2.0,
                    text_anchor: "middle",
                    dy: "-0.35em",
                };
                labels.push(PendingLabel {
                    edge_id: edge.id.clone(),
                    edge_kind,
                    display_label,
                    anchor,
                });
            }
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
                .attr("data-connector-id", label.edge_id)
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
