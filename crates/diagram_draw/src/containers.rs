use serde_json::Value;

use crate::svg::Element;
use crate::theme::Theme;
use crate::types::LaidOutNode;

const NODE_WIDTH: f64 = 200.0;
const NODE_HEIGHT: f64 = 70.0;

/// Port of `drawGeneralPackageContainers` in `render/drawing.ts`. Reads
/// `prepared.meta.packageContainerGroups`, carried through in the fixture's top-level `meta`
/// field. Returns `None` when there are no package groups, matching the TS early return (no
/// `.general-package-containers` group is appended at all).
pub fn draw_general_package_containers(
    meta: &Value,
    nodes: &[LaidOutNode],
    theme: &Theme,
) -> Option<Element> {
    let groups = meta.get("packageContainerGroups")?.as_array()?;
    if groups.is_empty() {
        return None;
    }
    let mut layer = Element::new("g").attr("class", "general-package-containers");
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
            .map(|n| n.x + if n.width > 0.0 { n.width } else { NODE_WIDTH })
            .fold(f64::NEG_INFINITY, f64::max);
        let max_y = member_nodes
            .iter()
            .map(|n| {
                n.y + if n.height > 0.0 {
                    n.height
                } else {
                    NODE_HEIGHT
                }
            })
            .fold(f64::NEG_INFINITY, f64::max);
        let padding = 28.0;
        let x = min_x - padding;
        let y = min_y - padding;
        let width = max_x - min_x + padding * 2.0;
        let height = max_y - min_y + padding * 2.0;
        layer = layer.child(
            Element::new("rect")
                .attr("class", "general-package-frame")
                .attr_f("x", x)
                .attr_f("y", y)
                .attr_f("width", width)
                .attr_f("height", height)
                .attr("rx", "18")
                .style("fill", "transparent")
                .style("stroke", theme.node_border)
                .style("stroke-width", "1.5px")
                .style("opacity", "0.9"),
        );
        layer = layer.child(
            Element::new("text")
                .attr("class", "general-package-label")
                .attr_f("x", x + 14.0)
                .attr_f("y", y + 21.0)
                .style("font-size", "11px")
                .style("font-weight", "700")
                .style("fill", theme.node_border)
                .text(label),
        );
    }
    any.then_some(layer)
}
