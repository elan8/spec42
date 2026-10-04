use crate::svg::Element;
use crate::theme::Theme;

/// Port of `addMarkers` in `render/export.ts`. All 8 markers are appended unconditionally
/// regardless of view type -- confirmed by the golden fixture's `markerIds` list containing every
/// one of them even for a General View payload with no interconnection edges.
pub fn markers(theme: &Theme) -> Element {
    Element::new("defs")
        .child(
            Element::new("marker")
                .attr("id", "viz-arrow")
                .attr("markerWidth", "10")
                .attr("markerHeight", "10")
                .attr("refX", "9")
                .attr("refY", "3")
                .attr("orient", "auto")
                .attr("markerUnits", "strokeWidth")
                .child(
                    Element::new("path")
                        .attr("d", "M0,0 L0,6 L9,3 z")
                        .attr("fill", theme.edge_default),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "general-d3-arrow")
                .attr("viewBox", "0 -5 10 10")
                .attr("refX", "8")
                .attr("refY", "0")
                .attr("markerWidth", "5")
                .attr("markerHeight", "5")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,-4L10,0L0,4")
                        .style("fill", theme.edge_default),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "general-d3-arrow-open")
                .attr("viewBox", "0 -5 10 10")
                .attr("refX", "9")
                .attr("refY", "0")
                .attr("markerWidth", "8")
                .attr("markerHeight", "8")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,-4L10,0L0,4")
                        .style("fill", "none")
                        .style("stroke", theme.edge_default)
                        .style("stroke-width", "1.3"),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "general-d3-specializes")
                .attr("viewBox", "0 -6 12 12")
                .attr("refX", "11")
                .attr("refY", "0")
                .attr("markerWidth", "8")
                .attr("markerHeight", "8")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,0L10,-4L10,4Z")
                        .style("fill", theme.node_fill)
                        .style("stroke", theme.edge_default)
                        .style("stroke-width", "1.2"),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "general-d3-diamond")
                .attr("viewBox", "0 -6 12 12")
                .attr("refX", "2")
                .attr("refY", "0")
                .attr("markerWidth", "7")
                .attr("markerHeight", "7")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,0L5,-4L10,0L5,4Z")
                        .style("fill", theme.edge_default),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "ibd-connection-dot")
                .attr("viewBox", "-5 -5 10 10")
                .attr("refX", "0")
                .attr("refY", "0")
                .attr("markerWidth", "5")
                .attr("markerHeight", "5")
                .attr("orient", "auto")
                .child(
                    Element::new("circle")
                        .attr("r", "3")
                        .style("fill", theme.node_fill)
                        .style("stroke", theme.edge_default)
                        .style("stroke-width", "1.5"),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "ibd-flow-arrow")
                .attr("viewBox", "0 -5 10 10")
                .attr("refX", "10")
                .attr("refY", "0")
                .attr("markerWidth", "8")
                .attr("markerHeight", "8")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,-4L10,0L0,4Z")
                        .style("fill", theme.edge_default),
                ),
        )
        .child(
            Element::new("marker")
                .attr("id", "ibd-interface-arrow")
                .attr("viewBox", "0 -5 10 10")
                .attr("refX", "10")
                .attr("refY", "0")
                .attr("markerWidth", "8")
                .attr("markerHeight", "8")
                .attr("orient", "auto")
                .child(
                    Element::new("path")
                        .attr("d", "M0,-4L10,0L0,4Z")
                        .style("fill", "none")
                        .style("stroke", theme.edge_default)
                        .style("stroke-width", "1.5"),
                ),
        )
}

/// Port of `nodeChromeStyleSheet`/`installNodeChromeStyles` in `render/node-chrome-style.ts`.
pub fn node_chrome_style_sheet(theme: &Theme) -> String {
    [
        ".sysml-disclosure { cursor: pointer; outline: none; }".to_string(),
        ".sysml-disclosure .sysml-disclosure-box,".to_string(),
        ".sysml-disclosure .sysml-disclosure-glyph { pointer-events: none; }".to_string(),
        format!(
            ".sysml-disclosure:hover .sysml-disclosure-target {{ fill: {} !important; fill-opacity: 0.55; }}",
            theme.control_hover_fill
        ),
        format!(
            ".sysml-disclosure:hover .sysml-disclosure-box {{ fill: {}; stroke: {}; }}",
            theme.control_hover_fill, theme.control_foreground
        ),
        format!(
            ".sysml-disclosure:focus-visible .sysml-disclosure-target {{ stroke: {}; stroke-width: 2px; fill: {} !important; fill-opacity: 0.35; }}",
            theme.focus_ring, theme.control_hover_fill
        ),
        format!(".sysml-disclosure:focus-visible .sysml-disclosure-box {{ stroke: {}; }}", theme.focus_ring),
        ".sysml-compartment-label text { user-select: none; }".to_string(),
        ".viz-node text { user-select: none; }".to_string(),
    ]
    .join("\n")
}

pub fn node_chrome_style(theme: &Theme) -> Element {
    Element::new("style")
        .attr("class", "sysml-node-chrome-style")
        .text(node_chrome_style_sheet(theme))
}
