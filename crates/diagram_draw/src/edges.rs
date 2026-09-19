use std::collections::HashMap;

use crate::graph_normalization::normalize_edge_kind;
use crate::svg::{format_number as n, Element};
use crate::theme::{stroke_color_for_edge, Theme};
use crate::tooltip::{edge_tooltip_descriptor, tooltip_fallback_text};
use crate::types::{attr_text, EdgeSection, LaidOutEdge, LaidOutNode, Point};

/// General-View branch of `applyEdgeMarker` in `render/drawing.ts` (the interconnection/IBD
/// branch is out of scope for this spike -- General View is the only view being ported).
///
/// `style` values live in one composed `style="..."` string attribute, so -- unlike ordinary
/// attributes -- the *order* `.style()` is called in is observable output, not just an
/// implementation detail. `styles` therefore preserves each branch's exact TS call order rather
/// than applying a fixed order generically (a mismatch here is exactly the class of drift a
/// class-count-only test cannot see; see `tests/golden_parity.rs`).
struct MarkerStyle {
    unsupported: bool,
    styles: Vec<(&'static str, &'static str)>,
}

fn marker_url(id: &'static str) -> &'static str {
    // `applyEdgeMarker` always spells `url(#...)` inline; the ids below are the only ones this
    // branch ever needs, so a small static table avoids allocating per edge.
    match id {
        "general-d3-specializes" => "url(#general-d3-specializes)",
        "general-d3-arrow-open" => "url(#general-d3-arrow-open)",
        "general-d3-diamond" => "url(#general-d3-diamond)",
        "general-d3-arrow" => "url(#general-d3-arrow)",
        other => other,
    }
}

fn apply_edge_marker(edge_kind: &str) -> MarkerStyle {
    let ms = |unsupported: bool, styles: Vec<(&'static str, &'static str)>| MarkerStyle {
        unsupported,
        styles,
    };
    match edge_kind {
        "specializes" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-specializes")),
                ("stroke-width", "1.7px"),
            ],
        ),
        "subsetting" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-arrow-open")),
                ("stroke-dasharray", "6,3"),
            ],
        ),
        "typing" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-arrow-open")),
                ("stroke-dasharray", "5,3"),
            ],
        ),
        "hierarchy" => ms(
            false,
            vec![
                ("marker-start", marker_url("general-d3-diamond")),
                ("marker-end", "none"),
            ],
        ),
        "bind" => ms(
            false,
            vec![("stroke-dasharray", "2,2"), ("marker-end", "none")],
        ),
        "allocate" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-arrow")),
                ("stroke-dasharray", "8,4"),
            ],
        ),
        "dependency" | "usage" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-arrow-open")),
                ("stroke-dasharray", "4,4"),
            ],
        ),
        "redefinition" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-specializes")),
                ("stroke-dasharray", "5,3"),
            ],
        ),
        "composition" => ms(
            false,
            vec![
                ("marker-start", marker_url("general-d3-diamond")),
                ("marker-end", "none"),
                ("stroke-dasharray", "6,3"),
            ],
        ),
        "connection" => ms(
            false,
            vec![("marker-start", "none"), ("marker-end", "none")],
        ),
        "satisfy" | "verify" | "derivation" => ms(
            false,
            vec![
                ("marker-end", marker_url("general-d3-arrow-open")),
                ("stroke-dasharray", "7,4"),
            ],
        ),
        _ => ms(
            true,
            vec![
                ("stroke-dasharray", "2,4"),
                ("marker-start", "none"),
                ("marker-end", "none"),
            ],
        ),
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
    let relation_type = attr_text(&edge.attributes, "relationType");
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
        edge_id: String,
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
        } else {
            // Every named branch of `applyEdgeMarker` also redundantly re-sets `stroke` as a
            // plain attribute (the inline `style` above already carries it) -- harmless in CSS,
            // but it is real attribute content a byte-level comparison against real TS/D3 output
            // catches, so it is reproduced rather than "simplified" away.
            path_el = path_el.attr("stroke", stroke);
        }
        for (name, value) in marker.styles {
            path_el = path_el.style(name, value);
        }
        edge_layer = edge_layer.child(path_el);

        // Port of `installDiagramTooltips`' static bake-in step: for every `[data-tooltip-kind]`
        // element it sets `aria-label` (newlines joined with "; ") and appends a `<title>` with
        // the same text unescaped. The hover/positioning machinery in that file is interaction
        // -only and never reaches serialized SVG text.
        let tooltip_text = tooltip_fallback_text(&edge_tooltip_descriptor(edge, nodes_by_id));
        edge_layer = edge_layer.child(
            Element::new("path")
                .attr("class", "viz-edge-hit-target")
                .attr("data-tooltip-kind", "edge")
                .attr("data-tooltip-id", edge.id.clone())
                .attr("d", path)
                .attr("aria-label", tooltip_text.replace('\n', "; "))
                .style("fill", "none")
                .style("stroke", "transparent")
                .style("stroke-width", "12px")
                .style("pointer-events", "stroke")
                .child(Element::new("title").text(tooltip_text)),
        );

        if !display_label.is_empty() {
            labels.push(PendingLabel {
                edge_id: edge.id.clone(),
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
                .attr("data-connector-id", label.edge_id.clone())
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
