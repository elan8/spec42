//! Narrow compatibility adapter for the ELK JSON surface currently emitted by Spec42.
//!
//! Spec42 authors edges on the root graph and requests `elk.json.edgeCoords=ROOT`. At the pinned
//! revision, `elkrs` can move an edge whose endpoints share a nested container onto that container
//! and leave its section/label coordinates container-relative. ELK.js keeps those authored root
//! edges on the root and emits root-relative geometry. This adapter restores that observable
//! contract without changing the prepared-view boundary or inventing renderer semantics.

use std::collections::BTreeMap;

use serde_json::Value;

pub fn layout_json(input: &str) -> Result<Value, String> {
    let input_value: Value = serde_json::from_str(input)
        .map_err(|error| format!("Failed to parse ELK input before elkrs layout: {error}"))?;
    reject_nested_input_edges(&input_value, true)?;
    let root_edge_order = input_value
        .get("edges")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|edge| edge.get("id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut output = elkrs::create_elk().layout_json(input)?;
    let mut edges = BTreeMap::new();
    collect_root_relative_edges(&mut output, 0.0, 0.0, &mut edges)?;

    let mut ordered = Vec::with_capacity(edges.len());
    for id in root_edge_order {
        if let Some(edge) = edges.remove(&id) {
            ordered.push(edge);
        }
    }
    ordered.extend(edges.into_values());
    output["edges"] = Value::Array(ordered);
    Ok(output)
}

fn reject_nested_input_edges(node: &Value, is_root: bool) -> Result<(), String> {
    if !is_root
        && node
            .get("edges")
            .and_then(Value::as_array)
            .is_some_and(|edges| !edges.is_empty())
    {
        return Err(
            "elkrs compatibility adapter only accepts Spec42 graphs whose edges are authored on the root"
                .to_string(),
        );
    }
    if let Some(children) = node.get("children").and_then(Value::as_array) {
        for child in children {
            reject_nested_input_edges(child, false)?;
        }
    }
    Ok(())
}

fn collect_root_relative_edges(
    node: &mut Value,
    offset_x: f64,
    offset_y: f64,
    edges: &mut BTreeMap<String, Value>,
) -> Result<(), String> {
    if let Some(node_edges) = node.get_mut("edges").and_then(Value::as_array_mut) {
        for mut edge in std::mem::take(node_edges) {
            let id = edge
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| "elkrs returned an edge without an id".to_string())?
                .to_string();
            translate_edge(&mut edge, offset_x, offset_y);
            if edges.insert(id.clone(), edge).is_some() {
                return Err(format!("elkrs returned duplicate edge id {id}"));
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
    fn lifts_container_relative_edge_geometry_to_the_root() {
        let mut output = serde_json::json!({
            "id": "root",
            "children": [{
                "id": "container",
                "x": 100.0,
                "y": 200.0,
                "children": [{ "id": "nested", "x": 10.0, "y": 20.0 }],
                "edges": [{
                    "id": "edge",
                    "sections": [{
                        "startPoint": { "x": 1.0, "y": 2.0 },
                        "bendPoints": [{ "x": 3.0, "y": 4.0 }],
                        "endPoint": { "x": 5.0, "y": 6.0 }
                    }],
                    "labels": [{ "id": "label", "x": 7.0, "y": 8.0 }]
                }]
            }]
        });
        let mut edges = BTreeMap::new();
        collect_root_relative_edges(&mut output, 0.0, 0.0, &mut edges).unwrap();
        let edge = edges.get("edge").unwrap();
        assert_eq!(edge["sections"][0]["startPoint"]["x"], 101.0);
        assert_eq!(edge["sections"][0]["bendPoints"][0]["y"], 204.0);
        assert_eq!(edge["sections"][0]["endPoint"]["x"], 105.0);
        assert_eq!(edge["labels"][0]["y"], 208.0);
        assert!(output["children"][0]["edges"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn rejects_an_input_contract_the_adapter_does_not_own() {
        let input = serde_json::json!({
            "id": "root",
            "children": [{ "id": "container", "edges": [{ "id": "nested" }] }]
        });
        let error = reject_nested_input_edges(&input, true).unwrap_err();
        assert!(error.contains("edges are authored on the root"));
    }
}
