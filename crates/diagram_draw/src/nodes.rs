use crate::node_notation::{notation_role_from_attributes, resolve_node_chrome, ChromeOptions};
use crate::svg::{format_number as n, Element};
use crate::sysml_node::{
    collect_compartments, node_chrome_state_from_attributes, render_node, RenderNodeOptions,
};
use crate::theme::{stroke_color_for_node, Theme};
use crate::types::LaidOutNode;

/// Port of the non-interconnection branch of `drawNodes` in `render/drawing.ts`. The
/// interconnection/IBD branch (`renderIbdNode`) is out of scope for this spike.
pub fn draw_nodes(nodes: &[LaidOutNode], theme: &Theme) -> Element {
    let mut layer = Element::new("g").attr("class", "viz-nodes");
    for node in nodes {
        let is_layout_container = node
            .attributes
            .get("isSyntheticContainer")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
            || node
                .attributes
                .get("isPackageContainer")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            || node
                .attributes
                .get("_isLayoutContainer")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
        let is_package_container = node
            .attributes
            .get("isPackageContainer")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let outer_chrome = resolve_node_chrome(
            notation_role_from_attributes(&node.attributes),
            ChromeOptions {
                is_container: Some(is_layout_container),
                is_package_container,
            },
        );
        let outer_class = format!("general-node viz-node {}", outer_chrome.structure_class);

        let compartments = collect_compartments(&node.label, &node.kind, &node.attributes);
        let chrome = resolve_node_chrome(
            notation_role_from_attributes(&node.attributes),
            ChromeOptions::default(),
        );
        let state = node_chrome_state_from_attributes(&node.attributes);
        let width = if node.width > 0.0 { node.width } else { 200.0 };
        let height = if node.height > 0.0 { node.height } else { 70.0 };

        let outer = Element::new("g")
            .attr("class", outer_class.trim().to_string())
            .attr(
                "transform",
                format!("translate({},{})", n(node.x), n(node.y)),
            )
            .attr("data-node-id", node.id.clone())
            .attr("data-element-name", node.label.clone())
            .attr(
                "data-bounds",
                format!("{},{},{},{}", n(node.x), n(node.y), n(width), n(height)),
            )
            .child(render_node(
                &compartments,
                RenderNodeOptions {
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                    node_class: String::new(),
                    data_element_name: &node.label,
                    stroke_color: stroke_color_for_node(theme),
                    selected: false,
                    chrome,
                    theme,
                    state,
                },
            ));
        layer = layer.child(outer);
    }
    layer
}
