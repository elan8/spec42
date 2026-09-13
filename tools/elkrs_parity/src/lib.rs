//! Geometry-scalar parity comparison shared by the `elkrs_parity` CLI and the fast in-process
//! shadow-corpus test (`crates/server/tests/integration/layout_shadow_corpus.rs`).
//!
//! ELK.js and elkrs disagree on incidental JSON shape (which container an intra-hierarchy edge is
//! attached to, which internal solver-state layout options get echoed back per node), so plain
//! `serde_json::Value` equality is not a meaningful parity signal. What Spec42 actually consumes is
//! node/port/label rectangles and routed edge-section points; this module extracts exactly those
//! scalars from either engine's output and compares them within a numeric tolerance.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Default, Serialize)]
pub struct GeometryCounts {
    pub nodes: usize,
    pub ports: usize,
    pub labels: usize,
    pub edge_sections: usize,
    pub bend_points: usize,
    pub scalars: usize,
}

#[derive(Debug, Serialize)]
pub struct Comparison {
    pub status: ComparisonStatus,
    pub compared_scalars: usize,
    pub missing_from_elkjs: usize,
    pub missing_from_elkrs: usize,
    pub changed_scalars: usize,
    pub max_absolute_delta: f64,
    pub differences: Vec<GeometryDifference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonStatus {
    Exact,
    WithinTolerance,
    Different,
    EngineError,
}

#[derive(Debug, Serialize)]
pub struct GeometryDifference {
    pub path: String,
    pub elkjs: Option<f64>,
    pub elkrs: Option<f64>,
    pub absolute_delta: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Geometry {
    pub values: BTreeMap<String, f64>,
    pub counts: GeometryCounts,
}

/// A digest that's stable across equivalent JSON regardless of formatting or object-key order,
/// used to detect nondeterministic layout output across repeated runs.
///
/// This explicit re-sort is necessary, not redundant: the pinned `elkrs` dependency requests
/// serde_json's `preserve_order` feature (`cargo tree -e features -i serde_json` shows the edge:
/// `elkrs -> serde_json feature "preserve_order"`), and Cargo's feature unification applies that
/// workspace-wide — so `serde_json::Value::Object` here is insertion-order-preserving, not the
/// `BTreeMap`-backed default serde_json ships without that feature. Without this canonicalization
/// step, two structurally-identical layout outputs that merely built their JSON objects in a
/// different key order would hash differently and be misreported as nondeterministic.
pub fn canonical_json_digest(value: &Value) -> String {
    fn canonicalize(value: &Value) -> Value {
        match value {
            Value::Object(object) => {
                let sorted = object
                    .iter()
                    .map(|(key, value)| (key.clone(), canonicalize(value)))
                    .collect::<BTreeMap<_, _>>();
                Value::Object(sorted.into_iter().collect())
            }
            Value::Array(items) => Value::Array(items.iter().map(canonicalize).collect()),
            other => other.clone(),
        }
    }

    let bytes = serde_json::to_vec(&canonicalize(value)).expect("JSON value serializes");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Debug, Default)]
struct Topology {
    nodes: BTreeSet<String>,
    ports: BTreeSet<String>,
    labels: BTreeSet<String>,
    edges: BTreeSet<String>,
}

/// Confirms the output names exactly the nodes/ports/labels/edges the input authored, and (when
/// `require_geometry` matters for `output`) that every rectangle and edge section is fully
/// numeric. Used to reject a technically-valid-JSON output that silently dropped or duplicated
/// part of the graph.
pub fn validate_layout_output(input: &Value, output: &Value) -> Vec<String> {
    let mut expected = Topology::default();
    collect_topology(input, true, false, &mut expected, &mut Vec::new());
    let mut observed = Topology::default();
    let mut errors = Vec::new();
    collect_topology(output, true, true, &mut observed, &mut errors);

    compare_identity_set("node", &expected.nodes, &observed.nodes, &mut errors);
    compare_identity_set("port", &expected.ports, &observed.ports, &mut errors);
    compare_identity_set("label", &expected.labels, &observed.labels, &mut errors);
    compare_identity_set("edge", &expected.edges, &observed.edges, &mut errors);
    errors.sort();
    errors.dedup();
    errors
}

fn compare_identity_set(
    kind: &str,
    expected: &BTreeSet<String>,
    observed: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    for id in expected.difference(observed) {
        errors.push(format!("missing {kind} {id}"));
    }
    for id in observed.difference(expected) {
        errors.push(format!("unexpected {kind} {id}"));
    }
}

fn collect_topology(
    value: &Value,
    is_root: bool,
    require_geometry: bool,
    topology: &mut Topology,
    errors: &mut Vec<String>,
) {
    if !is_root {
        collect_identified_rect(value, "node", require_geometry, &mut topology.nodes, errors);
    }
    collect_labels(value, require_geometry, topology, errors);
    if let Some(ports) = value.get("ports").and_then(Value::as_array) {
        for port in ports {
            collect_identified_rect(port, "port", require_geometry, &mut topology.ports, errors);
            collect_labels(port, require_geometry, topology, errors);
        }
    }
    if let Some(edges) = value.get("edges").and_then(Value::as_array) {
        for edge in edges {
            let Some(id) = edge.get("id").and_then(Value::as_str) else {
                errors.push("edge without id".to_string());
                continue;
            };
            if !topology.edges.insert(id.to_string()) {
                errors.push(format!("duplicate edge {id}"));
            }
            collect_labels(edge, require_geometry, topology, errors);
            if require_geometry {
                validate_edge_sections(edge, id, errors);
            }
        }
    }
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        for child in children {
            collect_topology(child, false, require_geometry, topology, errors);
        }
    }
}

fn collect_labels(
    value: &Value,
    require_geometry: bool,
    topology: &mut Topology,
    errors: &mut Vec<String>,
) {
    if let Some(labels) = value.get("labels").and_then(Value::as_array) {
        for label in labels {
            collect_identified_rect(
                label,
                "label",
                require_geometry,
                &mut topology.labels,
                errors,
            );
        }
    }
}

fn collect_identified_rect(
    value: &Value,
    kind: &str,
    require_geometry: bool,
    identities: &mut BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let Some(id) = value.get("id").and_then(Value::as_str) else {
        errors.push(format!("{kind} without id"));
        return;
    };
    if !identities.insert(id.to_string()) {
        errors.push(format!("duplicate {kind} {id}"));
    }
    if require_geometry {
        for coordinate in ["x", "y", "width", "height"] {
            if value.get(coordinate).and_then(Value::as_f64).is_none() {
                errors.push(format!("{kind} {id} missing numeric {coordinate}"));
            }
        }
    }
}

fn validate_edge_sections(edge: &Value, id: &str, errors: &mut Vec<String>) {
    let Some(sections) = edge.get("sections").and_then(Value::as_array) else {
        errors.push(format!("edge {id} missing sections"));
        return;
    };
    if sections.is_empty() {
        errors.push(format!("edge {id} has no sections"));
    }
    for (index, section) in sections.iter().enumerate() {
        for point_name in ["startPoint", "endPoint"] {
            let point = section.get(point_name);
            for coordinate in ["x", "y"] {
                if point
                    .and_then(|point| point.get(coordinate))
                    .and_then(Value::as_f64)
                    .is_none()
                {
                    errors.push(format!(
                        "edge {id} section {index} missing numeric {point_name}.{coordinate}"
                    ));
                }
            }
        }
    }
}

pub fn extract_geometry(root: &Value) -> Geometry {
    let mut geometry = Geometry::default();
    visit_graph(root, "graph", &mut geometry);
    geometry
}

fn visit_graph(value: &Value, path: &str, geometry: &mut Geometry) {
    record_rect(value, path, geometry);
    if path == "graph" || value.get("children").is_some() {
        geometry.counts.nodes += 1;
    }

    visit_named_array(
        value,
        "labels",
        path,
        "label",
        geometry,
        |item, item_path, geometry| {
            geometry.counts.labels += 1;
            record_rect(item, item_path, geometry);
        },
    );
    visit_named_array(
        value,
        "ports",
        path,
        "port",
        geometry,
        |port, port_path, geometry| {
            geometry.counts.ports += 1;
            record_rect(port, port_path, geometry);
            visit_named_array(
                port,
                "labels",
                port_path,
                "label",
                geometry,
                |item, item_path, geometry| {
                    geometry.counts.labels += 1;
                    record_rect(item, item_path, geometry);
                },
            );
        },
    );
    // ELK may publish an edge on the root or on its lowest-common-ancestor container. The edge id
    // is the stable identity consumed by Spec42, so compare the same edge directly even when the
    // engines disagree about its owning JSON object or coordinate frame.
    if let Some(edges) = value.get("edges").and_then(Value::as_array) {
        for (index, edge) in edges.iter().enumerate() {
            let id = edge.get("id").and_then(Value::as_str).unwrap_or("");
            let suffix = if id.is_empty() {
                format!("{path}:{index}")
            } else {
                escape_path(id)
            };
            visit_edge(edge, &format!("edge:{suffix}"), geometry);
        }
    }
    visit_named_array(value, "children", path, "node", geometry, visit_graph);
}

fn visit_edge(edge: &Value, path: &str, geometry: &mut Geometry) {
    visit_named_array(
        edge,
        "labels",
        path,
        "label",
        geometry,
        |item, item_path, geometry| {
            geometry.counts.labels += 1;
            record_rect(item, item_path, geometry);
        },
    );
    let Some(sections) = edge.get("sections").and_then(Value::as_array) else {
        return;
    };
    for (index, section) in sections.iter().enumerate() {
        geometry.counts.edge_sections += 1;
        let section_path = format!("{path}/section:{index}");
        record_point(
            section.get("startPoint"),
            &format!("{section_path}/start"),
            geometry,
        );
        record_point(
            section.get("endPoint"),
            &format!("{section_path}/end"),
            geometry,
        );
        if let Some(points) = section.get("bendPoints").and_then(Value::as_array) {
            for (point_index, point) in points.iter().enumerate() {
                geometry.counts.bend_points += 1;
                record_point(
                    Some(point),
                    &format!("{section_path}/bend:{point_index}"),
                    geometry,
                );
            }
        }
    }
}

fn visit_named_array<F>(
    value: &Value,
    key: &str,
    parent_path: &str,
    kind: &str,
    geometry: &mut Geometry,
    mut visit: F,
) where
    F: FnMut(&Value, &str, &mut Geometry),
{
    let Some(items) = value.get(key).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        let id = item.get("id").and_then(Value::as_str).unwrap_or("");
        let suffix = if id.is_empty() {
            index.to_string()
        } else {
            escape_path(id)
        };
        let item_path = format!("{parent_path}/{kind}:{suffix}");
        visit(item, &item_path, geometry);
    }
}

fn escape_path(value: &str) -> String {
    value.replace('%', "%25").replace('/', "%2F")
}

fn record_rect(value: &Value, path: &str, geometry: &mut Geometry) {
    for key in ["x", "y", "width", "height"] {
        record_number(value.get(key), &format!("{path}/{key}"), geometry);
    }
}

fn record_point(value: Option<&Value>, path: &str, geometry: &mut Geometry) {
    let Some(value) = value else { return };
    for key in ["x", "y"] {
        record_number(value.get(key), &format!("{path}/{key}"), geometry);
    }
}

fn record_number(value: Option<&Value>, path: &str, geometry: &mut Geometry) {
    if let Some(number) = value.and_then(Value::as_f64) {
        geometry.values.insert(path.to_string(), number);
        geometry.counts.scalars += 1;
    }
}

pub fn compare_geometry(
    elkjs: &BTreeMap<String, f64>,
    elkrs: &BTreeMap<String, f64>,
    tolerance: f64,
) -> Comparison {
    let paths: BTreeSet<_> = elkjs.keys().chain(elkrs.keys()).collect();
    let mut differences = Vec::new();
    let mut compared_scalars = 0;
    let mut missing_from_elkjs = 0;
    let mut missing_from_elkrs = 0;
    let mut changed_scalars = 0;
    let mut max_absolute_delta: f64 = 0.0;
    let mut has_nonzero_delta = false;

    for path in paths {
        let left = elkjs.get(path).copied();
        let right = elkrs.get(path).copied();
        match (left, right) {
            (Some(elkjs_value), Some(elkrs_value)) => {
                compared_scalars += 1;
                let delta = (elkjs_value - elkrs_value).abs();
                max_absolute_delta = max_absolute_delta.max(delta);
                has_nonzero_delta |= delta > 0.0;
                if delta > tolerance {
                    changed_scalars += 1;
                    differences.push(GeometryDifference {
                        path: path.clone(),
                        elkjs: left,
                        elkrs: right,
                        absolute_delta: Some(delta),
                    });
                }
            }
            (None, Some(_)) => {
                missing_from_elkjs += 1;
                differences.push(GeometryDifference {
                    path: path.clone(),
                    elkjs: None,
                    elkrs: right,
                    absolute_delta: None,
                });
            }
            (Some(_), None) => {
                missing_from_elkrs += 1;
                differences.push(GeometryDifference {
                    path: path.clone(),
                    elkjs: left,
                    elkrs: None,
                    absolute_delta: None,
                });
            }
            (None, None) => unreachable!(),
        }
    }

    let status = if missing_from_elkjs > 0 || missing_from_elkrs > 0 || changed_scalars > 0 {
        ComparisonStatus::Different
    } else if has_nonzero_delta {
        ComparisonStatus::WithinTolerance
    } else {
        ComparisonStatus::Exact
    };
    Comparison {
        status,
        compared_scalars,
        missing_from_elkjs,
        missing_from_elkrs,
        changed_scalars,
        max_absolute_delta,
        differences,
    }
}

pub fn median(durations: &[std::time::Duration]) -> std::time::Duration {
    if durations.is_empty() {
        return std::time::Duration::ZERO;
    }
    let mut sorted = durations.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_json_formatting_and_object_order() {
        let left: Value = serde_json::from_str(
            r#"{"id":"root","children":[{"id":"a","x":1,"y":2,"width":3,"height":4}]}"#,
        )
        .unwrap();
        let right: Value = serde_json::from_str(
            r#"{ "children": [ { "height": 4.0, "width": 3, "y": 2, "x": 1, "id": "a" } ], "id": "root" }"#,
        )
        .unwrap();
        let comparison = compare_geometry(
            &extract_geometry(&left).values,
            &extract_geometry(&right).values,
            0.0,
        );
        assert_eq!(comparison.status, ComparisonStatus::Exact);
    }

    #[test]
    fn canonical_json_digest_ignores_object_key_order() {
        let left: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":1.0}]}"#).unwrap();
        let right: Value =
            serde_json::from_str(r#"{"children":[{"x":1.0,"id":"a"}],"id":"root"}"#).unwrap();
        assert_eq!(canonical_json_digest(&left), canonical_json_digest(&right));

        let different: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":2.0}]}"#).unwrap();
        assert_ne!(
            canonical_json_digest(&left),
            canonical_json_digest(&different)
        );
    }

    #[test]
    fn reports_sorted_geometry_paths_and_missing_values() {
        let left: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":1,"y":2}]}"#).unwrap();
        let right: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":3,"width":4}]}"#)
                .unwrap();
        let comparison = compare_geometry(
            &extract_geometry(&left).values,
            &extract_geometry(&right).values,
            0.0,
        );
        let paths: Vec<_> = comparison
            .differences
            .iter()
            .map(|diff| diff.path.as_str())
            .collect();
        assert_eq!(
            paths,
            vec!["graph/node:a/width", "graph/node:a/x", "graph/node:a/y",]
        );
        assert_eq!(comparison.status, ComparisonStatus::Different);
        assert_eq!(comparison.changed_scalars, 1);
        assert_eq!(comparison.missing_from_elkjs, 1);
        assert_eq!(comparison.missing_from_elkrs, 1);
    }

    #[test]
    fn tolerance_classifies_small_numeric_drift() {
        let left = BTreeMap::from([("node/x".to_string(), 1.0)]);
        let right = BTreeMap::from([("node/x".to_string(), 1.0 + 1e-10)]);
        let comparison = compare_geometry(&left, &right, 1e-9);
        assert_eq!(comparison.status, ComparisonStatus::WithinTolerance);
        assert!(comparison.differences.is_empty());
    }

    #[test]
    fn layout_contract_rejects_missing_edge_sections() {
        let input = serde_json::json!({
            "id": "root",
            "children": [
                { "id": "a", "width": 10, "height": 10 },
                { "id": "b", "width": 10, "height": 10 }
            ],
            "edges": [{ "id": "e", "sources": ["a"], "targets": ["b"] }]
        });
        let output = serde_json::json!({
            "id": "root",
            "width": 100,
            "height": 100,
            "children": [
                { "id": "a", "x": 0, "y": 0, "width": 10, "height": 10 },
                { "id": "b", "x": 20, "y": 0, "width": 10, "height": 10 }
            ],
            "edges": [{ "id": "e", "sources": ["a"], "targets": ["b"] }]
        });
        assert_eq!(
            validate_layout_output(&input, &output),
            vec!["edge e missing sections"]
        );
    }

    #[test]
    fn layout_contract_rejects_missing_node_geometry() {
        let input = serde_json::json!({
            "id": "root",
            "children": [{ "id": "a", "width": 10, "height": 10 }]
        });
        let output = serde_json::json!({
            "id": "root",
            "children": [{ "id": "a", "width": 10, "height": 10 }]
        });
        assert_eq!(
            validate_layout_output(&input, &output),
            vec!["node a missing numeric x", "node a missing numeric y"]
        );
    }
}
