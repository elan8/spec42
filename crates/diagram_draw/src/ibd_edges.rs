//! Port of the Interconnection-View branches of `drawEdges`/`applyEdgeMarker`/`pathForIbdEdge`/
//! `interconnectionEdgeLabelAnchor` in `render/drawing.ts` (the General-View branches are already
//! ported in `edges.rs`).

use std::collections::{BTreeSet, HashMap};

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

fn apply_edge_marker_ibd(edge_kind: &str, theme: &Theme, typed_projection: bool) -> MarkerStyle {
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
        "connection" if typed_projection => MarkerStyle {
            unsupported: false,
            attrs: vec![("stroke", stroke), ("stroke-width", "2")],
            styles: vec![],
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

#[derive(Clone, Copy)]
struct LabelBox {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

impl LabelBox {
    fn intersects(self, other: Self, margin: f64) -> bool {
        self.left < other.right + margin
            && self.right + margin > other.left
            && self.top < other.bottom + margin
            && self.bottom + margin > other.top
    }
}

/// Place an authored connector name only in a clear corridor. If every candidate collides,
/// the name remains available through the connector tooltip rather than obscuring the drawing.
fn typed_connector_label_anchor(
    route: &[Point],
    text: &str,
    all_routes: &[Vec<Point>],
    nodes_by_id: &HashMap<&str, &LaidOutNode>,
    placed: &[LabelBox],
) -> Option<(LabelAnchor, LabelBox)> {
    let width = truncate(text, 18).encode_utf16().count() as f64 * 6.5 + 10.0;
    let height = 16.0;
    let mut segments: Vec<(usize, Point, Point, f64, bool)> = route
        .windows(2)
        .enumerate()
        .map(|(index, pair)| {
            let horizontal = (pair[0].y - pair[1].y).abs() < 1.0;
            (
                index,
                pair[0],
                pair[1],
                (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y),
                horizontal,
            )
        })
        .collect();
    segments.sort_by(|left, right| {
        right
            .4
            .cmp(&left.4)
            .then(right.3.total_cmp(&left.3))
            .then(left.0.cmp(&right.0))
    });
    for (_, start, end, length, horizontal) in segments {
        if length
            < if horizontal {
                width + 12.0
            } else {
                height + 12.0
            }
        {
            continue;
        }
        for fraction in [0.5, 0.25, 0.75] {
            let x = start.x + (end.x - start.x) * fraction;
            let y = start.y + (end.y - start.y) * fraction;
            for side in [-1.0, 1.0] {
                let bounds = if horizontal {
                    let top = if side < 0.0 {
                        y - height - 10.0
                    } else {
                        y + 10.0
                    };
                    LabelBox {
                        left: x - width / 2.0,
                        top,
                        right: x + width / 2.0,
                        bottom: top + height,
                    }
                } else {
                    let left = if side < 0.0 {
                        x - width - 10.0
                    } else {
                        x + 10.0
                    };
                    LabelBox {
                        left,
                        top: y - height / 2.0,
                        right: left + width,
                        bottom: y + height / 2.0,
                    }
                };
                if horizontal
                    && (bounds.left < start.x.min(end.x) || bounds.right > start.x.max(end.x))
                {
                    continue;
                }
                if !horizontal
                    && (bounds.top < start.y.min(end.y) || bounds.bottom > start.y.max(end.y))
                {
                    continue;
                }
                let hits_node = nodes_by_id.values().any(|node| {
                    if node.attributes.get("isSyntheticContainer")
                        == Some(&serde_json::Value::Bool(true))
                        || node.attributes.get("isSyntheticPackage")
                            == Some(&serde_json::Value::Bool(true))
                    {
                        return false;
                    }
                    bounds.intersects(
                        LabelBox {
                            left: node.x,
                            top: node.y,
                            right: node.x + node.width,
                            bottom: node.y + node.height,
                        },
                        14.0,
                    )
                });
                let hits_route =
                    all_routes
                        .iter()
                        .flat_map(|points| points.windows(2))
                        .any(|pair| {
                            bounds.intersects(
                                LabelBox {
                                    left: pair[0].x.min(pair[1].x),
                                    top: pair[0].y.min(pair[1].y),
                                    right: pair[0].x.max(pair[1].x),
                                    bottom: pair[0].y.max(pair[1].y),
                                },
                                3.0,
                            )
                        });
                if hits_node
                    || hits_route
                    || placed.iter().any(|other| bounds.intersects(*other, 5.0))
                {
                    continue;
                }
                let anchor = LabelAnchor {
                    x: if horizontal { x } else { bounds.left },
                    y: bounds.top + 12.0,
                    text_anchor: if horizontal { "middle" } else { "start" },
                    dy: "0",
                };
                return Some((anchor, bounds));
            }
        }
    }
    None
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
    // TS's stable descending sort keeps the first (lowest-index) segment on a length tie, whereas
    // `Iterator::max_by` returns the *last* maximum on ties -- break ties on `Reverse(index)` so
    // the earliest segment wins, matching the stable sort exactly.
    fn longest<'a>(candidates: &[(usize, &'a Segment)]) -> Option<&'a Segment> {
        candidates
            .iter()
            .max_by(|(ia, a), (ib, b)| a.length.total_cmp(&b.length).then(ib.cmp(ia)))
            .map(|(_, s)| *s)
    }
    let indexed: Vec<(usize, &Segment)> = segments.iter().enumerate().collect();
    let longest_horizontal = longest(
        &indexed
            .iter()
            .copied()
            .filter(|(_, s)| s.horizontal && s.length >= 24.0)
            .collect::<Vec<_>>(),
    );
    let segment = longest_horizontal.or_else(|| longest(&indexed))?;
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
    if matches!(
        edge.attributes.get("typedProjection"),
        Some(serde_json::Value::Bool(true))
    ) && label.eq_ignore_ascii_case("connector")
    {
        return String::new();
    }
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

/// JS `String.length`/`.slice()` count UTF-16 code units, not Unicode scalar values -- use
/// `encode_utf16` here (matching `behavior_common::truncate_label`), not `.chars()`, so labels
/// containing non-BMP characters (e.g. emoji) truncate at the same code-unit boundary as the
/// original TS `truncate` in `drawing.ts`.
fn truncate(value: &str, max: usize) -> String {
    let units: Vec<u16> = value.encode_utf16().collect();
    if units.len() > max {
        let truncated = String::from_utf16_lossy(&units[..max.saturating_sub(1)]);
        format!("{truncated}...")
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
    let all_routes: Vec<Vec<Point>> = edges
        .iter()
        .map(|edge| route_points_for_edge(edge, &layout_edges_by_id))
        .collect();
    let port_types: Vec<String> = edges
        .iter()
        .map(|edge| attr_text(&edge.attributes, "portTypeIdentity"))
        .filter(|identity: &String| !identity.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut typed_labels = HashMap::new();
    let mut placed_labels = Vec::new();
    for (edge, route) in edges.iter().zip(&all_routes) {
        if edge.attributes.get("typedProjection") != Some(&serde_json::Value::Bool(true)) {
            continue;
        }
        let kind = edge
            .edge_kind
            .clone()
            .unwrap_or_else(|| normalize_edge_kind(&edge.label));
        let label = ibd_edge_display_label(edge, &kind);
        if label.is_empty() {
            continue;
        }
        if let Some((anchor, bounds)) =
            typed_connector_label_anchor(route, &label, &all_routes, nodes_by_id, &placed_labels)
        {
            typed_labels.insert(edge.id.as_str(), anchor);
            placed_labels.push(bounds);
        }
    }

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
        let typed_projection =
            edge.attributes.get("typedProjection") == Some(&serde_json::Value::Bool(true));
        let type_identity = attr_text(&edge.attributes, "portTypeIdentity");
        let palette_index = port_types.binary_search(&type_identity).ok();
        let palette: &[&str; 6] = if theme.color_scheme == "dark" {
            &[
                "#60a5fa", "#fbbf24", "#5eead4", "#c084fc", "#fb7185", "#a3e635",
            ]
        } else {
            &[
                "#1d4ed8", "#b45309", "#0f766e", "#7e22ce", "#be123c", "#4d7c0f",
            ]
        };
        let stroke = if typed_projection && edge_kind == "connection" {
            palette_index.map_or(stroke_color_for_edge(theme), |index| {
                palette[index % palette.len()]
            })
        } else {
            stroke_color_for_edge(theme)
        };
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
        if typed_projection && !type_identity.is_empty() {
            path_el = path_el.attr("data-port-type", type_identity);
        }
        if typed_projection && !display_label.is_empty() {
            path_el = path_el.attr(
                "data-label-placement",
                if typed_labels.contains_key(edge.id.as_str()) {
                    "visible"
                } else {
                    "tooltip-only"
                },
            );
        }

        let marker = apply_edge_marker_ibd(&edge_kind, theme, typed_projection);
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
            if typed_projection {
                if let Some(anchor) = typed_labels.remove(edge.id.as_str()) {
                    labels.push(PendingLabel {
                        edge_id: edge.id.clone(),
                        edge_kind,
                        display_label,
                        anchor,
                    });
                }
            } else if let Some(anchor) =
                interconnection_edge_label_anchor(edge, &layout_edges_by_id)
            {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn edge_with_layout_route_points(
        points: Vec<Point>,
    ) -> (LaidOutEdge, InterconnectionLayoutEdgeDto) {
        let edge = LaidOutEdge {
            id: "e".to_string(),
            source: "a".to_string(),
            target: "b".to_string(),
            label: String::new(),
            edge_kind: None,
            attributes: BTreeMap::new(),
            layout: None,
        };
        let layout_edge = InterconnectionLayoutEdgeDto {
            id: "e".to_string(),
            route_points: points,
        };
        (edge, layout_edge)
    }

    /// TS's stable descending sort keeps the *first* segment on a length tie; `Iterator::max_by`
    /// alone would keep the last. A symmetric zig-zag route with two equal-length horizontal
    /// segments must anchor the label on the earlier one.
    #[test]
    fn label_anchor_prefers_the_earliest_segment_on_a_length_tie() {
        let (edge, layout_edge) = edge_with_layout_route_points(vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 40.0, y: 0.0 },
            Point { x: 40.0, y: 40.0 },
            Point { x: 80.0, y: 40.0 },
        ]);
        let mut layout_edges_by_id = HashMap::new();
        layout_edges_by_id.insert(edge.id.as_str(), &layout_edge);

        let anchor = interconnection_edge_label_anchor(&edge, &layout_edges_by_id)
            .expect("two-plus route points always resolve an anchor");
        // Midpoint of the *first* horizontal segment (0,0)-(40,0), not the second (40,40)-(80,40).
        assert_eq!(anchor.x, 20.0);
        assert_eq!(anchor.y, 0.0);
    }

    /// JS `String.length`/`.slice()` count UTF-16 code units, not Unicode scalar values. Each of
    /// these two emoji is one `char` but a *surrogate pair* (2 code units) in UTF-16, so a
    /// `.chars()`-based length check (2 <= max) would wrongly skip truncation entirely, while the
    /// UTF-16-code-unit check (4 > max) correctly truncates, matching the original TS behavior.
    #[test]
    fn truncate_counts_utf16_code_units_like_the_original() {
        let text = "🙂🙂";
        let truncated = truncate(text, 3);
        assert_eq!(truncated, "🙂...");
    }

    #[test]
    fn typed_connector_label_avoids_parallel_routes() {
        let route = vec![Point { x: 0.0, y: 0.0 }, Point { x: 200.0, y: 0.0 }];
        let upper = vec![Point { x: 0.0, y: -15.0 }, Point { x: 200.0, y: -15.0 }];
        let nodes = HashMap::new();
        let (anchor, bounds) = typed_connector_label_anchor(
            &route,
            "powerLink",
            &[route.clone(), upper.clone()],
            &nodes,
            &[],
        )
        .expect("lower corridor remains clear");
        assert!(anchor.y > 0.0);
        assert!(bounds.top > 0.0);

        let lower = vec![Point { x: 0.0, y: 15.0 }, Point { x: 200.0, y: 15.0 }];
        assert!(typed_connector_label_anchor(
            &route,
            "powerLink",
            &[route.clone(), upper, lower],
            &nodes,
            &[],
        )
        .is_none());
    }

    #[test]
    fn typed_connector_label_avoids_part_box() {
        let route = vec![Point { x: 0.0, y: 0.0 }, Point { x: 200.0, y: 0.0 }];
        let part = LaidOutNode {
            id: "part".into(),
            label: "part".into(),
            kind: "part".into(),
            attributes: BTreeMap::new(),
            x: 40.0,
            y: -40.0,
            width: 120.0,
            height: 30.0,
            compartments: None,
        };
        let mut nodes = HashMap::new();
        nodes.insert(part.id.as_str(), &part);
        let (_, bounds) = typed_connector_label_anchor(
            &route,
            "videoLink",
            std::slice::from_ref(&route),
            &nodes,
            &[],
        )
        .expect("lower corridor remains clear");
        assert!(bounds.top > 0.0);
    }

    #[test]
    fn typed_connector_labels_do_not_stack_on_each_other() {
        let route = vec![Point { x: 0.0, y: 0.0 }, Point { x: 200.0, y: 0.0 }];
        let nodes = HashMap::new();
        let (_, first) = typed_connector_label_anchor(
            &route,
            "firstConnector",
            std::slice::from_ref(&route),
            &nodes,
            &[],
        )
        .unwrap();
        let (_, second) = typed_connector_label_anchor(
            &route,
            "secondConnector",
            std::slice::from_ref(&route),
            &nodes,
            &[first],
        )
        .unwrap();
        assert!(!first.intersects(second, 5.0));
    }
}
