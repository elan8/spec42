//! Browser, Grid, and Geometry drawing (spec42 #181).
//!
//! Port of `views/standard-views-render.ts`. These views are tables / an indented membership
//! tree / a labelled 2D placeholder — they do not go through elkrs. Geometry stays a
//! provisional preview: Spec42 does not yet extract model-authored spatial coordinates.

use std::collections::HashSet;

use serde_json::Value;

use crate::behavior_common::{truncate_label, PreparedNode, PreparedView};
use crate::json_util::{as_array, as_string, field};
use crate::svg::{format_number as n, Element};
use crate::sysml_node::format_stereotype;
use crate::theme::Theme;

pub fn render_browser_view(
    prepared: &PreparedView,
    theme: &Theme,
    width: f64,
    _height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    let rows = source_rows(prepared);
    let hierarchy_layout = field(&prepared.meta, "hierarchyLayout") == &Value::Bool(true);
    let collapsed = collapsed_row_ids(prepared);
    let row_height = 28.0;
    let left = 52.0;
    let top = 88.0;
    let table_width = 520.0_f64.max(920.0_f64.min(width - 120.0));

    let mut root = Element::new("g").attr("class", "viz-root");
    root = root.child(title_text(prepared.title.as_str(), "Browser View", theme));
    if !hierarchy_layout {
        root = root.child(provisional_badge(theme, "provisional SysML notation"));
    }

    let mut layer = Element::new("g").attr("class", "browser-view-rows");
    let mut visible_index = 0usize;
    for (index, row) in rows.iter().enumerate() {
        if !row_is_visible(row, index, &rows, hierarchy_layout, &collapsed) {
            continue;
        }
        let y = top + visible_index as f64 * row_height;
        visible_index += 1;
        let depth = if hierarchy_layout {
            field(row, "depth").as_f64().unwrap_or(0.0)
        } else {
            let segments = as_string(field(row, "qualifiedName"), "")
                .split("::")
                .filter(|part| !part.is_empty())
                .count();
            segments.saturating_sub(1) as f64
        };
        let has_children = field(row, "hasChildren") == &Value::Bool(true);
        let row_id = node_id_from_meta(row, &prepared.nodes, index, "browser-row");
        let fill = if visible_index.is_multiple_of(2) {
            theme.node_fill
        } else {
            theme.canvas_background
        };
        let mut item = Element::new("g")
            .attr("class", "browser-row")
            .attr("data-node-id", &row_id)
            .attr("transform", format!("translate({},{})", n(left), n(y)))
            .child(
                Element::new("rect")
                    .attr("class", "node-background")
                    .attr("data-original-stroke", theme.node_border)
                    .attr("data-original-width", "1px")
                    .attr_f("width", table_width)
                    .attr_f("height", row_height - 3.0)
                    .attr("rx", "4")
                    .style("fill", fill)
                    .style("stroke", theme.node_border)
                    .style("stroke-width", "1px")
                    .style("opacity", "0.9"),
            );
        // A row whose members are not loaded yet opens by asking the host for its branch.
        let unloaded = as_string(field(row, "members"), "") == "unloaded";
        if has_children {
            let collapsed_row = unloaded || collapsed.contains(&row_id);
            item = item.child(
                Element::new("g")
                    .attr(
                        "class",
                        "browser-toggle general-node-toggle sysml-disclosure",
                    )
                    .maybe_attr(
                        "data-browser-load",
                        unloaded.then(|| as_string(field(row, "key"), "")),
                    )
                    .attr("role", "button")
                    .attr("tabindex", "0")
                    .attr(
                        "aria-expanded",
                        if collapsed_row { "false" } else { "true" },
                    )
                    .child(
                        Element::new("text")
                            .attr_f("x", 8.0 + depth * 16.0)
                            .attr_f("y", 18.0)
                            .style("font-size", "11px")
                            .style("font-weight", "700")
                            .style("fill", theme.text_secondary)
                            .style("cursor", "pointer")
                            .text(if collapsed_row { "▸" } else { "▾" }),
                    ),
            );
        }
        let visibility_prefix = visibility_glyph(&as_string(field(row, "visibility"), ""));
        // SysML 8.2.3.6 (`el-prefix`, `usage-name-with-alias`): an inherited usage is shown with
        // a caret before its name.
        let inherited = if as_string(field(row, "membership"), "") == "inherited" {
            "^"
        } else {
            ""
        };
        let label = format!("{inherited}{}", as_string(first_row_label(row), "Unnamed"));
        item = item.child(
            Element::new("text")
                .attr_f(
                    "x",
                    14.0 + depth * 16.0 + if has_children { 12.0 } else { 0.0 },
                )
                .attr_f("y", 18.0)
                .style("font-size", "11px")
                .style("font-weight", "600")
                .style("fill", theme.text_primary)
                .text(format!("{visibility_prefix}{}", truncate_label(&label, 48))),
        );
        item = item.child(
            Element::new("text")
                .attr_f("x", table_width - 14.0)
                .attr_f("y", 18.0)
                .attr("text-anchor", "end")
                .style("font-size", "10px")
                .style("fill", theme.text_secondary)
                .text(format_stereotype(&truncate_label(
                    &as_string(field(row, "kind"), "element"),
                    24,
                ))),
        );
        layer = layer.child(item);
    }
    root = root.child(layer);
    (
        root,
        (
            0.0,
            0.0,
            left + table_width + 80.0,
            top + visible_index as f64 * row_height + 80.0,
        ),
    )
}

pub fn render_grid_view(
    prepared: &PreparedView,
    theme: &Theme,
    _width: f64,
    _height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    if field(&prepared.meta, "relationshipMatrix") == &Value::Bool(true) {
        return render_relationship_matrix(prepared, theme);
    }

    let cells = source_cells(prepared);
    let left = 52.0;
    let top = 92.0;
    let columns = grid_columns(prepared);
    let table_width: f64 = columns.iter().map(|column| column.width).sum();
    let row_height = 30.0;

    let mut root = Element::new("g").attr("class", "viz-root");
    root = root.child(title_text(prepared.title.as_str(), "Grid View", theme));
    if field(&prepared.meta, "provisional") == &Value::Bool(true) {
        root = root.child(provisional_badge(theme, "provisional SysML notation"));
    }

    let mut table = Element::new("g")
        .attr("class", "grid-view-table")
        .attr("transform", format!("translate({},{})", n(left), n(top)));
    let mut x = 0.0;
    for column in &columns {
        table = table
            .child(
                Element::new("rect")
                    .attr("class", "grid-header-cell")
                    .attr_f("x", x)
                    .attr_f("width", column.width)
                    .attr_f("height", row_height)
                    .style("fill", theme.node_border)
                    .style("stroke", theme.node_border),
            )
            .child(
                Element::new("text")
                    .attr_f("x", x + 10.0)
                    .attr_f("y", 20.0)
                    .style("font-size", "11px")
                    .style("font-weight", "700")
                    .style("fill", theme.canvas_background)
                    .text(column.label.clone()),
            );
        x += column.width;
    }
    for (row_index, row) in cells.iter().enumerate() {
        let mut group = Element::new("g")
            .attr("class", "grid-row")
            .attr(
                "data-node-id",
                node_id_from_meta(row, &prepared.nodes, row_index, "grid-row"),
            )
            .attr(
                "transform",
                format!("translate(0,{})", n((row_index + 1) as f64 * row_height)),
            );
        x = 0.0;
        for column in &columns {
            let cell_text = as_string(field(row, &column.key), "");
            let max_chars = if column.width > 100.0 { 28 } else { 8 };
            let mut text = Element::new("text")
                .attr_f("x", x + 10.0)
                .attr_f("y", 20.0)
                .style("font-size", "10px")
                .style("fill", theme.text_primary)
                .text(truncate_label(&cell_text, max_chars));
            if !column.notation_status.is_empty() {
                text = text.attr("data-notation-status", column.notation_status.clone());
            }
            group = group
                .child(
                    Element::new("rect")
                        .attr("class", "grid-cell")
                        .attr_f("x", x)
                        .attr_f("width", column.width)
                        .attr_f("height", row_height)
                        .style(
                            "fill",
                            if row_index.is_multiple_of(2) {
                                theme.node_fill
                            } else {
                                theme.canvas_background
                            },
                        )
                        .style("stroke", theme.node_border)
                        .style("stroke-width", "1px"),
                )
                .child(text);
            x += column.width;
        }
        table = table.child(group);
    }
    root = root.child(table);
    (
        root,
        (
            0.0,
            0.0,
            left + table_width + 80.0,
            top + (cells.len() + 2) as f64 * row_height + 80.0,
        ),
    )
}

/// A row or column of a relationship matrix: the node it is and the name it is headed with.
struct MatrixHeader {
    id: String,
    label: String,
}

fn matrix_headers(meta: &Value, key: &str, legacy_key: &str) -> Vec<MatrixHeader> {
    let typed = as_array(field(meta, key));
    if !typed.is_empty() {
        return typed
            .iter()
            .map(|entry| MatrixHeader {
                id: as_string(field(entry, "id"), ""),
                label: as_string(field(entry, "label"), ""),
            })
            .filter(|header| !header.id.is_empty())
            .collect();
    }
    as_array(field(meta, legacy_key))
        .iter()
        .map(|value| as_string(value, ""))
        .filter(|id| !id.is_empty())
        .map(|id| MatrixHeader {
            label: short_matrix_label(&id),
            id,
        })
        .collect()
}

/// The keyword a relationship kind is shown with, as on its edge in a General View.
fn matrix_kind_keyword(kind: &str) -> String {
    match kind {
        "" => String::new(),
        "specializes" | "specialization" => "specializes".into(),
        other => format!("«{other}»"),
    }
}

/// A relationship matrix (SysML v2 9.2.20.2.5): rows are the elements at the source end of the
/// relationships, columns those at the target end, and a cell holds one dot for each relationship
/// from its row to its column, with the relationship kinds as its tooltip. A legend under the
/// title names the kinds the matrix shows.
fn render_relationship_matrix(
    prepared: &PreparedView,
    theme: &Theme,
) -> (Element, (f64, f64, f64, f64)) {
    let rows = matrix_headers(&prepared.meta, "matrixRows", "matrixRowIds");
    let columns = matrix_headers(&prepared.meta, "matrixColumns", "matrixColIds");
    let matrix_cells = as_array(field(&prepared.meta, "matrixCells"));
    // The relationships from each row to each column: their kinds and their edges.
    let relationships = |row: &str, column: &str| -> Vec<(String, String)> {
        matrix_cells
            .iter()
            .filter(|entry| {
                as_string(field(entry, "source"), "") == row
                    && as_string(field(entry, "target"), "") == column
            })
            .flat_map(|entry| {
                let edge = as_string(field(entry, "edgeId"), "");
                let labels = as_array(field(entry, "labels"));
                if labels.is_empty() {
                    vec![(as_string(field(entry, "kind"), ""), edge)]
                } else {
                    labels
                        .iter()
                        .map(|label| (as_string(label, ""), edge.clone()))
                        .collect()
                }
            })
            .collect()
    };
    const CHARACTER: f64 = 6.2;
    const LABEL_LIMIT: usize = 28;
    let label_width =
        |header: &MatrixHeader| header.label.chars().count().min(LABEL_LIMIT) as f64 * CHARACTER;
    let cell_size = 36.0;
    // Room for the longest row name, and for the longest column name written upwards at 45°.
    let row_header = rows.iter().map(label_width).fold(60.0_f64, f64::max) + 16.0;
    let column_header = columns.iter().map(label_width).fold(40.0_f64, f64::max)
        * std::f64::consts::FRAC_1_SQRT_2
        + 24.0;
    let left = 40.0;
    let mut kinds: Vec<String> = matrix_cells
        .iter()
        .flat_map(|entry| {
            let labels = as_array(field(entry, "labels"));
            if labels.is_empty() {
                vec![as_string(field(entry, "kind"), "")]
            } else {
                labels.iter().map(|label| as_string(label, "")).collect()
            }
        })
        .filter(|kind| !kind.is_empty())
        .collect();
    kinds.sort();
    kinds.dedup();
    let legend_top = 76.0;
    let top = legend_top + 24.0;

    let mut root = Element::new("g").attr("class", "viz-root");
    root = root.child(title_text(
        prepared.title.as_str(),
        "Relationship Matrix",
        theme,
    ));
    if !kinds.is_empty() {
        root = root.child(
            Element::new("text")
                .attr("class", "grid-relationship-matrix-legend")
                .attr_f("x", left)
                .attr_f("y", legend_top)
                .style("font-size", "11px")
                .style("fill", theme.text_secondary)
                .text(format!(
                    "rows: source · columns: target · ● {}",
                    kinds
                        .iter()
                        .map(|kind| matrix_kind_keyword(kind))
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
        );
    }

    let mut layer = Element::new("g")
        .attr("class", "grid-relationship-matrix")
        .attr("transform", format!("translate({},{})", n(left), n(top)));
    for (index, column) in columns.iter().enumerate() {
        let x = row_header + index as f64 * cell_size + cell_size / 2.0;
        let y = column_header - 6.0;
        layer = layer.child(
            Element::new("text")
                .attr("class", "grid-relationship-matrix-column")
                .attr("data-node-id", column.id.clone())
                .attr_f("x", x)
                .attr_f("y", y)
                .attr("transform", format!("rotate(-45, {}, {})", n(x), n(y)))
                .style("font-size", "10px")
                .style("fill", theme.text_primary)
                .text(truncate_label(&column.label, LABEL_LIMIT)),
        );
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = column_header + row_index as f64 * cell_size;
        layer = layer.child(
            Element::new("text")
                .attr("class", "grid-relationship-matrix-row")
                .attr("data-node-id", row.id.clone())
                .attr_f("x", row_header - 8.0)
                .attr_f("y", y + cell_size / 2.0 + 4.0)
                .attr("text-anchor", "end")
                .style("font-size", "10px")
                .style("fill", theme.text_primary)
                .text(truncate_label(&row.label, LABEL_LIMIT)),
        );
        for (column_index, column) in columns.iter().enumerate() {
            let entries = relationships(&row.id, &column.id);
            let present = !entries.is_empty();
            let x = row_header + column_index as f64 * cell_size;
            let mut cell_group = Element::new("g")
                .attr("class", "grid-relationship-matrix-cell")
                .attr("data-row-id", row.id.clone())
                .attr("data-col-id", column.id.clone())
                .child(
                    Element::new("rect")
                        .attr_f("x", x)
                        .attr_f("y", y)
                        .attr_f("width", cell_size - 2.0)
                        .attr_f("height", cell_size - 2.0)
                        .style(
                            "fill",
                            if present {
                                theme.node_fill
                            } else {
                                theme.canvas_background
                            },
                        )
                        .style("stroke", theme.node_border)
                        .style("stroke-width", "1px"),
                );
            if present {
                let edges: Vec<&str> = entries
                    .iter()
                    .map(|(_, edge)| edge.as_str())
                    .filter(|edge| !edge.is_empty())
                    .collect();
                if !edges.is_empty() {
                    cell_group = cell_group.attr("data-edge-ids", edges.join(" "));
                }
                let dot_spacing = 10.0;
                let dots_width = (entries.len().saturating_sub(1)) as f64 * dot_spacing;
                let start_x = x + (cell_size - 2.0) / 2.0 - dots_width / 2.0;
                for index in 0..entries.len() {
                    cell_group = cell_group.child(
                        Element::new("text")
                            .attr_f("x", start_x + index as f64 * dot_spacing)
                            .attr_f("y", y + (cell_size - 2.0) / 2.0 + 4.0)
                            .attr("text-anchor", "middle")
                            .style("font-size", "12px")
                            .style("font-weight", "700")
                            .style("fill", theme.edge_default)
                            .text("●"),
                    );
                }
                let tooltip = entries
                    .iter()
                    .map(|(kind, _)| matrix_kind_keyword(kind))
                    .collect::<Vec<_>>()
                    .join(", ");
                cell_group = cell_group.child(
                    Element::new("title")
                        .text(format!("{} → {}: {tooltip}", row.label, column.label)),
                );
            }
            layer = layer.child(cell_group);
        }
    }
    root = root.child(layer);
    // The last column's name, written upwards at 45°, reaches beyond the last cell.
    let overhang = columns.last().map_or(0.0, |column| {
        label_width(column) * std::f64::consts::FRAC_1_SQRT_2
    });
    let width = row_header + columns.len() as f64 * cell_size + overhang + 40.0;
    let height = column_header + rows.len() as f64 * cell_size + 40.0;
    let legend_width = 300.0;
    (
        root,
        (0.0, 0.0, left + width.max(legend_width), top + height),
    )
}

pub fn render_geometry_view(
    prepared: &PreparedView,
    theme: &Theme,
    _width: f64,
    _height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    let nodes = source_elements(prepared);
    let geometry_mode = as_string(field(&prepared.meta, "geometryMode"), "2d");
    let geometry_projection =
        as_string(field(&prepared.meta, "geometryProjection"), "orthographic");
    let left = 64.0;
    let top = 88.0;
    let cell_width = 128.0;
    let cell_height = 72.0;
    let columns = 1.0_f64.max((nodes.len() as f64).sqrt().ceil());
    let column_count = columns as usize;

    let mut root = Element::new("g").attr("class", "viz-root");
    root = root.child(title_text(prepared.title.as_str(), "Geometry View", theme));
    if field(&prepared.meta, "provisional") == &Value::Bool(true) {
        root = root.child(provisional_badge(
            theme,
            &format!("{geometry_mode} {geometry_projection} preview"),
        ));
    }

    let rows = if column_count == 0 {
        0.0
    } else {
        (nodes.len() as f64 / columns).ceil()
    };
    let mut layer = Element::new("g")
        .attr("class", "geometry-view-scene")
        .attr("transform", format!("translate({},{})", n(left), n(top)))
        .child(
            Element::new("rect")
                .attr_f("width", columns * cell_width + 24.0)
                .attr_f("height", rows * cell_height + 24.0)
                .attr("rx", "8")
                .style("fill", "none")
                .style("stroke", theme.frame_stroke)
                .style("stroke-dasharray", "8,6"),
        );
    for (index, node) in nodes.iter().enumerate() {
        let col = (index % column_count.max(1)) as f64;
        let row = (index / column_count.max(1)) as f64;
        let x = col * cell_width + 12.0;
        let y = row * cell_height + 12.0;
        let node_id = node_id_from_meta(node, &prepared.nodes, index, "geometry-node");
        let label = as_string(first_row_label(node), "");
        layer = layer.child(
            Element::new("g")
                .attr("class", "geometry-object")
                .attr("data-node-id", node_id)
                .attr("transform", format!("translate({},{})", n(x), n(y)))
                .child(
                    Element::new("rect")
                        .attr("class", "node-background")
                        .attr("data-original-stroke", theme.node_border)
                        .attr("data-original-width", "1.5px")
                        .attr_f("width", cell_width - 20.0)
                        .attr_f("height", cell_height - 16.0)
                        .attr("rx", "6")
                        .style("fill", theme.node_fill)
                        .style("stroke", theme.node_border)
                        .style("stroke-width", "1.5px"),
                )
                .child(
                    Element::new("text")
                        .attr_f("x", (cell_width - 20.0) / 2.0)
                        .attr_f("y", 24.0)
                        .attr("text-anchor", "middle")
                        .style("font-size", "10px")
                        .style("font-weight", "700")
                        .style("fill", theme.text_primary)
                        .text(truncate_label(&label, 16)),
                )
                .child(
                    Element::new("text")
                        .attr_f("x", (cell_width - 20.0) / 2.0)
                        .attr_f("y", 42.0)
                        .attr("text-anchor", "middle")
                        .style("font-size", "8px")
                        .style("fill", theme.text_secondary)
                        .text(truncate_label(
                            &as_string(field(node, "kind"), "element"),
                            18,
                        )),
                ),
        );
    }
    root = root.child(layer);
    (
        root,
        (
            0.0,
            0.0,
            left + columns * cell_width + 80.0,
            top + rows * cell_height + 120.0,
        ),
    )
}

fn title_text(title: &str, fallback: &str, theme: &Theme) -> Element {
    let text = if title.is_empty() { fallback } else { title };
    Element::new("text")
        .attr_f("x", 24.0)
        .attr_f("y", 28.0)
        .style("font-size", "14px")
        .style("font-weight", "700")
        .style("fill", theme.text_primary)
        .text(text)
}

fn provisional_badge(theme: &Theme, label: &str) -> Element {
    Element::new("g")
        .attr("class", "provisional-view-badge")
        .child(
            Element::new("rect")
                .attr_f("x", 22.0)
                .attr_f("y", 42.0)
                .attr_f("width", 176.0)
                .attr_f("height", 24.0)
                .attr("rx", "5")
                .style("fill", theme.canvas_background)
                .style("stroke", theme.edge_default)
                .style("stroke-dasharray", "4,3"),
        )
        .child(
            Element::new("text")
                .attr_f("x", 34.0)
                .attr_f("y", 58.0)
                .style("font-size", "10px")
                .style("fill", theme.text_secondary)
                .text(label),
        )
}

fn visibility_glyph(visibility: &str) -> &'static str {
    match visibility {
        "Public" => "+ ",
        "Private" => "- ",
        "Protected" => "# ",
        _ => "",
    }
}

fn short_matrix_label(id: &str) -> String {
    let last = id.split("::").filter(|part| !part.is_empty()).last();
    truncate_label(last.unwrap_or(id), 10)
}

fn first_row_label(row: &Value) -> &Value {
    let label = field(row, "label");
    if !label.is_null() {
        return label;
    }
    let name = field(row, "name");
    if !name.is_null() {
        return name;
    }
    field(row, "id")
}

fn node_id_from_meta(
    row: &Value,
    nodes: &[PreparedNode],
    index: usize,
    fallback_prefix: &str,
) -> String {
    let id = as_string(field(row, "id"), "");
    let label = as_string(first_row_label(row), "");
    if let Some(node) = nodes
        .iter()
        .find(|node| node.id == id || node.label == label)
    {
        return node.id.clone();
    }
    if id.is_empty() {
        format!("{fallback_prefix}-{index}")
    } else {
        id
    }
}

fn source_rows(prepared: &PreparedView) -> Vec<Value> {
    let rows = as_array(field(&prepared.meta, "rows"));
    if !rows.is_empty() {
        return rows.to_vec();
    }
    prepared
        .nodes
        .iter()
        .map(|node| {
            serde_json::json!({
                "id": node.id,
                "label": node.label,
                "kind": node.kind,
            })
        })
        .collect()
}

fn source_cells(prepared: &PreparedView) -> Vec<Value> {
    let cells = as_array(field(&prepared.meta, "cells"));
    if !cells.is_empty() {
        return cells.to_vec();
    }
    prepared
        .nodes
        .iter()
        .map(|node| {
            Value::Object(
                node.attributes
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            )
        })
        .collect()
}

fn source_elements(prepared: &PreparedView) -> Vec<Value> {
    let elements = as_array(field(&prepared.meta, "elements"));
    if !elements.is_empty() {
        return elements.to_vec();
    }
    prepared
        .nodes
        .iter()
        .map(|node| {
            serde_json::json!({
                "id": node.id,
                "label": node.label,
                "kind": node.kind,
            })
        })
        .collect()
}

fn collapsed_row_ids(prepared: &PreparedView) -> HashSet<String> {
    as_array(field(&prepared.meta, "collapsedRowIds"))
        .iter()
        .map(|value| as_string(value, ""))
        .filter(|value| !value.is_empty())
        .collect()
}

fn row_is_visible(
    row: &Value,
    index: usize,
    source_rows: &[Value],
    hierarchy_layout: bool,
    collapsed: &HashSet<String>,
) -> bool {
    if !hierarchy_layout {
        return true;
    }
    let parent_id = as_string(field(row, "parentId"), "");
    if parent_id.is_empty() {
        return true;
    }
    for cursor in (0..index).rev() {
        let ancestor = &source_rows[cursor];
        if as_string(field(ancestor, "id"), "") != parent_id {
            continue;
        }
        return row_is_visible(ancestor, cursor, source_rows, hierarchy_layout, collapsed)
            && !collapsed.contains(&parent_id);
    }
    !collapsed.contains(&parent_id)
}

struct GridColumn {
    key: String,
    label: String,
    notation_status: String,
    width: f64,
}

fn grid_columns(prepared: &PreparedView) -> Vec<GridColumn> {
    let column_views = as_array(field(&prepared.meta, "columns"));
    if column_views.is_empty() {
        return vec![
            GridColumn {
                key: "name".into(),
                label: "Name".into(),
                notation_status: String::new(),
                width: 220.0,
            },
            GridColumn {
                key: "kind".into(),
                label: "Kind".into(),
                notation_status: String::new(),
                width: 150.0,
            },
            GridColumn {
                key: "attributeCount".into(),
                label: "Attrs".into(),
                notation_status: String::new(),
                width: 80.0,
            },
            GridColumn {
                key: "partCount".into(),
                label: "Parts".into(),
                notation_status: String::new(),
                width: 80.0,
            },
            GridColumn {
                key: "portCount".into(),
                label: "Ports".into(),
                notation_status: String::new(),
                width: 80.0,
            },
        ];
    }
    column_views
        .iter()
        .map(|column| GridColumn {
            key: as_string(field(column, "key"), "name"),
            label: as_string(field(column, "label"), "Column"),
            notation_status: as_string(field(column, "notationStatus"), ""),
            width: 220.0,
        })
        .collect()
}
