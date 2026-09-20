//! Shared edge hit-target + tooltip bake-in helpers, used by every view family's edge drawing
//! (`appendPathEdgeHitTarget`/`appendLineEdgeHitTarget`/`markVisibleEdge` in
//! `render/diagram-tooltip.ts`, plus the static `aria-label`/`<title>` bake-in step
//! `installDiagramTooltips` performs on every `[data-tooltip-kind]` element -- see `tooltip.rs`'s
//! module docs for why only that static step, not the hover/positioning machinery, is ported).

use crate::svg::Element;

/// Sets `data-edge-id`/`data-base-stroke-width` on an already-built visible-edge element,
/// matching `markVisibleEdge`.
pub fn mark_visible_edge(element: Element, edge_id: &str, stroke_width: &str) -> Element {
    element
        .attr("data-edge-id", edge_id)
        .attr("data-base-stroke-width", stroke_width)
}

/// Port of `appendPathEdgeHitTarget` plus the static half of `installDiagramTooltips`'s bake-in
/// (`aria-label` + `<title>`), inlined here since both always happen together in the exported SVG.
/// `tooltip_text` is `None` when no descriptor matches this edge's id (e.g. `installDiagramTooltips`
/// keys descriptors off `prepared.edges`, which sequence-view messages are not members of).
pub fn path_edge_hit_target(path_d: &str, edge_id: &str, tooltip_text: Option<&str>) -> Element {
    let element = Element::new("path")
        .attr("class", "viz-edge-hit-target")
        .attr("data-tooltip-kind", "edge")
        .attr("data-tooltip-id", edge_id)
        .attr("d", path_d)
        .style("fill", "none")
        .style("stroke", "transparent")
        .style("stroke-width", "12px")
        .style("pointer-events", "stroke");
    apply_tooltip(element, tooltip_text)
}

/// Port of `appendLineEdgeHitTarget` plus the same static tooltip bake-in.
pub fn line_edge_hit_target(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    edge_id: &str,
    tooltip_text: Option<&str>,
) -> Element {
    let element = Element::new("line")
        .attr("class", "viz-edge-hit-target")
        .attr("data-tooltip-kind", "edge")
        .attr("data-tooltip-id", edge_id)
        .attr_f("x1", x1)
        .attr_f("y1", y1)
        .attr_f("x2", x2)
        .attr_f("y2", y2)
        .style("stroke", "transparent")
        .style("stroke-width", "12px")
        .style("pointer-events", "stroke");
    apply_tooltip(element, tooltip_text)
}

fn apply_tooltip(element: Element, tooltip_text: Option<&str>) -> Element {
    match tooltip_text {
        Some(text) => element
            .attr("aria-label", text.replace('\n', "; "))
            .child(Element::new("title").text(text)),
        None => element,
    }
}
