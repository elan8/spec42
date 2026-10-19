//! Port of `drawInterconnectionContainers`, `shouldDrawIbdViewFrame`, and `drawIbdViewFrame` in
//! `render/drawing.ts`. Note these use plain attributes (`.attr("stroke", ...)`) for styling,
//! unlike `drawGeneralPackageContainers`' `.style(...)` calls -- a real difference in the
//! original, not an inconsistency to "fix".

use serde_json::Value;

use crate::svg::Element;
use crate::theme::Theme;
use crate::types::{
    InterconnectionLayoutContainerDto, LaidOutNode, IBD_NODE_HEIGHT, IBD_NODE_WIDTH,
};

/// Port of `drawInterconnectionContainers`. Prefers real layout containers; falls back to
/// `prepared.meta.packageContainerGroups`-derived boxes (mirroring General View's package frames,
/// but with IBD-node-sized defaults and `ibd-containers`/`ibd-part ibd-container` classes) only
/// when the layout pass produced none.
pub fn draw_interconnection_containers(
    meta: &Value,
    nodes: &[LaidOutNode],
    theme: &Theme,
    layout_containers: &[InterconnectionLayoutContainerDto],
) -> Option<Element> {
    if !layout_containers.is_empty() {
        let mut layer = Element::new("g").attr("class", "ibd-containers");
        for container in layout_containers {
            let group = Element::new("g")
                .attr("class", "ibd-part ibd-container")
                .attr(
                    "transform",
                    format!(
                        "translate({},{})",
                        crate::svg::format_number(container.x),
                        crate::svg::format_number(container.y)
                    ),
                )
                .attr("data-element-name", container.label.clone())
                .child(
                    Element::new("rect")
                        .attr_f("width", container.width)
                        .attr_f("height", container.height)
                        .attr("rx", "14")
                        .attr("fill", "none")
                        .attr("stroke", theme.node_border)
                        .attr("stroke-width", "1.4")
                        .attr("stroke-dasharray", "6,4")
                        .attr("opacity", "0.7"),
                )
                .child(
                    Element::new("text")
                        .attr_f("x", 12.0)
                        .attr_f("y", 20.0)
                        .attr("fill", theme.text_secondary)
                        .attr("font-size", "11")
                        .text(container.label.clone()),
                );
            layer = layer.child(group);
        }
        return Some(layer);
    }

    let groups = meta.get("packageContainerGroups")?.as_array()?;
    if groups.is_empty() {
        return None;
    }
    let mut layer = Element::new("g").attr("class", "ibd-containers");
    let mut any = false;
    for group in groups {
        let member_ids: Vec<&str> = group
            .get("memberIds")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let label = group
            .get("name")
            .or_else(|| group.get("label"))
            .or_else(|| group.get("id"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let member_nodes: Vec<&LaidOutNode> = member_ids
            .iter()
            .filter_map(|id| nodes.iter().find(|node| node.id == *id))
            .collect();
        if member_nodes.is_empty() {
            continue;
        }
        any = true;
        let min_x = member_nodes
            .iter()
            .map(|n| n.x)
            .fold(f64::INFINITY, f64::min);
        let min_y = member_nodes
            .iter()
            .map(|n| n.y)
            .fold(f64::INFINITY, f64::min);
        let max_x = member_nodes
            .iter()
            .map(|n| {
                n.x + if n.width > 0.0 {
                    n.width
                } else {
                    IBD_NODE_WIDTH
                }
            })
            .fold(f64::NEG_INFINITY, f64::max);
        let max_y = member_nodes
            .iter()
            .map(|n| {
                n.y + if n.height > 0.0 {
                    n.height
                } else {
                    IBD_NODE_HEIGHT
                }
            })
            .fold(f64::NEG_INFINITY, f64::max);
        let padding = 26.0;
        let x = min_x - padding;
        let y = min_y - padding;
        let width = max_x - min_x + padding * 2.0;
        let height = max_y - min_y + padding * 2.0;
        layer = layer.child(
            Element::new("g")
                .attr("class", "ibd-part ibd-container")
                .attr(
                    "transform",
                    format!(
                        "translate({},{})",
                        crate::svg::format_number(x),
                        crate::svg::format_number(y)
                    ),
                )
                .attr("data-element-name", label.clone())
                .child(
                    Element::new("rect")
                        .attr_f("width", width)
                        .attr_f("height", height)
                        .attr("rx", "14")
                        .attr("fill", "none")
                        .attr("stroke", theme.node_border)
                        .attr("stroke-width", "1.4")
                        .attr("stroke-dasharray", "6,4")
                        .attr("opacity", "0.7"),
                )
                .child(
                    Element::new("text")
                        .attr_f("x", 12.0)
                        .attr_f("y", 20.0)
                        .attr("fill", theme.text_secondary)
                        .attr("font-size", "11")
                        .text(label),
                ),
        );
    }
    any.then_some(layer)
}
