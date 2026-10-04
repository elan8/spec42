use std::collections::HashMap;

use crate::ibd_node::render_ibd_node;
use crate::node_notation::{notation_role_from_attributes, resolve_node_chrome, ChromeOptions};
use crate::svg::{format_number as n, Element};
use crate::sysml_node::{
    collect_compartments, node_chrome_state_from_attributes, render_node, RenderNodeOptions,
};
use crate::theme::{stroke_color_for_node, Theme};
use crate::types::{
    attr_bool, InterconnectionLayoutDto, InterconnectionLayoutNodeDto, LaidOutNode,
    IBD_NODE_HEIGHT, IBD_NODE_WIDTH,
};

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

        let compartments = node
            .compartments
            .clone()
            .unwrap_or_else(|| collect_compartments(&node.label, &node.kind, &node.attributes));
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

fn is_ibd_layout_container(node: &LaidOutNode) -> bool {
    attr_bool(&node.attributes, "isSyntheticContainer")
        || attr_bool(&node.attributes, "isPackageContainer")
        || attr_bool(&node.attributes, "_isLayoutContainer")
}

fn layout_depth(node: &LaidOutNode) -> f64 {
    node.attributes
        .get("_layoutDepth")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
}

/// Port of `orderIbdNodesForPaint`: containers first, then by `_layoutDepth` ascending within
/// each group, with original array position as the final tiebreak (this comparator already
/// covers every case explicitly, so a stable sort isn't load-bearing here the way it is in the
/// original -- but `sort_by` is stable regardless).
fn order_ibd_nodes_for_paint(nodes: &[LaidOutNode]) -> Vec<&LaidOutNode> {
    let mut indexed: Vec<(usize, &LaidOutNode)> = nodes.iter().enumerate().collect();
    indexed.sort_by(|(a_index, a), (b_index, b)| {
        let a_container = is_ibd_layout_container(a);
        let b_container = is_ibd_layout_container(b);
        if a_container != b_container {
            return if a_container {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            };
        }
        let a_depth = layout_depth(a);
        let b_depth = layout_depth(b);
        a_depth.total_cmp(&b_depth).then(a_index.cmp(b_index))
    });
    indexed.into_iter().map(|(_, node)| node).collect()
}

/// Port of the Interconnection-View branch of `drawNodes`, plus `drawInterconnectionPortOverlays`
/// folded in as a same-pass split (see `ibd_node.rs`'s module docs) instead of a DOM-move
/// post-pass. Returns `g.viz-nodes` and, when any node has ports, `g.viz-port-overlays`.
pub fn draw_ibd_nodes(
    nodes: &[LaidOutNode],
    theme: &Theme,
    layout: Option<&InterconnectionLayoutDto>,
) -> (Element, Option<Element>) {
    let ordered = order_ibd_nodes_for_paint(nodes);
    let layout_nodes_by_id: HashMap<&str, &InterconnectionLayoutNodeDto> = layout
        .map(|l| l.nodes.iter().map(|n| (n.id.as_str(), n)).collect())
        .unwrap_or_default();

    let mut viz_nodes = Element::new("g").attr("class", "viz-nodes");
    let mut overlay_layer = Element::new("g")
        .attr("class", "viz-port-overlays")
        .style("pointer-events", "none");
    let mut overlay_count = 0;

    for node in ordered {
        let is_layout_container = is_ibd_layout_container(node);
        let is_package_container = attr_bool(&node.attributes, "isPackageContainer");
        let outer_chrome = resolve_node_chrome(
            notation_role_from_attributes(&node.attributes),
            ChromeOptions {
                is_container: Some(is_layout_container),
                is_package_container,
            },
        );
        let base_class = format!("ibd-part viz-node {}", outer_chrome.structure_class);
        let outer_class = if is_layout_container {
            format!("{base_class} ibd-container")
        } else {
            base_class
        };

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
        let layout_node = layout_nodes_by_id.get(node.id.as_str()).copied();
        let rendered = render_ibd_node(node, false, theme, layout_node);

        let mut outer = Element::new("g")
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
            );
        for child in rendered.children {
            outer = outer.child(child);
        }
        viz_nodes = viz_nodes.child(outer);

        if !rendered.ports.is_empty() {
            let mut overlay = Element::new("g")
                .attr("class", "viz-port-overlay")
                .attr("data-node-id", node.id.clone())
                .attr(
                    "transform",
                    format!("translate({},{})", n(node.x), n(node.y)),
                );
            for port in rendered.ports {
                overlay = overlay.child(port);
                overlay_count += 1;
            }
            overlay_layer = overlay_layer.child(overlay);
        }
    }

    (viz_nodes, (overlay_count > 0).then_some(overlay_layer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sysml_node::{Compartments, DetailItem, Section};
    use crate::theme::LIGHT;
    use serde_json::json;
    use std::collections::BTreeMap;

    /// No production-shaped fixture can exercise "prefer the precomputed `compartments` field"
    /// versus "always recompute from `attributes`" as distinct behaviors, because a real fixture's
    /// `compartments` blob and its `attributes` always agree (both were produced from the same
    /// state at the same instant by `reshapeGeneralLayoutResult`). This test manufactures the
    /// divergence directly -- attributes that would recompute to a *collapsed*, different-content
    /// section, alongside a precomputed `compartments` snapshot that is already resolved
    /// *expanded* with different items -- and asserts the drawn SVG reflects the precomputed
    /// snapshot, matching `d.compartments ?? collectCompartments(d)` in `drawing.ts`.
    #[test]
    fn prefers_precomputed_compartments_over_recomputing_from_attributes() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "generalViewInheritedAttributes".to_string(),
            json!(["a", "b"]),
        );

        let precomputed = Compartments {
            stereotype: "part def".to_string(),
            name: "Node".to_string(),
            typed_by_name: None,
            attributes: Vec::new(),
            parts: Vec::new(),
            ports: Vec::new(),
            collapsible_sections: vec![Section {
                key: "inherited-attributes".to_string(),
                title: "Attributes".to_string(),
                items: vec![
                    DetailItem {
                        display_text: "x".to_string(),
                        declared_in: None,
                    },
                    DetailItem {
                        display_text: "y".to_string(),
                        declared_in: None,
                    },
                ],
                collapsed: false,
            }],
        };

        let node = LaidOutNode {
            id: "n".to_string(),
            label: "Node".to_string(),
            kind: "part def".to_string(),
            attributes,
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 100.0,
            compartments: Some(precomputed),
        };

        let svg = draw_nodes(std::slice::from_ref(&node), &LIGHT).to_string();
        assert!(
            svg.contains(">x<") && svg.contains(">y<"),
            "expected precomputed items in output:\n{svg}"
        );
        assert!(
            !svg.contains(">a<") && !svg.contains(">b<"),
            "attributes-recomputed items must not appear when compartments is precomputed:\n{svg}"
        );
    }
}
