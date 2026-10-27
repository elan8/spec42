//! Legend of an interconnection view's connector colours: one entry per port family, plus the
//! neutral colour of connectors whose ports share no family.

use crate::ibd_edges::{port_family_color, PORT_FAMILY_COLOR_COUNT};
use crate::svg::Element;
use crate::theme::{stroke_color_for_edge, Theme};
use crate::types::LaidOutEdge;
use serde_json::Value;

const FONT_SIZE: f64 = 11.0;
const ROW_HEIGHT: f64 = 18.0;
const SWATCH: f64 = 18.0;
const GAP: f64 = 6.0;

/// The legend laid out as a column from `(left, top)`, with its bottom edge, or
/// `None` when the view has no typed connection to explain.
pub fn draw_port_family_legend(
    meta: &Value,
    edges: &[LaidOutEdge],
    theme: &Theme,
    (left, top): (f64, f64),
) -> Option<(Element, f64)> {
    let families: Vec<&str> = meta
        .get("portFamilies")
        .and_then(Value::as_array)
        .map(|families| families.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let typed_connection = |edge: &&LaidOutEdge| {
        edge.attributes.get("typedProjection") == Some(&Value::Bool(true))
            && edge.edge_kind.as_deref() == Some("connection")
    };
    let unrelated = edges
        .iter()
        .filter(typed_connection)
        .any(|edge| !edge.attributes.contains_key("portFamily"));
    if families.is_empty() && !unrelated {
        return None;
    }

    let mut entries: Vec<(String, &'static str)> = families
        .iter()
        .enumerate()
        .map(|(index, label)| (label.to_string(), port_family_color(theme, index)))
        .collect();
    if unrelated {
        entries.push((
            "No common port type".to_string(),
            stroke_color_for_edge(theme),
        ));
    }

    let mut legend = Element::new("g").attr("class", "ibd-port-legend");
    legend = legend.child(
        Element::new("text")
            .attr("class", "ibd-port-legend-title")
            .attr_f("x", left)
            .attr_f("y", top + FONT_SIZE)
            .style("font-size", format!("{FONT_SIZE}px"))
            .style("font-weight", "700")
            .style("fill", theme.text_primary)
            .text("Port families"),
    );
    // One entry per row, like a conventional legend.
    let first_row = top + ROW_HEIGHT + 4.0;
    let x = left;
    let mut y = first_row;
    for (index, (label, color)) in entries.iter().enumerate() {
        y = first_row + index as f64 * ROW_HEIGHT;
        let middle = y + ROW_HEIGHT / 2.0 - 3.0;
        legend = legend
            .child(
                Element::new("line")
                    .attr("class", "ibd-port-legend-swatch")
                    .attr_f("x1", x)
                    .attr_f("y1", middle)
                    .attr_f("x2", x + SWATCH)
                    .attr_f("y2", middle)
                    .style("stroke", *color)
                    .style("stroke-width", "3"),
            )
            .child(
                Element::new("text")
                    .attr("class", "ibd-port-legend-label")
                    .attr_f("x", x + SWATCH + GAP)
                    .attr_f("y", middle + FONT_SIZE / 2.0 - 1.0)
                    .style("font-size", format!("{FONT_SIZE}px"))
                    .style("fill", theme.text_secondary)
                    .text(label),
            );
    }
    let mut bottom = y + ROW_HEIGHT;
    if families.len() > PORT_FAMILY_COLOR_COUNT {
        legend = legend.child(
            Element::new("text")
                .attr("class", "ibd-port-legend-note")
                .attr_f("x", left)
                .attr_f("y", bottom + FONT_SIZE)
                .style("font-size", format!("{FONT_SIZE}px"))
                .style("fill", theme.text_secondary)
                .text(format!(
                    "Colours repeat: {} families share {PORT_FAMILY_COLOR_COUNT} colours.",
                    families.len()
                )),
        );
        bottom += ROW_HEIGHT;
    }
    Some((legend, bottom))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::LIGHT;
    use serde_json::json;

    fn render(meta: &Value) -> Option<String> {
        draw_port_family_legend(meta, &[], &LIGHT, (0.0, 0.0)).map(|(legend, _)| legend.to_string())
    }

    #[test]
    fn a_view_without_families_or_typed_connections_has_no_legend() {
        assert!(render(&json!({ "portFamilies": [] })).is_none());
        assert!(render(&json!({})).is_none());
    }

    #[test]
    fn more_families_than_colours_repeat_colours_and_say_so() {
        let families: Vec<String> = (0..PORT_FAMILY_COLOR_COUNT + 1)
            .map(|index| format!("Family{index}"))
            .collect();
        let legend = render(&json!({ "portFamilies": families })).expect("legend");
        assert_eq!(
            legend.matches("ibd-port-legend-label").count(),
            PORT_FAMILY_COLOR_COUNT + 1
        );
        assert!(legend.contains("Colours repeat: 13 families share 12 colours."));
        assert_eq!(
            port_family_color(&LIGHT, 0),
            port_family_color(&LIGHT, PORT_FAMILY_COLOR_COUNT)
        );
        let within = render(&json!({ "portFamilies": ["A", "B"] })).expect("legend");
        assert!(!within.contains("Colours repeat"));
    }
}
