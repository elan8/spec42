//! Native Rust SVG drawing for Spec42 diagram views (spec42 #176).
//!
//! Phase 1 spike: General View only, drawing from an already-laid-out graph (`GeneralViewGraph`,
//! the same `LaidOutNode[]`/`LaidOutEdge[]` shape `render/drawing.ts` draws from in
//! `vscode/diagram-renderer`). Layout itself and the other view families are out of scope -- see
//! the issue and `crates/diagram_draw`'s tests for the acceptance bar this spike targets.

mod containers;
mod edges;
mod graph_normalization;
mod markers;
mod node_notation;
mod nodes;
pub mod svg;
pub mod svg_markers;
mod sysml_node;
pub mod theme;
pub mod types;

use std::collections::HashMap;

use svg::Element;
use theme::Theme;
use types::{GeneralViewGraph, LaidOutNode};

/// Port of `renderVisualization`'s General View skeleton in `renderer.ts`: `svg.sysml-viz-svg` ->
/// `rect.viz-bg` + `defs` (markers) + `style` (node chrome) -> `g.viz-root` -> package containers,
/// edges, nodes (that draw order -- containers under edges under nodes -- matches the
/// `redrawGeneral` call sequence in `renderer.ts`).
pub fn render_general_view_svg(
    graph: &GeneralViewGraph,
    theme: &Theme,
    width: f64,
    height: f64,
) -> String {
    let nodes_by_id: HashMap<&str, &LaidOutNode> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();

    let mut root = Element::new("g").attr("class", "viz-root");
    if let Some(containers_layer) =
        containers::draw_general_package_containers(&graph.meta, &graph.nodes, theme)
    {
        root = root.child(containers_layer);
    }
    for edge_layer in edges::draw_edges(&graph.edges, &nodes_by_id, theme) {
        root = root.child(edge_layer);
    }
    root = root.child(nodes::draw_nodes(&graph.nodes, theme));

    let viz_bg = Element::new("rect")
        .attr("class", "viz-bg")
        .attr_f("width", width)
        .attr_f("height", height)
        .attr("fill", theme.canvas_background);

    let svg_root = Element::new("svg")
        .attr("class", "sysml-viz-svg")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr(
            "viewBox",
            format!(
                "0 0 {} {}",
                svg::format_number(width),
                svg::format_number(height)
            ),
        )
        .attr("role", "img")
        .attr(
            "aria-label",
            if graph.title.is_empty() {
                "SysML view".to_string()
            } else {
                graph.title.clone()
            },
        )
        .attr("data-color-scheme", "light")
        .style("touch-action", "none")
        .style("cursor", "grab")
        .child(viz_bg)
        .child(markers::markers(theme))
        .child(markers::node_chrome_style(theme))
        .child(root);

    svg_root.to_string()
}
