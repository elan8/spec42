use crate::layout::{
    layout_with_report, render_svg, DiagramEdge, DiagramGraph, DiagramNode, DiagramPort,
    LayoutConfig, PortSide, SvgRenderOptions,
};

use crate::{RenderedDiagram, Result, ViewState};

#[allow(dead_code)]
pub fn build_rendered_diagram(
    graph: DiagramGraph,
    view: &str,
    selection: Option<String>,
) -> Result<RenderedDiagram> {
    build_rendered_diagram_with_config(graph, view, selection, LayoutConfig::default())
}

pub fn build_rendered_diagram_with_config(
    graph: DiagramGraph,
    view: &str,
    selection: Option<String>,
    config: LayoutConfig,
) -> Result<RenderedDiagram> {
    let (layout, report) = layout_with_report(&graph, &config)?;
    let visual_edge_bridges = std::env::var("SPEC42_SVG_EDGE_BRIDGES")
        .ok()
        .as_deref()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        && view.contains("interconnection-view");
    let rendered = render_svg(
        &layout,
        &SvgRenderOptions {
            class_name: format!("diagram-root {}", view),
            visual_edge_bridges,
        },
    );
    Ok(RenderedDiagram {
        svg: rendered.svg,
        hit_map: rendered.hit_regions,
        bounds: rendered.bounds,
        metrics: report.metrics,
        warnings: report.warnings,
        view_state: ViewState {
            view: view.to_string(),
            selection,
        },
    })
}

pub fn port_side_from_text(side: Option<&str>, direction: Option<&str>) -> PortSide {
    match side.unwrap_or_default().to_ascii_lowercase().as_str() {
        "left" | "west" => PortSide::Left,
        "right" | "east" => PortSide::Right,
        "top" | "north" => PortSide::Top,
        "bottom" | "south" => PortSide::Bottom,
        _ => match direction.unwrap_or_default().to_ascii_lowercase().as_str() {
            "in" => PortSide::Left,
            "out" => PortSide::Right,
            _ => PortSide::Right,
        },
    }
}

pub fn detail_lines(attributes: &[(String, String)], extra: &[String]) -> Vec<String> {
    let mut lines: Vec<String> = attributes
        .iter()
        .filter(|(key, _)| !is_internal_attribute(key))
        .take(4)
        .map(|(key, value)| format!("{key}: {value}"))
        .collect();
    lines.extend(extra.iter().take(4).cloned());
    lines
}

fn is_internal_attribute(key: &str) -> bool {
    matches!(key, "synthetic" | "originRange")
}

pub fn default_node(
    id: String,
    label: String,
    kind: String,
    parent_id: Option<String>,
    detail_lines: Vec<String>,
    ports: Vec<DiagramPort>,
    width: f32,
    height: f32,
) -> DiagramNode {
    DiagramNode {
        id,
        label,
        kind,
        width,
        height,
        parent_id,
        detail_lines,
        ports,
    }
}

pub fn edge(
    id: String,
    source_node: String,
    target_node: String,
    source_port: Option<String>,
    target_port: Option<String>,
    label: Option<String>,
    kind: String,
) -> DiagramEdge {
    DiagramEdge {
        id,
        source_node,
        target_node,
        source_port,
        target_port,
        label,
        kind,
    }
}
