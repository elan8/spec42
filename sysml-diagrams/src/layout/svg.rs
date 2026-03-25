use crate::layout::{
    Bounds, DiagramLayout, HitRegion, HitRegionKind, Point, PortSide, RenderedSvg, SvgRenderOptions,
};

pub(crate) fn render_svg(layout: &DiagramLayout, options: &SvgRenderOptions) -> RenderedSvg {
    let mut svg = String::new();
    let mut hit_regions = Vec::new();
    let mut node_groups = String::new();
    let mut edge_group = String::new();
    let is_interconnection_view = options.class_name.contains("interconnection-view");
    let is_general_view = options.class_name.contains("general-view");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\" viewBox=\"0 0 {:.0} {:.0}\" class=\"{}\">",
        layout.width,
        layout.height,
        layout.width,
        layout.height,
        escape_text(&options.class_name)
    ));
    svg.push_str("<style>:root{color-scheme:light dark}.diagram-root{--diagram-paper:var(--vscode-editor-background,#1e1e1e);--diagram-ink:var(--vscode-editor-foreground,#d4d4d4);--diagram-muted:#a0a0a0;--diagram-faint:#6b6b6b;--diagram-border:var(--diagram-ink);--diagram-edge:var(--diagram-ink);--diagram-edge-label:var(--diagram-edge);--diagram-port-stroke:var(--diagram-edge);--diagram-port-label:var(--diagram-ink);}.interconnection-view{--diagram-border:#8f99a3;--diagram-edge:#6fd3ff;--diagram-edge-label:#8eddff;--diagram-port-stroke:var(--diagram-edge);--diagram-port-label:#dbefff;}.diagram-viewport{font-family:var(--vscode-editor-font-family, sans-serif)}.diagram-edge{fill:none;stroke:var(--diagram-edge);opacity:1}.diagram-edge-label{font-size:10px;font-weight:400;fill:var(--diagram-edge-label);paint-order:stroke;stroke:var(--diagram-paper);stroke-width:3px;stroke-linejoin:round}.diagram-port{fill:var(--diagram-paper);stroke:var(--diagram-port-stroke);stroke-width:1.2px}.diagram-port-label{font-size:10px;fill:var(--diagram-port-label)}.diagram-stereotype{font-size:9px;fill:var(--diagram-muted)}.diagram-title{fill:var(--diagram-ink);font-size:11px;font-weight:700}.diagram-typed-by{font-size:10px;font-style:italic;fill:var(--diagram-ink)}.diagram-detail{font-size:10px;fill:var(--diagram-ink)}.diagram-compartment-title{font-size:9px;font-weight:700;fill:var(--diagram-muted)}.diagram-divider{stroke:var(--diagram-faint);stroke-width:1px;opacity:.8}.node-background{fill:var(--diagram-paper);stroke:var(--diagram-border)}@media print {.diagram-root{--diagram-paper:#ffffff;--diagram-ink:#000000;--diagram-muted:#333333;--diagram-faint:#b5b5b5;--diagram-border:#5f5f5f;--diagram-edge:#000000;--diagram-edge-label:#000000;--diagram-port-stroke:#000000;--diagram-port-label:#000000;}}</style>");
    svg.push_str("<defs>");
    svg.push_str("<marker id=\"edge-arrow-solid\" viewBox=\"0 -5 10 10\" refX=\"9\" refY=\"0\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\"><path d=\"M0,-4L10,0L0,4Z\" fill=\"context-stroke\"/></marker>");
    svg.push_str("<marker id=\"edge-arrow-open\" viewBox=\"0 -5 10 10\" refX=\"9\" refY=\"0\" markerWidth=\"8\" markerHeight=\"8\" orient=\"auto\"><path d=\"M0,-4L10,0L0,4\" fill=\"none\" stroke=\"context-stroke\" stroke-width=\"1.3\"/></marker>");
    svg.push_str("<marker id=\"edge-triangle-hollow\" viewBox=\"0 -6 12 12\" refX=\"11\" refY=\"0\" markerWidth=\"8\" markerHeight=\"8\" orient=\"auto\"><path d=\"M0,0L10,-4L10,4Z\" fill=\"var(--diagram-paper,#ffffff)\" stroke=\"context-stroke\" stroke-width=\"1.2\"/></marker>");
    svg.push_str("<marker id=\"edge-diamond-solid\" viewBox=\"0 -6 12 12\" refX=\"2\" refY=\"0\" markerWidth=\"9\" markerHeight=\"9\" orient=\"auto\"><path d=\"M2,0L6,-4L10,0L6,4Z\" fill=\"context-stroke\" stroke=\"context-stroke\" stroke-width=\"1\"/></marker>");
    svg.push_str("<marker id=\"edge-dot\" viewBox=\"0 0 10 10\" refX=\"5\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\"><circle cx=\"5\" cy=\"5\" r=\"2.4\" fill=\"context-stroke\"/></marker>");
    svg.push_str("</defs>");
    svg.push_str("<g class=\"diagram-viewport\">");

    let edge_jumps = if options.visual_edge_bridges {
        compute_horizontal_edge_jumps(layout)
    } else {
        vec![Vec::new(); layout.edges.len()]
    };

    for (edge_index, edge) in layout.edges.iter().enumerate() {
        let path = path_from_points_with_horizontal_jumps(&edge.points, &edge_jumps[edge_index]);
        let style = edge_style(&edge.kind);
        edge_group.push_str(&format!(
            "<path class=\"diagram-edge edge-{}\" data-element-id=\"{}\" d=\"{}\" style=\"stroke:{};stroke-width:{};stroke-dasharray:{}\" marker-start=\"{}\" marker-end=\"{}\"/>",
            escape_text(&normalize_kind(&edge.kind).replace(' ', "-")),
            escape_text(&edge.id),
            path,
            style.color,
            style.width,
            style.dash_array,
            style.marker_start,
            style.marker_end
        ));
        let label_text = edge_label_text(edge.label.as_deref(), &edge.kind);
        if let Some(label) = label_text {
            let bounds = label_bounds(&edge.points);
            edge_group.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-edge-label\" data-edge-label=\"{}\" style=\"fill:{}\">{}</text>",
                bounds.x,
                bounds.y,
                escape_text(&edge.id),
                style.color,
                escape_text(label)
            ));
            hit_regions.push(HitRegion {
                id: format!("{}:label", edge.id),
                kind: HitRegionKind::EdgeLabel,
                element_id: edge.id.clone(),
                qualified_name: None,
                bounds: Bounds {
                    x: bounds.x - 4.0,
                    y: bounds.y - 12.0,
                    width: 80.0,
                    height: 16.0,
                },
            });
        }
    }

    for node in &layout.nodes {
        let kind = normalize_kind(&node.kind);
        if kind.starts_with("layout-") {
            continue;
        }
        let is_root = node.parent_id.is_none();
        let is_interconnection_root = is_interconnection_view && is_root;

        let is_definition = kind.contains("def");
        let corner_radius = if kind.contains("requirement") {
            16.0
        } else if kind.contains("action") || kind.contains("state") || kind.contains("calc") {
            12.0
        } else if is_definition {
            4.0
        } else {
            8.0
        };
        let stereotype = format_stereotype(&kind);
        let typed_by = typed_by_line(&node.detail_lines);
        let compartments = build_compartments(&node.detail_lines);
        let header_height = 44.0 + if typed_by.is_some() { 14.0 } else { 0.0 };
        let title_y = node.bounds.y + 31.0;
        node_groups.push_str(&format!(
            "<g class=\"diagram-node {}\" data-element-id=\"{}\" data-element-name=\"{}\" data-qualified-name=\"{}\">",
            escape_text(&node.kind.replace(' ', "-")),
            escape_text(&node.id),
            escape_text(&node.label),
            escape_text(&node.id)
        ));

        if is_interconnection_root {
            let bar_height = 40.0;
            // Draw a full container (root) with a header bar.
            node_groups.push_str(&format!(
                "<rect class=\"node-background\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"4.0\" ry=\"4.0\" style=\"fill:var(--diagram-paper);stroke:var(--diagram-border);stroke-width:1.6px;stroke-dasharray:none\"/>",
                node.bounds.x,
                node.bounds.y,
                node.bounds.width,
                node.bounds.height
            ));
            node_groups.push_str(&format!(
                "<rect class=\"node-background\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"4.0\" ry=\"4.0\" style=\"fill:var(--diagram-paper);stroke:var(--diagram-border);stroke-width:1.6px;stroke-dasharray:none\"/>",
                node.bounds.x,
                node.bounds.y,
                node.bounds.width,
                bar_height
            ));
            node_groups.push_str(&format!(
                "<line class=\"diagram-divider\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" style=\"stroke:var(--diagram-faint)\"/>",
                node.bounds.x + 6.0,
                node.bounds.y + bar_height,
                node.bounds.right() - 6.0,
                node.bounds.y + bar_height
            ));
            node_groups.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-stereotype\" text-anchor=\"middle\">{}</text>",
                node.bounds.x + node.bounds.width / 2.0,
                node.bounds.y + 17.0,
                stereotype
            ));
            node_groups.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-title\" text-anchor=\"middle\">{}</text>",
                node.bounds.x + node.bounds.width / 2.0,
                node.bounds.y + 33.0,
                escape_text(&node.label)
            ));
        } else {
            node_groups.push_str(&format!(
                "<rect class=\"node-background\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" style=\"fill:{};stroke:{};stroke-width:{};stroke-dasharray:{}\"/>",
                node.bounds.x,
                node.bounds.y,
                node.bounds.width,
                node.bounds.height,
                corner_radius,
                corner_radius,
                "var(--diagram-paper)",
                "var(--diagram-border)",
                "1.6px",
                "none"
            ));
            node_groups.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"39\" rx=\"{:.1}\" ry=\"{:.1}\" style=\"fill:var(--diagram-paper);stroke:none\"/>",
                node.bounds.x,
                node.bounds.y + 1.0,
                node.bounds.width,
                (corner_radius - 2.0_f32).max(2.0_f32),
                (corner_radius - 2.0_f32).max(2.0_f32)
            ));
            node_groups.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-stereotype\" text-anchor=\"middle\">{}</text>",
                node.bounds.x + node.bounds.width / 2.0,
                node.bounds.y + 17.0,
                stereotype
            ));
            node_groups.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-title\" text-anchor=\"middle\">{}</text>",
                node.bounds.x + node.bounds.width / 2.0,
                title_y,
                escape_text(&node.label)
            ));
        }

        if !is_interconnection_root {
            if let Some(typed_by) = typed_by.as_ref() {
                node_groups.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-typed-by\" text-anchor=\"middle\">: {}</text>",
                    node.bounds.x + node.bounds.width / 2.0,
                    node.bounds.y + 43.0,
                    escape_text(typed_by)
                ));
            }
            let mut content_y = node.bounds.y + header_height;
            for compartment in &compartments {
                node_groups.push_str(&format!(
                    "<line class=\"diagram-divider\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" style=\"stroke:{}\"/>",
                    node.bounds.x + 6.0,
                    content_y,
                    node.bounds.right() - 6.0,
                    content_y,
                    "var(--diagram-border)"
                ));
                node_groups.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-compartment-title\">{}</text>",
                    node.bounds.x + 12.0,
                    content_y + 13.0,
                    escape_text(compartment.title)
                ));
                content_y += 18.0;
                let visible_lines = compartment.lines.iter().take(6).collect::<Vec<_>>();
                for line in visible_lines {
                    node_groups.push_str(&format!(
                        "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-detail\">{}</text>",
                        node.bounds.x + 12.0,
                        content_y + 10.0,
                        escape_text(line)
                    ));
                    content_y += 14.0;
                }
                if compartment.lines.len() > 6 {
                    node_groups.push_str(&format!(
                        "<text x=\"{:.1}\" y=\"{:.1}\" class=\"diagram-detail\">...</text>",
                        node.bounds.x + 12.0,
                        content_y + 10.0
                    ));
                    content_y += 14.0;
                }
                content_y += 8.0;
            }
            if kind.contains("actor") {
                node_groups.push_str(&actor_glyph(
                    node.bounds.x + 22.0,
                    node.bounds.y + 30.0,
                    "var(--diagram-border)",
                ));
            }
            if kind.contains("requirement") {
                node_groups.push_str(&format!(
                    "<path d=\"M {:.1} {:.1} l 12 0 l 6 6 l 0 12 l -6 6 l -12 0 l -6 -6 l 0 -12 z\" style=\"fill:none;stroke:var(--diagram-border);stroke-width:1.2px;opacity:.85\"/>",
                    node.bounds.right() - 24.0,
                    node.bounds.y + 12.0
                ));
            }
        }
        if !is_general_view {
            for port in &node.ports {
                node_groups.push_str(&format!(
                    "<rect class=\"diagram-port\" x=\"{:.1}\" y=\"{:.1}\" width=\"8\" height=\"8\" data-port-id=\"{}\"/>",
                    port.position.x - 4.0,
                    port.position.y - 4.0,
                    escape_text(&port.id)
                ));
                let (label_x, anchor) = match port.side {
                    PortSide::Left => (port.position.x + 8.0, "start"),
                    PortSide::Right => (port.position.x - 8.0, "end"),
                    PortSide::Top | PortSide::Bottom => (port.position.x, "middle"),
                };
                let label_y = match port.side {
                    PortSide::Top => port.position.y + 14.0,
                    PortSide::Bottom => port.position.y - 6.0,
                    PortSide::Left | PortSide::Right => port.position.y + 4.0,
                };
                node_groups.push_str(&format!(
                    "<text class=\"diagram-port-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\">{}</text>",
                    label_x,
                    label_y,
                    anchor,
                    escape_text(&port.name)
                ));
                hit_regions.push(HitRegion {
                    id: port.id.clone(),
                    kind: HitRegionKind::Port,
                    element_id: port.node_id.clone(),
                    qualified_name: Some(format!("{}::{}", port.node_id, port.id)),
                    bounds: Bounds {
                        x: port.position.x - 6.0,
                        y: port.position.y - 6.0,
                        width: 12.0,
                        height: 12.0,
                    },
                });
            }
        }
        node_groups.push_str("</g>");
        hit_regions.push(HitRegion {
            id: node.id.clone(),
            kind: HitRegionKind::Node,
            element_id: node.id.clone(),
            qualified_name: Some(node.id.clone()),
            bounds: node.bounds,
        });
    }

    svg.push_str(&node_groups);
    svg.push_str(&edge_group);
    svg.push_str("</g></svg>");
    RenderedSvg {
        svg,
        hit_regions,
        bounds: Bounds {
            x: 0.0,
            y: 0.0,
            width: layout.width,
            height: layout.height,
        },
    }
}

fn normalize_kind(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn format_stereotype(kind: &str) -> String {
    format!("&#171;{}&#187;", escape_text(kind))
}

fn filtered_detail_lines(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| !line.trim_start().starts_with("type: "))
        .cloned()
        .collect()
}

#[derive(Debug)]
struct Compartment {
    title: &'static str,
    lines: Vec<String>,
}

struct EdgeStyle {
    color: &'static str,
    dash_array: &'static str,
    marker_start: &'static str,
    marker_end: &'static str,
    width: &'static str,
}

fn typed_by_line(lines: &[String]) -> Option<String> {
    lines.iter().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        let normalized = key.trim().to_ascii_lowercase();
        if matches!(
            normalized.as_str(),
            "parttype"
                | "porttype"
                | "interfacetype"
                | "statetype"
                | "actiontype"
                | "usecasetype"
                | "actortype"
        ) {
            Some(value.trim().trim_matches('"').to_string())
        } else {
            None
        }
    })
}

fn build_compartments(lines: &[String]) -> Vec<Compartment> {
    let mut attributes = Vec::new();
    let mut ports = Vec::new();
    let mut nested = Vec::new();
    let mut properties = Vec::new();

    for line in filtered_detail_lines(lines) {
        let trimmed = line.trim();
        let normalized = trimmed.to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }
        if normalized.starts_with("parttype:")
            || normalized.starts_with("porttype:")
            || normalized.starts_with("interfacetype:")
            || normalized.starts_with("statetype:")
            || normalized.starts_with("actiontype:")
            || normalized.starts_with("usecasetype:")
            || normalized.starts_with("actortype:")
        {
            continue;
        }
        if normalized.contains("port") {
            ports.push(trimmed.to_string());
        } else if normalized.contains("part")
            || normalized.contains("state")
            || normalized.contains("action")
            || normalized.contains("requirement")
            || normalized.contains("interface")
        {
            nested.push(trimmed.to_string());
        } else if normalized.contains("attribute") {
            attributes.push(trimmed.to_string());
        } else {
            properties.push(trimmed.to_string());
        }
    }

    let mut compartments = Vec::new();
    if !attributes.is_empty() {
        compartments.push(Compartment {
            title: "Attributes",
            lines: attributes,
        });
    }
    if !properties.is_empty() {
        compartments.push(Compartment {
            title: "Properties",
            lines: properties,
        });
    }
    if !nested.is_empty() {
        compartments.push(Compartment {
            title: "Parts",
            lines: nested,
        });
    }
    if !ports.is_empty() {
        compartments.push(Compartment {
            title: "Ports",
            lines: ports,
        });
    }
    compartments
}

fn edge_style(kind: &str) -> EdgeStyle {
    let kind = normalize_kind(kind);
    match kind.as_str() {
        "specializes" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "none",
            marker_start: "none",
            marker_end: "url(#edge-triangle-hollow)",
            width: "1.7px",
        },
        "typing" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "5,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-open)",
            width: "1.4px",
        },
        "layout-hint" => EdgeStyle {
            color: "transparent",
            dash_array: "none",
            marker_start: "none",
            marker_end: "none",
            width: "0px",
        },
        "hierarchy" | "contains" | "containment" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "none",
            marker_start: "url(#edge-diamond-solid)",
            marker_end: "none",
            width: "1.4px",
        },
        "connection" | "connect" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "none",
            marker_start: "none",
            marker_end: "none",
            width: "1.4px",
        },
        "bind" | "binding" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "6,3",
            marker_start: "none",
            marker_end: "none",
            width: "1.2px",
        },
        "perform" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "6,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-open)",
            width: "1.4px",
        },
        "allocate" | "allocation" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "10,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        "satisfy" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "9,4,2,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        "verify" => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "4,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        _ => EdgeStyle {
            color: "var(--diagram-edge)",
            dash_array: "none",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
    }
}

fn is_structural_guide_edge(kind: &str) -> bool {
    matches!(
        normalize_kind(kind).as_str(),
        "hierarchy" | "contains" | "containment" | "layout-hint"
    )
}

fn edge_label_text<'a>(explicit_label: Option<&'a str>, kind: &'a str) -> Option<&'a str> {
    let normalized_kind = normalize_kind(kind);
    let normalized_label = explicit_label
        .map(str::trim)
        .map(|label| label.to_ascii_lowercase());
    if is_structural_guide_edge(kind) {
        return None;
    }
    if normalized_label.as_deref() == Some(normalized_kind.as_str()) {
        return None;
    }
    explicit_label.filter(|label| !label.trim().is_empty())
}

fn actor_glyph(x: f32, y: f32, stroke: &str) -> String {
    format!(
        "<g transform=\"translate({:.1},{:.1})\" style=\"stroke:{};stroke-width:1.8px;fill:none;opacity:.9\"><circle cx=\"0\" cy=\"-6\" r=\"5\"/><line x1=\"0\" y1=\"-1\" x2=\"0\" y2=\"13\"/><line x1=\"-9\" y1=\"3\" x2=\"9\" y2=\"3\"/><line x1=\"0\" y1=\"13\" x2=\"-8\" y2=\"24\"/><line x1=\"0\" y1=\"13\" x2=\"8\" y2=\"24\"/></g>",
        x, y, stroke
    )
}

fn path_from_points(points: &[Point]) -> String {
    let mut out = String::new();
    for (index, point) in points.iter().enumerate() {
        if index == 0 {
            out.push_str(&format!("M {:.1} {:.1}", point.x, point.y));
        } else {
            out.push_str(&format!(" L {:.1} {:.1}", point.x, point.y));
        }
    }
    out
}

#[derive(Clone, Copy, Debug)]
struct HorizontalJump {
    segment_index: usize,
    x: f32,
}

fn path_from_points_with_horizontal_jumps(points: &[Point], jumps: &[HorizontalJump]) -> String {
    if jumps.is_empty() {
        return path_from_points(points);
    }

    let mut out = String::new();
    if let Some(first) = points.first() {
        out.push_str(&format!("M {:.1} {:.1}", first.x, first.y));
    }

    for (segment_index, segment) in points.windows(2).enumerate() {
        let start = segment[0];
        let end = segment[1];
        if (start.y - end.y).abs() > 0.1 {
            out.push_str(&format!(" L {:.1} {:.1}", end.x, end.y));
            continue;
        }

        let mut segment_jumps = jumps
            .iter()
            .filter(|jump| jump.segment_index == segment_index)
            .map(|jump| jump.x)
            .collect::<Vec<_>>();
        if segment_jumps.is_empty() {
            out.push_str(&format!(" L {:.1} {:.1}", end.x, end.y));
            continue;
        }

        let left_to_right = end.x >= start.x;
        segment_jumps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if !left_to_right {
            segment_jumps.reverse();
        }

        let mut current_x = start.x;
        let segment_length = (end.x - start.x).abs();
        let jump_half_width = 7.0f32.min(segment_length / 3.0);
        let jump_height = 7.0f32;
        let mut previous_jump_x: Option<f32> = None;

        for jump_x in segment_jumps {
            if previous_jump_x
                .is_some_and(|previous| (jump_x - previous).abs() < jump_half_width * 2.5)
            {
                continue;
            }

            let before_x = if left_to_right {
                (jump_x - jump_half_width).max(current_x)
            } else {
                (jump_x + jump_half_width).min(current_x)
            };
            let after_x = if left_to_right {
                (jump_x + jump_half_width).min(end.x)
            } else {
                (jump_x - jump_half_width).max(end.x)
            };
            if (before_x - current_x).abs() < 0.1 || (after_x - before_x).abs() < 0.1 {
                continue;
            }

            out.push_str(&format!(" L {:.1} {:.1}", before_x, start.y));
            if left_to_right {
                out.push_str(&format!(
                    " Q {:.1} {:.1}, {:.1} {:.1} Q {:.1} {:.1}, {:.1} {:.1}",
                    jump_x - jump_half_width * 0.5,
                    start.y,
                    jump_x,
                    start.y - jump_height,
                    jump_x + jump_half_width * 0.5,
                    start.y,
                    after_x,
                    start.y
                ));
            } else {
                out.push_str(&format!(
                    " Q {:.1} {:.1}, {:.1} {:.1} Q {:.1} {:.1}, {:.1} {:.1}",
                    jump_x + jump_half_width * 0.5,
                    start.y,
                    jump_x,
                    start.y - jump_height,
                    jump_x - jump_half_width * 0.5,
                    start.y,
                    after_x,
                    start.y
                ));
            }
            current_x = after_x;
            previous_jump_x = Some(jump_x);
        }

        if (current_x - end.x).abs() > 0.1 {
            out.push_str(&format!(" L {:.1} {:.1}", end.x, end.y));
        }
    }

    out
}

fn compute_horizontal_edge_jumps(layout: &DiagramLayout) -> Vec<Vec<HorizontalJump>> {
    let mut jumps = vec![Vec::new(); layout.edges.len()];
    for (left_index, left_edge) in layout.edges.iter().enumerate() {
        for (right_offset, right_edge) in layout.edges.iter().skip(left_index + 1).enumerate() {
            let right_index = left_index + 1 + right_offset;
            for (left_segment_index, left_segment) in left_edge.points.windows(2).enumerate() {
                for (right_segment_index, right_segment) in right_edge.points.windows(2).enumerate() {
                    if let Some((horizontal_owner, crossing_x)) = horizontal_crossing(
                        left_segment[0],
                        left_segment[1],
                        right_segment[0],
                        right_segment[1],
                    ) {
                        let (target_edge_index, target_segment_index) = if horizontal_owner == 0 {
                            (left_index, left_segment_index)
                        } else {
                            (right_index, right_segment_index)
                        };
                        jumps[target_edge_index].push(HorizontalJump {
                            segment_index: target_segment_index,
                            x: crossing_x,
                        });
                    }
                }
            }
        }
    }
    jumps
}

fn horizontal_crossing(
    left_start: Point,
    left_end: Point,
    right_start: Point,
    right_end: Point,
) -> Option<(usize, f32)> {
    let left_horizontal = (left_start.y - left_end.y).abs() <= 0.1;
    let left_vertical = (left_start.x - left_end.x).abs() <= 0.1;
    let right_horizontal = (right_start.y - right_end.y).abs() <= 0.1;
    let right_vertical = (right_start.x - right_end.x).abs() <= 0.1;

    let (horizontal_owner, horizontal_start, horizontal_end, vertical_start, vertical_end) =
        if left_horizontal && right_vertical {
            (0usize, left_start, left_end, right_start, right_end)
        } else if right_horizontal && left_vertical {
            (1usize, right_start, right_end, left_start, left_end)
        } else {
            return None;
        };

    let crossing_x = vertical_start.x;
    let crossing_y = horizontal_start.y;
    let horizontal_min_x = horizontal_start.x.min(horizontal_end.x);
    let horizontal_max_x = horizontal_start.x.max(horizontal_end.x);
    let vertical_min_y = vertical_start.y.min(vertical_end.y);
    let vertical_max_y = vertical_start.y.max(vertical_end.y);
    const ENDPOINT_MARGIN: f32 = 3.0;

    if crossing_x <= horizontal_min_x + ENDPOINT_MARGIN
        || crossing_x >= horizontal_max_x - ENDPOINT_MARGIN
        || crossing_y <= vertical_min_y + ENDPOINT_MARGIN
        || crossing_y >= vertical_max_y - ENDPOINT_MARGIN
    {
        return None;
    }

    if point_matches(crossing_x, crossing_y, left_start)
        || point_matches(crossing_x, crossing_y, left_end)
        || point_matches(crossing_x, crossing_y, right_start)
        || point_matches(crossing_x, crossing_y, right_end)
    {
        return None;
    }

    Some((horizontal_owner, crossing_x))
}

fn point_matches(x: f32, y: f32, point: Point) -> bool {
    (x - point.x).abs() <= 0.1 && (y - point.y).abs() <= 0.1
}

pub(crate) fn label_bounds(points: &[Point]) -> Point {
    if points.len() < 2 {
        return Point { x: 0.0, y: 0.0 };
    }
    let mid = points.len() / 2;
    let anchor = points[mid];
    Point {
        x: anchor.x + 6.0,
        y: anchor.y - 6.0,
    }
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::{horizontal_crossing, Point};

    #[test]
    fn horizontal_crossing_detects_near_endpoint_crossings_that_are_not_junctions() {
        let crossing = horizontal_crossing(
            Point { x: 936.0, y: 1636.2 },
            Point { x: 1744.0, y: 1636.2 },
            Point { x: 1597.5, y: 1508.2 },
            Point { x: 1597.5, y: 1892.2 },
        );

        assert_eq!(crossing, Some((0, 1597.5)));
    }

    #[test]
    fn horizontal_crossing_ignores_true_segment_junctions() {
        let crossing = horizontal_crossing(
            Point { x: 920.0, y: 731.8 },
            Point { x: 936.0, y: 731.8 },
            Point { x: 936.0, y: 731.8 },
            Point { x: 936.0, y: 1636.2 },
        );

        assert_eq!(crossing, None);
    }
}
