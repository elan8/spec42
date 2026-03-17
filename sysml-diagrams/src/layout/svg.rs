use crate::layout::{
    Bounds, DiagramLayout, HitRegion, HitRegionKind, Point, PortSide, RenderedSvg, SvgRenderOptions,
};

pub(crate) fn render_svg(layout: &DiagramLayout, options: &SvgRenderOptions) -> RenderedSvg {
    let mut svg = String::new();
    let mut hit_regions = Vec::new();
    let mut node_groups = String::new();
    let mut edge_group = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\" viewBox=\"0 0 {:.0} {:.0}\" class=\"{}\">",
        layout.width,
        layout.height,
        layout.width,
        layout.height,
        escape_text(&options.class_name)
    ));
    svg.push_str("<style>:root{color-scheme:light dark}.diagram-root{--diagram-paper:var(--vscode-editor-background,#1e1e1e);--diagram-ink:var(--vscode-editor-foreground,#d4d4d4);--diagram-muted:#a0a0a0;--diagram-faint:#6b6b6b;}.diagram-viewport{font-family:var(--vscode-editor-font-family, sans-serif)}.diagram-edge{fill:none;stroke:var(--diagram-ink);opacity:1}.diagram-edge-label{font-size:10px;font-weight:400;fill:var(--diagram-ink);paint-order:stroke;stroke:var(--diagram-paper);stroke-width:3px;stroke-linejoin:round}.diagram-port{fill:var(--diagram-paper);stroke:var(--diagram-ink);stroke-width:1.2px}.diagram-port-label{font-size:10px;fill:var(--diagram-ink)}.diagram-stereotype{font-size:9px;fill:var(--diagram-muted)}.diagram-title{fill:var(--diagram-ink);font-size:11px;font-weight:700}.diagram-typed-by{font-size:10px;font-style:italic;fill:var(--diagram-ink)}.diagram-detail{font-size:10px;fill:var(--diagram-ink)}.diagram-compartment-title{font-size:9px;font-weight:700;fill:var(--diagram-muted)}.diagram-divider{stroke:var(--diagram-faint);stroke-width:1px;opacity:.8}.node-background{fill:var(--diagram-paper);stroke:var(--diagram-ink)}@media print {.diagram-root{--diagram-paper:#ffffff;--diagram-ink:#000000;--diagram-muted:#333333;--diagram-faint:#b5b5b5;}}</style>");
    svg.push_str("<defs>");
    svg.push_str("<marker id=\"edge-arrow-solid\" viewBox=\"0 -5 10 10\" refX=\"9\" refY=\"0\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\"><path d=\"M0,-4L10,0L0,4Z\" fill=\"context-stroke\"/></marker>");
    svg.push_str("<marker id=\"edge-arrow-open\" viewBox=\"0 -5 10 10\" refX=\"9\" refY=\"0\" markerWidth=\"8\" markerHeight=\"8\" orient=\"auto\"><path d=\"M0,-4L10,0L0,4\" fill=\"none\" stroke=\"context-stroke\" stroke-width=\"1.3\"/></marker>");
    svg.push_str("<marker id=\"edge-triangle-hollow\" viewBox=\"0 -6 12 12\" refX=\"11\" refY=\"0\" markerWidth=\"8\" markerHeight=\"8\" orient=\"auto\"><path d=\"M0,0L10,-4L10,4Z\" fill=\"var(--diagram-paper,#ffffff)\" stroke=\"context-stroke\" stroke-width=\"1.2\"/></marker>");
    svg.push_str("<marker id=\"edge-diamond-solid\" viewBox=\"0 -6 12 12\" refX=\"2\" refY=\"0\" markerWidth=\"9\" markerHeight=\"9\" orient=\"auto\"><path d=\"M2,0L6,-4L10,0L6,4Z\" fill=\"context-stroke\" stroke=\"context-stroke\" stroke-width=\"1\"/></marker>");
    svg.push_str("<marker id=\"edge-dot\" viewBox=\"0 0 10 10\" refX=\"5\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\"><circle cx=\"5\" cy=\"5\" r=\"2.4\" fill=\"context-stroke\"/></marker>");
    svg.push_str("</defs>");
    svg.push_str("<g class=\"diagram-viewport\">");

    for edge in &layout.edges {
        let path = path_from_points(&edge.points);
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

    let is_interconnection_view = options.class_name.contains("interconnection-view");

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
            node_groups.push_str(&format!(
                "<rect class=\"node-background\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"4.0\" ry=\"4.0\" style=\"fill:var(--diagram-paper);stroke:var(--diagram-ink);stroke-width:1.6px;stroke-dasharray:none\"/>",
                node.bounds.x,
                node.bounds.y,
                node.bounds.width,
                bar_height
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
                "var(--diagram-ink)",
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
                    "var(--diagram-ink)"
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
                    "var(--diagram-ink)",
                ));
            }
            if kind.contains("requirement") {
                node_groups.push_str(&format!(
                    "<path d=\"M {:.1} {:.1} l 12 0 l 6 6 l 0 12 l -6 6 l -12 0 l -6 -6 l 0 -12 z\" style=\"fill:none;stroke:var(--diagram-ink);stroke-width:1.2px;opacity:.85\"/>",
                    node.bounds.right() - 24.0,
                    node.bounds.y + 12.0
                ));
            }
        }
        for port in &node.ports {
            node_groups.push_str(&format!(
                "<circle class=\"diagram-port\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" data-port-id=\"{}\"/>",
                port.position.x,
                port.position.y,
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
            color: "var(--diagram-ink)",
            dash_array: "none",
            marker_start: "none",
            marker_end: "url(#edge-triangle-hollow)",
            width: "1.7px",
        },
        "typing" => EdgeStyle {
            color: "var(--diagram-ink)",
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
            color: "var(--diagram-ink)",
            dash_array: "none",
            marker_start: "url(#edge-diamond-solid)",
            marker_end: "none",
            width: "1.4px",
        },
        "connection" | "connect" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "none",
            marker_start: "none",
            marker_end: "none",
            width: "1.4px",
        },
        "bind" | "binding" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "6,3",
            marker_start: "none",
            marker_end: "none",
            width: "1.2px",
        },
        "perform" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "6,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-open)",
            width: "1.4px",
        },
        "allocate" | "allocation" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "10,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        "satisfy" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "9,4,2,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        "verify" => EdgeStyle {
            color: "var(--diagram-ink)",
            dash_array: "4,4",
            marker_start: "none",
            marker_end: "url(#edge-arrow-solid)",
            width: "1.4px",
        },
        _ => EdgeStyle {
            color: "var(--diagram-ink)",
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

