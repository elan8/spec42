//! Native Rust SVG drawing for Spec42 diagram views (spec42 #176).
//!
//! Phase 1 ported General View. Phase 2 adds Interconnection View and the three Behavior views
//! (action-flow, state-transition, sequence). All five draw from an already-laid-out/prepared
//! input -- layout itself, headless-export wiring, and the interactive webview stay out of scope;
//! see the issue and this crate's tests for the acceptance bar each view targets.

pub mod action_flow;
pub mod behavior_common;
mod containers;
mod edges;
mod graph_normalization;
mod hit_target;
mod markers;
mod node_notation;
mod nodes;
pub mod sequence;
pub mod state_transition;
pub mod svg;
pub mod svg_markers;
mod sysml_node;
pub mod theme;
mod tooltip;
pub mod types;
pub mod xml;

use std::collections::HashMap;

use behavior_common::{BehaviorLayoutResult, PreparedView};
use svg::{format_number as n, Element};
use theme::Theme;
use types::{GeneralViewGraph, LaidOutNode};

const DEFAULT_NODE_WIDTH: f64 = 200.0;
const DEFAULT_NODE_HEIGHT: f64 = 70.0;

/// Port of `contentBounds` in `render/export.ts`: the bounding box of every node's laid-out rect,
/// falling back to a fixed placeholder box when there are no nodes. General-View-specific (its
/// `LaidOutNode` shape); the behavior views compute their own extents directly, matching
/// `renderActionFlowView`/`renderStateTransitionView`/`renderSequenceView`'s own return values.
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

/// Port of `renderVisualization`'s shared SVG skeleton in `renderer.ts`, as it actually reaches
/// serialized output through `exportHeadlessSvg`/`controller.exportSvg()` (`render/export.ts`):
/// `svg.sysml-viz-svg` -> `rect.viz-bg` + `defs` (the 8 general/IBD markers, plus `extra_marker`
/// when a behavior view supplies one -- `addMarkers`/`addActionFlowMarkers`/etc. all append into
/// the *same* `defs`, confirmed from `renderer.ts`: every view's SVG carries all 8 general/IBD
/// markers regardless of whether it uses them) + `style` (node chrome) -> `g.viz-root` -> view
/// content. `width`/`height` size the `viz-bg` canvas rect only; `bounds` (already `{x,y,width,
/// height}`) independently feeds the `viewBox`, padded 40px on every side, exactly as `exportSvg`
/// computes it -- the two are unrelated in the original.
fn render_svg_document(
    theme: &Theme,
    width: f64,
    height: f64,
    title: &str,
    bounds: (f64, f64, f64, f64),
    extra_marker: Option<Element>,
    root_content: Element,
) -> String {
    let viz_bg = Element::new("rect")
        .attr("class", "viz-bg")
        .attr_f("width", width)
        .attr_f("height", height)
        .attr("fill", theme.canvas_background);

    let (bounds_x, bounds_y, bounds_width, bounds_height) = bounds;
    let view_box = format!(
        "{} {} {} {}",
        n(bounds_x - 40.0),
        n(bounds_y - 40.0),
        n(bounds_width + 80.0),
        n(bounds_height + 80.0)
    );

    let defs = markers::markers(theme).maybe_child(extra_marker);

    Element::new("svg")
        .attr("class", "sysml-viz-svg")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("viewBox", view_box)
        .attr("role", "img")
        .attr(
            "aria-label",
            if title.is_empty() {
                "SysML view".to_string()
            } else {
                title.to_string()
            },
        )
        .attr("data-color-scheme", "light")
        .style("touch-action", "none")
        .style("cursor", "grab")
        .child(viz_bg)
        .child(defs)
        .child(markers::node_chrome_style(theme))
        .child(root_content)
        .attr_trailing("xmlns", "http://www.w3.org/2000/svg")
        .to_string()
}

/// Port of General View's branch of `renderVisualization`: package containers, edges, nodes (that
/// draw order -- containers under edges under nodes -- matches the `redrawGeneral` call sequence
/// in `renderer.ts`).
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

    render_svg_document(
        theme,
        width,
        height,
        &graph.title,
        content_bounds(&graph.nodes),
        None,
        root,
    )
}

/// Port of `renderSequenceView`'s call site in `renderer.ts` (`addSequenceMarkers` +
/// `renderSequenceView`, then `contentBoundsFromExtents` feeding `exportSvg`'s `viewBox`).
pub fn render_sequence_view_svg(
    prepared: &PreparedView,
    theme: &Theme,
    width: f64,
    height: f64,
) -> String {
    let (root, (min_x, min_y, max_x, max_y)) =
        sequence::render_sequence_view(prepared, theme, width, height);
    let bounds = (
        min_x,
        min_y,
        (max_x - min_x).max(1.0),
        (max_y - min_y).max(1.0),
    );
    render_svg_document(
        theme,
        width,
        height,
        &prepared.title,
        bounds,
        Some(sequence::sequence_marker(theme)),
        root,
    )
}

/// Port of `renderStateTransitionView`'s call site in `renderer.ts`.
pub fn render_state_transition_view_svg(
    prepared: &PreparedView,
    layout: &BehaviorLayoutResult,
    theme: &Theme,
    width: f64,
    height: f64,
) -> String {
    let (root, (min_x, min_y, max_x, max_y)) =
        state_transition::render_state_transition_view(prepared, layout, theme, width, height);
    let bounds = (
        min_x,
        min_y,
        (max_x - min_x).max(1.0),
        (max_y - min_y).max(1.0),
    );
    render_svg_document(
        theme,
        width,
        height,
        &prepared.title,
        bounds,
        Some(state_transition::state_transition_marker(theme)),
        root,
    )
}

/// Port of `renderActionFlowView`'s call site in `renderer.ts`.
pub fn render_action_flow_view_svg(
    prepared: &PreparedView,
    layout: &BehaviorLayoutResult,
    theme: &Theme,
    width: f64,
    height: f64,
) -> String {
    let (root, (min_x, min_y, max_x, max_y)) =
        action_flow::render_action_flow_view(prepared, layout, theme, width, height);
    let bounds = (
        min_x,
        min_y,
        (max_x - min_x).max(1.0),
        (max_y - min_y).max(1.0),
    );
    render_svg_document(
        theme,
        width,
        height,
        &prepared.title,
        bounds,
        Some(action_flow::action_flow_marker(theme)),
        root,
    )
}
