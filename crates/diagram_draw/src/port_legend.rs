//! Legend of an interconnection view's connector colours: one entry per port family of a drawn
//! connection, plus the neutral colour of connectors whose ports share no family.

use crate::ibd_edges::port_family_color;
use crate::svg::Element;
use crate::theme::{stroke_color_for_edge, Theme};
use crate::types::LaidOutEdge;
use serde_json::Value;
use std::collections::BTreeSet;

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
    // Only the families of drawn connections: a closed part hides the connections of its
    // contents, and a bus drawn as one port hides its signals' families.
    let mut drawn = BTreeSet::new();
    let mut unrelated = false;
    for edge in edges.iter().filter(typed_connection) {
        match edge.attributes.get("portFamily").and_then(Value::as_u64) {
            Some(family) => {
                drawn.insert(family as usize);
            }
            None => unrelated = true,
        }
    }
    // Each family keeps its index in the view's family order, and so its colour.
    let shown: Vec<(usize, &str)> = families
        .iter()
        .enumerate()
        .filter(|(index, _)| drawn.contains(index))
        .map(|(index, label)| (index, *label))
        .collect();
    if shown.is_empty() && !unrelated {
        return None;
    }

    let mut entries: Vec<(String, &'static str)> = shown
        .iter()
        .map(|(index, label)| (label.to_string(), port_family_color(theme, *index)))
        .collect();
    let colours = entries
        .iter()
        .map(|(_, color)| *color)
        .collect::<BTreeSet<_>>()
        .len();
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
    if colours < shown.len() {
        legend = legend.child(
            Element::new("text")
                .attr("class", "ibd-port-legend-note")
                .attr_f("x", left)
                .attr_f("y", bottom + FONT_SIZE)
                .style("font-size", format!("{FONT_SIZE}px"))
                .style("fill", theme.text_secondary)
                .text(format!(
                    "Colours repeat: {} families share {colours} colours.",
                    shown.len()
                )),
        );
        bottom += ROW_HEIGHT;
    }
    Some((legend, bottom))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibd_edges::PORT_FAMILY_COLOR_COUNT;
    use crate::theme::LIGHT;
    use serde_json::json;

    /// A drawn typed connection of `family`, or of no common family.
    fn connection(family: Option<usize>) -> LaidOutEdge {
        let mut attributes = json!({ "typedProjection": true });
        if let Some(family) = family {
            attributes["portFamily"] = json!(family);
        }
        serde_json::from_value(json!({
            "id": format!("e{family:?}"),
            "source": "a",
            "target": "b",
            "label": "",
            "edgeKind": "connection",
            "attributes": attributes,
            "layout": null,
        }))
        .expect("edge")
    }

    fn render(meta: &Value, edges: &[LaidOutEdge]) -> Option<String> {
        draw_port_family_legend(meta, edges, &LIGHT, (0.0, 0.0))
            .map(|(legend, _)| legend.to_string())
    }

    fn labels(legend: &str) -> Vec<String> {
        legend
            .split("class=\"ibd-port-legend-label\"")
            .skip(1)
            .map(|rest| {
                let text = &rest[rest.find('>').expect("text") + 1..];
                text[..text.find('<').expect("end")].to_string()
            })
            .collect()
    }

    #[test]
    fn a_view_without_families_or_typed_connections_has_no_legend() {
        assert!(render(&json!({ "portFamilies": [] }), &[]).is_none());
        assert!(render(&json!({}), &[]).is_none());
    }

    #[test]
    fn only_the_families_of_drawn_connections_are_listed() {
        let meta = json!({ "portFamilies": ["Bus", "Signal", "Power"] });
        // The signal family's connectors are hidden, for example inside a closed part.
        let legend = render(&meta, &[connection(Some(0)), connection(Some(2))]).expect("legend");
        assert_eq!(labels(&legend), ["Bus", "Power"]);
        // Each keeps the colour its connections are drawn in.
        assert!(legend.contains(port_family_color(&LIGHT, 2)));
        assert!(!legend.contains(port_family_color(&LIGHT, 1)));
        // No drawn connection has a family: no legend.
        assert!(render(&meta, &[]).is_none());
        let unrelated = render(&meta, &[connection(None)]).expect("legend");
        assert_eq!(labels(&unrelated), ["No common port type"]);
    }

    #[test]
    fn more_families_than_colours_repeat_colours_and_say_so() {
        let families: Vec<String> = (0..PORT_FAMILY_COLOR_COUNT + 1)
            .map(|index| format!("Family{index}"))
            .collect();
        let all: Vec<LaidOutEdge> = (0..families.len()).map(|f| connection(Some(f))).collect();
        let legend = render(&json!({ "portFamilies": families }), &all).expect("legend");
        assert_eq!(labels(&legend).len(), PORT_FAMILY_COLOR_COUNT + 1);
        assert!(legend.contains("Colours repeat: 13 families share 12 colours."));
        assert_eq!(
            port_family_color(&LIGHT, 0),
            port_family_color(&LIGHT, PORT_FAMILY_COLOR_COUNT)
        );
        // Drawing only families 1 to 12 uses twelve distinct colours.
        let within = render(&json!({ "portFamilies": families }), &all[1..]).expect("legend");
        assert!(!within.contains("Colours repeat"));
    }
}
