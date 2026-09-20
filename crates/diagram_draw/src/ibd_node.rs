//! Port of `renderIbdNode` in `render/drawing.ts`. Unlike General View's `renderSysMLNode` (which
//! builds its own nested `<g>`), `renderIbdNode` draws directly into the outer node group
//! `drawNodes` already built (including conditionally adding the `ibd-container` class to that
//! *same* outer group) -- so this returns the pieces the caller (`nodes.rs`'s IBD node loop) needs
//! to assemble into that single group, rather than a nested element of its own.

use serde_json::Value;

use crate::ibd_ports::draw_ibd_ports;
use crate::node_notation::{
    node_body_chrome_style, notation_role_from_attributes, resolve_node_chrome, ChromeOptions,
    NodeBodyOptions,
};
use crate::svg::Element;
use crate::theme::Theme;
use crate::types::{
    attr_bool, attr_text, InterconnectionLayoutNodeDto, LaidOutNode, IBD_NODE_HEIGHT,
    IBD_NODE_WIDTH,
};

pub struct IbdNodeRender {
    /// Direct children of the outer node group, in draw order (background rect, header rect, then
    /// either the container label or the part stereotype/name/typing/children text, in the exact
    /// order `renderIbdNode` appends them). Whether to add the `ibd-container` class to that outer
    /// group is `nodes.rs`'s own `is_ibd_layout_container(node)` check -- the same condition
    /// `renderIbdNode` computes independently in the original, so it isn't duplicated here.
    pub children: Vec<Element>,
    /// `port-icon`/`port-label` pairs, kept separate for the port-overlay relocation step.
    pub ports: Vec<Element>,
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

pub fn render_ibd_node(
    node: &LaidOutNode,
    selected: bool,
    theme: &Theme,
    layout_node: Option<&InterconnectionLayoutNodeDto>,
) -> IbdNodeRender {
    let is_container = attr_bool(&node.attributes, "isSyntheticContainer")
        || attr_bool(&node.attributes, "isPackageContainer")
        || attr_bool(&node.attributes, "_isLayoutContainer");
    let width = if node.width > 0.0 {
        node.width
    } else {
        IBD_NODE_WIDTH
    };
    let height = if node.height > 0.0 {
        node.height
    } else {
        IBD_NODE_HEIGHT
    };
    let is_package_container = attr_bool(&node.attributes, "isPackageContainer");
    let chrome = resolve_node_chrome(
        notation_role_from_attributes(&node.attributes),
        ChromeOptions {
            is_container: Some(is_container),
            is_package_container,
        },
    );
    let body = node_body_chrome_style(
        &chrome,
        NodeBodyOptions {
            selected,
            is_container: Some(is_container),
            is_package_container,
            general_view: false,
        },
    );
    let stroke = if selected {
        theme.highlight
    } else {
        theme.node_border
    };
    let header_height = if is_container {
        28.0
    } else {
        let part_type = attr_text(&node.attributes, "partType");
        if !part_type.is_empty() {
            41.0
        } else {
            33.0
        }
    };

    let mut children = vec![
        Element::new("rect")
            .attr_f("width", width)
            .attr_f("height", height)
            .attr_f("rx", body.corner_radius)
            .attr("class", "graph-node-background")
            .attr("data-original-stroke", theme.node_border)
            .attr(
                "data-original-width",
                format!("{}px", crate::svg::format_number(body.stroke_width_px)),
            )
            .style("fill", theme.node_fill)
            .style("stroke", stroke)
            .style(
                "stroke-width",
                format!("{}px", crate::svg::format_number(body.stroke_width_px)),
            )
            .style("stroke-dasharray", body.stroke_dasharray),
        Element::new("rect")
            .attr_f("width", width)
            .attr_f("height", header_height)
            .attr("rx", "6")
            .style("fill", theme.panel_background),
    ];

    if is_container {
        children.push(
            Element::new("text")
                .attr_f("x", width / 2.0)
                .attr_f("y", header_height / 2.0 + 4.0)
                .attr("text-anchor", "middle")
                .text(node.label.clone())
                .style("font-size", "11px")
                .style("font-weight", "bold")
                .style("fill", theme.text_primary),
        );
        let ports = draw_ibd_ports(node, width, header_height, theme, layout_node);
        return IbdNodeRender { children, ports };
    }

    let stereo = if node.kind.is_empty() {
        "part".to_string()
    } else {
        node.kind.replace('_', " ")
    };
    children.push(
        Element::new("text")
            .attr_f("x", width / 2.0)
            .attr_f("y", 17.0)
            .attr("text-anchor", "middle")
            .text(format!("\u{ab}{stereo}\u{bb}"))
            .style("font-size", "9px")
            .style("fill", theme.text_primary),
    );
    children.push(
        Element::new("text")
            .attr("class", "node-name-text viz-node-name")
            .attr_f("x", width / 2.0)
            .attr_f("y", 31.0)
            .attr("text-anchor", "middle")
            .text(truncate(&node.label, 18))
            .style("font-size", "11px")
            .style("font-weight", "bold")
            .style("fill", theme.text_primary),
    );

    let typed_by = attr_text(&node.attributes, "partType");
    if !typed_by.is_empty() {
        children.push(
            Element::new("text")
                .attr_f("x", width / 2.0)
                .attr_f("y", 43.0)
                .attr("text-anchor", "middle")
                .text(format!(": {}", truncate(&typed_by, 18)))
                .style("font-size", "10px")
                .style("font-style", "italic")
                .style("fill", theme.text_primary),
        );
    }

    let content_start_y = if !typed_by.is_empty() { 50.0 } else { 38.0 };
    if let Some(Value::Array(items)) = node.attributes.get("children") {
        for (index, child) in items.iter().take(8).enumerate() {
            let child_type = child
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_lowercase();
            let prefix = if child_type.contains("attribute") {
                "[attr] "
            } else if child_type.contains("state") {
                "[state] "
            } else if child_type.contains("part") {
                "[part] "
            } else {
                ""
            };
            let name = child.get("name").and_then(Value::as_str).unwrap_or("");
            if name.is_empty() {
                continue;
            }
            children.push(
                Element::new("text")
                    .attr_f("x", 6.0)
                    .attr_f("y", content_start_y + 8.0 + index as f64 * 12.0)
                    .text(truncate(&format!("{prefix}{name}"), 28))
                    .style("font-size", "9px")
                    .style("fill", theme.text_secondary),
            );
        }
    }

    let ports = draw_ibd_ports(node, width, content_start_y + 20.0, theme, layout_node);
    IbdNodeRender { children, ports }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
