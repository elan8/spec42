//! Native layout for the neutral ELK JSON graph emitted by Spec42's diagram builders.
//!
//! Spec42 authors edges on the root graph and requests root-relative edge coordinates. The pinned
//! `elkrs` revision can publish intra-container edges on their lowest common ancestor, so this
//! crate restores that observable contract before returning a result.

use std::collections::BTreeMap;

use serde_json::Value;
use thiserror::Error;

pub const ELKRS_REVISION: &str = "8309be8cf614cfe277c572b28e4f79a1703f8e32";
pub const ELK_COMPATIBILITY_BASELINE: &str = "ELK 0.11.0";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LayoutError {
    #[error("invalid ELK JSON input: {0}")]
    InvalidJson(String),
    #[error("ELK graph root must be a JSON object")]
    InvalidRoot,
    #[error("nested input node {node_id:?} contains edges; Spec42 authors edges on the root")]
    NestedInputEdges { node_id: Option<String> },
    #[error("elkrs layout failed: {0}")]
    Engine(String),
    #[error("elkrs returned an edge without an id")]
    MissingEdgeId,
    #[error("elkrs returned duplicate edge id {0}")]
    DuplicateEdgeId(String),
}

pub fn layout_json(input: &str) -> Result<Value, LayoutError> {
    let input: Value =
        serde_json::from_str(input).map_err(|error| LayoutError::InvalidJson(error.to_string()))?;
    layout_value(&input)
}

pub fn layout_value(input: &Value) -> Result<Value, LayoutError> {
    if !input.is_object() {
        return Err(LayoutError::InvalidRoot);
    }
    reject_nested_input_edges(input, true)?;
    let root_edge_order = input
        .get("edges")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|edge| edge.get("id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let input = serde_json::to_string(input).expect("JSON values always serialize");
    let output = elkrs::create_elk()
        .layout_json(&input)
        .map_err(LayoutError::Engine)?;
    normalize_output(output, &root_edge_order)
}

fn reject_nested_input_edges(node: &Value, is_root: bool) -> Result<(), LayoutError> {
    if !is_root
        && node
            .get("edges")
            .and_then(Value::as_array)
            .is_some_and(|edges| !edges.is_empty())
    {
        return Err(LayoutError::NestedInputEdges {
            node_id: node.get("id").and_then(Value::as_str).map(str::to_owned),
        });
    }
    if let Some(children) = node.get("children").and_then(Value::as_array) {
        for child in children {
            reject_nested_input_edges(child, false)?;
        }
    }
    Ok(())
}

fn normalize_output(mut output: Value, root_edge_order: &[String]) -> Result<Value, LayoutError> {
    if !output.is_object() {
        return Err(LayoutError::InvalidRoot);
    }
    let mut edges = BTreeMap::new();
    collect_root_relative_edges(&mut output, 0.0, 0.0, &mut edges)?;
    let mut ordered = Vec::with_capacity(edges.len());
    for id in root_edge_order {
        if let Some(edge) = edges.remove(id) {
            ordered.push(edge);
        }
    }
    ordered.extend(edges.into_values());
    output["edges"] = Value::Array(ordered);
    Ok(output)
}

fn collect_root_relative_edges(
    node: &mut Value,
    offset_x: f64,
    offset_y: f64,
    edges: &mut BTreeMap<String, Value>,
) -> Result<(), LayoutError> {
    if let Some(node_edges) = node.get_mut("edges").and_then(Value::as_array_mut) {
        for mut edge in std::mem::take(node_edges) {
            let id = edge
                .get("id")
                .and_then(Value::as_str)
                .ok_or(LayoutError::MissingEdgeId)?
                .to_string();
            translate_edge(&mut edge, offset_x, offset_y);
            if edges.insert(id.clone(), edge).is_some() {
                return Err(LayoutError::DuplicateEdgeId(id));
            }
        }
    }
    if let Some(children) = node.get_mut("children").and_then(Value::as_array_mut) {
        for child in children {
            let child_x = child.get("x").and_then(Value::as_f64).unwrap_or(0.0);
            let child_y = child.get("y").and_then(Value::as_f64).unwrap_or(0.0);
            collect_root_relative_edges(child, offset_x + child_x, offset_y + child_y, edges)?;
        }
    }
    Ok(())
}

fn translate_edge(edge: &mut Value, offset_x: f64, offset_y: f64) {
    if offset_x == 0.0 && offset_y == 0.0 {
        return;
    }
    if let Some(sections) = edge.get_mut("sections").and_then(Value::as_array_mut) {
        for section in sections {
            for key in ["startPoint", "endPoint"] {
                if let Some(point) = section.get_mut(key) {
                    translate_point(point, offset_x, offset_y);
                }
            }
            if let Some(points) = section.get_mut("bendPoints").and_then(Value::as_array_mut) {
                for point in points {
                    translate_point(point, offset_x, offset_y);
                }
            }
        }
    }
    if let Some(labels) = edge.get_mut("labels").and_then(Value::as_array_mut) {
        for label in labels {
            translate_coordinate(label, "x", offset_x);
            translate_coordinate(label, "y", offset_y);
        }
    }
    if let Some(points) = edge.get_mut("junctionPoints").and_then(Value::as_array_mut) {
        for point in points {
            translate_point(point, offset_x, offset_y);
        }
    }
}

fn translate_point(point: &mut Value, offset_x: f64, offset_y: f64) {
    translate_coordinate(point, "x", offset_x);
    translate_coordinate(point, "y", offset_y);
}

fn translate_coordinate(value: &mut Value, key: &str, offset: f64) {
    let Some(number) = value.get(key).and_then(Value::as_f64) else {
        return;
    };
    value[key] = Value::from(number + offset);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_malformed_json_and_non_object_roots() {
        assert!(matches!(layout_json("{"), Err(LayoutError::InvalidJson(_))));
        assert_eq!(layout_json("[]"), Err(LayoutError::InvalidRoot));
    }

    #[test]
    fn rejects_nested_authored_edges() {
        let input = serde_json::json!({
            "id": "root",
            "children": [{ "id": "container", "edges": [{ "id": "nested" }] }]
        });
        assert_eq!(
            layout_value(&input),
            Err(LayoutError::NestedInputEdges {
                node_id: Some("container".into())
            })
        );
    }

    #[test]
    fn lifts_container_relative_edge_geometry_to_the_root() {
        let output = serde_json::json!({
            "id": "root",
            "children": [{
                "id": "container", "x": 100.0, "y": 200.0,
                "edges": [{
                    "id": "edge",
                    "sections": [{
                        "startPoint": { "x": 1.0, "y": 2.0 },
                        "bendPoints": [{ "x": 3.0, "y": 4.0 }],
                        "endPoint": { "x": 5.0, "y": 6.0 }
                    }],
                    "labels": [{ "id": "label", "x": 7.0, "y": 8.0 }],
                    "junctionPoints": [{ "x": 9.0, "y": 10.0 }]
                }]
            }]
        });
        let normalized = normalize_output(output, &["edge".into()]).unwrap();
        let edge = &normalized["edges"][0];
        assert_eq!(edge["sections"][0]["startPoint"]["x"], 101.0);
        assert_eq!(edge["sections"][0]["bendPoints"][0]["y"], 204.0);
        assert_eq!(edge["sections"][0]["endPoint"]["x"], 105.0);
        assert_eq!(edge["labels"][0]["y"], 208.0);
        assert_eq!(edge["junctionPoints"][0]["x"], 109.0);
        assert!(normalized["children"][0]["edges"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn rejects_duplicate_output_edge_ids() {
        let output = serde_json::json!({
            "id": "root", "edges": [{ "id": "duplicate" }],
            "children": [{ "id": "container", "edges": [{ "id": "duplicate" }] }]
        });
        assert_eq!(
            normalize_output(output, &[]),
            Err(LayoutError::DuplicateEdgeId("duplicate".into()))
        );
    }

    #[test]
    fn lays_out_a_complete_graph() {
        let input = serde_json::json!({
            "id": "root", "layoutOptions": { "elk.algorithm": "layered" },
            "children": [
                { "id": "a", "width": 10, "height": 10 },
                { "id": "b", "width": 10, "height": 10 }
            ],
            "edges": [{ "id": "edge", "sources": ["a"], "targets": ["b"] }]
        });
        let output = layout_value(&input).unwrap();
        assert!(output["children"][0]["x"].is_number());
        assert!(output["edges"][0]["sections"][0]["startPoint"]["x"].is_number());
    }
}
