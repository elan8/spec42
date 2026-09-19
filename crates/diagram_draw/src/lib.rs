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
mod tooltip;
pub mod types;
pub mod xml;

use std::collections::HashMap;

use svg::Element;
use theme::Theme;
use types::{GeneralViewGraph, LaidOutNode};

const DEFAULT_NODE_WIDTH: f64 = 200.0;
const DEFAULT_NODE_HEIGHT: f64 = 70.0;

/// Port of `contentBounds` in `render/export.ts`: the bounding box of every node's laid-out rect,
/// falling back to a fixed placeholder box when there are no nodes.
fn content_bounds(nodes: &[LaidOutNode]) -> (f64, f64, f64, f64) {
    if nodes.is_empty() {
        return (0.0, 0.0, 100.0, 100.0);
    }
    let min_x = nodes
        .iter()
        .map(|node| node.x)
        .fold(f64::INFINITY, f64::min);
    let min_y = nodes
        .iter()
        .map(|node| node.y)
        .fold(f64::INFINITY, f64::min);
    let max_x = nodes
        .iter()
        .map(|node| {
            node.x
                + if node.width > 0.0 {
                    node.width
                } else {
                    DEFAULT_NODE_WIDTH
                }
        })
        .fold(f64::NEG_INFINITY, f64::max);
    let max_y = nodes
        .iter()
        .map(|node| {
            node.y
                + if node.height > 0.0 {
                    node.height
                } else {
                    DEFAULT_NODE_HEIGHT
                }
        })
        .fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, max_x - min_x, max_y - min_y)
}

/// Port of `renderVisualization`'s General View skeleton in `renderer.ts`, as it actually reaches
/// serialized output through `exportHeadlessSvg`/`controller.exportSvg()` (`render/export.ts`):
/// `svg.sysml-viz-svg` -> `rect.viz-bg` + `defs` (markers) + `style` (node chrome) -> `g.viz-root`
/// -> package containers, edges, nodes (that draw order -- containers under edges under nodes --
/// matches the `redrawGeneral` call sequence in `renderer.ts`). `width`/`height` size the
/// `viz-bg` canvas rect only; the `viewBox` is independently derived from `content_bounds` plus
/// 40px padding on every side, exactly as `exportSvg` computes it -- the two are unrelated in the
/// original and conflating them was a bug caught by comparing against a real rendered fixture (see
/// `tests/golden_parity.rs`).
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

    let (bounds_x, bounds_y, bounds_width, bounds_height) = content_bounds(&graph.nodes);
    let view_box = format!(
        "{} {} {} {}",
        svg::format_number(bounds_x - 40.0),
        svg::format_number(bounds_y - 40.0),
        svg::format_number(bounds_width + 80.0),
        svg::format_number(bounds_height + 80.0)
    );

    let svg_root = Element::new("svg")
        .attr("class", "sysml-viz-svg")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("viewBox", view_box)
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
        .child(root)
        .attr_trailing("xmlns", "http://www.w3.org/2000/svg");

    svg_root.to_string()
}
