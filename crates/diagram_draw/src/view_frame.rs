//! Root-view notation shared by the five standard graphical views.

use crate::svg::{format_number as n, Element};
use crate::theme::Theme;

const VIEW_FRAME_PADDING: f64 = 12.0;
const VIEW_FRAME_TAB_HEIGHT: f64 = 22.0;
const VIEW_FRAME_TAB_CUT: f64 = 8.0;

/// The frame of a view (SysML v2 §§8.2.3.1 and 8.2.3.26): a rectangle around the content with the view's name in
/// a compartment in its upper left corner. Returns the frame and the extent it covers, as
/// `{min_x, min_y, max_x, max_y}`.
pub fn view_frame(
    title: &str,
    content: (f64, f64, f64, f64),
    theme: &Theme,
) -> (Element, (f64, f64, f64, f64)) {
    let view_name = title;
    let title = format!("«view» {title}");
    let tab_width = crate::svg::estimated_text_width(&title, 12.0) + 24.0;
    let min_x = content.0 - VIEW_FRAME_PADDING;
    let min_y = content.1 - VIEW_FRAME_PADDING - VIEW_FRAME_TAB_HEIGHT;
    let max_x = (content.2 + VIEW_FRAME_PADDING).max(min_x + tab_width + VIEW_FRAME_PADDING);
    let max_y = content.3 + VIEW_FRAME_PADDING;
    let tab = format!(
        "M{},{} H{} V{} L{},{} H{}",
        n(min_x),
        n(min_y),
        n(min_x + tab_width),
        n(min_y + VIEW_FRAME_TAB_HEIGHT - VIEW_FRAME_TAB_CUT),
        n(min_x + tab_width - VIEW_FRAME_TAB_CUT),
        n(min_y + VIEW_FRAME_TAB_HEIGHT),
        n(min_x),
    );
    let frame = Element::new("g")
        .attr("class", "view-frame")
        .attr("data-view-name", view_name)
        .style("pointer-events", "none")
        .child(
            Element::new("rect")
                .attr("class", "view-frame-border")
                .attr_f("x", min_x)
                .attr_f("y", min_y)
                .attr_f("width", max_x - min_x)
                .attr_f("height", max_y - min_y)
                .style("fill", "none")
                .style("stroke", theme.node_border)
                .style("stroke-width", "1px"),
        )
        .child(
            Element::new("path")
                .attr("class", "view-frame-name-compartment")
                .attr("d", tab)
                .style("fill", theme.panel_background)
                .style("stroke", theme.node_border)
                .style("stroke-width", "1px"),
        )
        .child(
            Element::new("text")
                .attr("class", "view-frame-name")
                .attr_f("x", min_x + 10.0)
                .attr_f("y", min_y + VIEW_FRAME_TAB_HEIGHT - 7.0)
                .style("font-size", "12px")
                .style("font-weight", "700")
                .style("fill", theme.text_primary)
                .text(title),
        );
    (frame, (min_x, min_y, max_x, max_y))
}

/// Enclose the completed drawing, including text, containers and boundary decorations.
pub(crate) fn enclose(
    root: Element,
    title: &str,
    content: (f64, f64, f64, f64),
    theme: &Theme,
) -> (Element, (f64, f64, f64, f64)) {
    let content = root.decorated_extent(content);
    let (frame, extent) = view_frame(title, content, theme);
    (root.prepend_child(frame), extent)
}
