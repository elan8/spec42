use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use super::interconnection_scene::{
    InterconnectionEdgeDto, InterconnectionPortDto, InterconnectionSceneDto,
};

const IBD_NODE_WIDTH: f64 = 280.0;
const IBD_NODE_HEIGHT: f64 = 140.0;
const ROOT_HEADER_HEIGHT: f64 = 28.0;

#[derive(Clone)]
struct PortDetail {
    id: String,
    name: String,
    direction: String,
    side_hint: String,
    scene_port_id: String,
}

#[derive(Clone)]
struct PreparedSceneNode {
    id: String,
    label: String,
    qualified_name: String,
    container_id: Option<String>,
    is_synthetic_container: bool,
    ports: Vec<PortDetail>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PortSide {
    West,
    East,
}

impl PortSide {
    fn as_elk(self) -> &'static str {
        match self {
            PortSide::West => "WEST",
            PortSide::East => "EAST",
        }
    }
}

#[derive(Default, Clone)]
struct PortUsage {
    source_count: usize,
    target_count: usize,
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn elk_id_for(node_id: &str) -> String {
    sanitize_id(node_id)
}

fn port_id_for(node_id: &str, port_name: &str) -> String {
    format!("{}__port__{}", sanitize_id(node_id), sanitize_id(port_name))
}

fn normalize_endpoint(value: &str) -> String {
    value.replace("::", ".").trim().to_string()
}

fn prepared_nodes_from_scene(scene: &InterconnectionSceneDto) -> Vec<PreparedSceneNode> {
    let mut node_ids: HashSet<String> = scene.nodes.iter().map(|node| node.id.clone()).collect();
    let ports_by_owner: HashMap<String, Vec<PortDetail>> = scene
        .ports
        .iter()
        .map(|port| (port.owner_node_id.clone(), port_detail_from_dto(port)))
        .fold(HashMap::new(), |mut acc, (owner, port)| {
            acc.entry(owner).or_default().push(port);
            acc
        });

    let mut nodes: Vec<PreparedSceneNode> = scene
        .nodes
        .iter()
        .map(|node| PreparedSceneNode {
            id: node.id.clone(),
            label: node.name.clone(),
            qualified_name: node.qualified_name.clone(),
            container_id: node.parent_id.clone(),
            is_synthetic_container: false,
            ports: ports_by_owner.get(&node.id).cloned().unwrap_or_default(),
        })
        .collect();

    for container in &scene.containers {
        if node_ids.contains(&container.id) {
            continue;
        }
        nodes.push(PreparedSceneNode {
            id: container.id.clone(),
            label: container.label.clone(),
            qualified_name: container.label.clone(),
            container_id: container.parent_id.clone(),
            is_synthetic_container: true,
            ports: Vec::new(),
        });
        node_ids.insert(container.id.clone());
    }

    nodes
}

fn port_detail_from_dto(port: &InterconnectionPortDto) -> PortDetail {
    PortDetail {
        id: port.id.clone(),
        name: port.name.clone(),
        direction: port.direction.clone().unwrap_or_default(),
        side_hint: port.side_hint.clone(),
        scene_port_id: port.id.clone(),
    }
}

fn port_layout_keys(node: &PreparedSceneNode, port: &PortDetail) -> Vec<String> {
    let mut keys = Vec::new();
    if !port.id.is_empty() {
        keys.push(normalize_endpoint(&port.id));
    }
    keys.push(format!("{}.{}", node.qualified_name, port.name));
    keys.push(normalize_endpoint(&port.name));
    keys.sort();
    keys.dedup();
    keys
}

fn build_port_usage(edges: &[InterconnectionEdgeDto]) -> HashMap<String, PortUsage> {
    let mut usage: HashMap<String, PortUsage> = HashMap::new();
    for edge in edges {
        for (endpoint, role_source) in [
            (edge.source_port_id.as_str(), true),
            (edge.target_port_id.as_str(), false),
        ] {
            let normalized = normalize_endpoint(endpoint);
            if normalized.is_empty() {
                continue;
            }
            let entry = usage.entry(normalized).or_default();
            if role_source {
                entry.source_count += 1;
            } else {
                entry.target_count += 1;
            }
        }
    }
    usage
}

fn usage_for_port(
    node: &PreparedSceneNode,
    port: &PortDetail,
    port_usage: &HashMap<String, PortUsage>,
) -> PortUsage {
    for key in port_layout_keys(node, port) {
        if let Some(explicit) = port_usage.get(&key) {
            return explicit.clone();
        }
    }
    let fallback_key = format!(
        "{}.{}",
        normalize_endpoint(&node.qualified_name),
        normalize_endpoint(&port.name)
    );
    if let Some(fallback) = port_usage.get(&fallback_key) {
        return fallback.clone();
    }
    PortUsage::default()
}

fn connector_port_name(node: &PreparedSceneNode, endpoint: &str) -> Option<String> {
    let endpoint_text = endpoint.trim();
    if endpoint_text.is_empty() {
        return None;
    }
    if let Some(port) = node
        .ports
        .iter()
        .find(|port| port.id == endpoint_text || port.scene_port_id == endpoint_text)
    {
        return Some(port.name.clone());
    }
    let endpoint_leaf = endpoint_text
        .split('.')
        .next_back()
        .unwrap_or(endpoint_text)
        .trim();
    node.ports
        .iter()
        .find(|port| {
            port.name == endpoint_leaf || endpoint_text.ends_with(&format!(".{}", port.name))
        })
        .map(|port| port.name.clone())
}

fn side_for_port(
    node: &PreparedSceneNode,
    port: &PortDetail,
    port_usage: &HashMap<String, PortUsage>,
) -> PortSide {
    let side_hint = port.side_hint.to_lowercase();
    if side_hint == "west" {
        return PortSide::West;
    }
    if side_hint == "east" {
        return PortSide::East;
    }
    let direction = port.direction.to_lowercase();
    if direction == "in" || direction == "input" {
        return PortSide::West;
    }
    if direction == "out" || direction == "output" {
        return PortSide::East;
    }
    let usage = usage_for_port(node, port, port_usage);
    if usage.target_count > usage.source_count {
        return PortSide::West;
    }
    if usage.source_count > usage.target_count {
        return PortSide::East;
    }
    let lower = port.name.to_lowercase();
    if lower.ends_with("in")
        || lower.contains("input")
        || lower.starts_with("in")
        || lower.ends_with("out")
        || lower.starts_with("out")
    {
        return if lower.ends_with("in") || lower.contains("input") || lower.starts_with("in") {
            PortSide::West
        } else {
            PortSide::East
        };
    }
    PortSide::East
}

fn compare_ports(
    a: &PortDetail,
    b: &PortDetail,
    usage_a: PortUsage,
    usage_b: PortUsage,
) -> std::cmp::Ordering {
    let degree_a = usage_a.source_count + usage_a.target_count;
    let degree_b = usage_b.source_count + usage_b.target_count;
    degree_b.cmp(&degree_a).then_with(|| a.name.cmp(&b.name))
}

fn split_ports_by_side(
    node: &PreparedSceneNode,
    port_usage: &HashMap<String, PortUsage>,
) -> (Vec<PortDetail>, Vec<PortDetail>) {
    let mut west = Vec::new();
    let mut east = Vec::new();
    for port in &node.ports {
        if side_for_port(node, port, port_usage) == PortSide::West {
            west.push(port.clone());
        } else {
            east.push(port.clone());
        }
    }
    west.sort_by(|a, b| {
        compare_ports(
            a,
            b,
            usage_for_port(node, a, port_usage),
            usage_for_port(node, b, port_usage),
        )
    });
    east.sort_by(|a, b| {
        compare_ports(
            a,
            b,
            usage_for_port(node, a, port_usage),
            usage_for_port(node, b, port_usage),
        )
    });
    (west, east)
}

fn compute_ibd_leaf_height(node: &PreparedSceneNode, port_rows: usize) -> f64 {
    let header_height = 50.0;
    let port_spacing = 26.0;
    let ports_height = if node.ports.is_empty() {
        0.0
    } else {
        port_rows as f64 * port_spacing + 22.0
    };
    (header_height + ports_height).clamp(IBD_NODE_HEIGHT, 340.0)
}

fn layout_options_json(pairs: &[(&str, &str)]) -> Value {
    Value::Object(
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), Value::String(value.to_string())))
            .collect(),
    )
}

fn elk_port_json(node_id: &str, port: &PortDetail, side: PortSide, index: usize) -> Value {
    json!({
        "id": port_id_for(node_id, &port.name),
        "width": 10,
        "height": 10,
        "layoutOptions": {
            "org.eclipse.elk.port.side": side.as_elk(),
            "org.eclipse.elk.port.index": index.to_string(),
        }
    })
}

fn to_elk_node(
    node: &PreparedSceneNode,
    children_by_parent: &HashMap<String, Vec<String>>,
    nodes_by_id: &HashMap<String, PreparedSceneNode>,
    port_usage: &HashMap<String, PortUsage>,
    prepared_id_for_elk_id: &mut HashMap<String, String>,
) -> Value {
    let (west_ports, east_ports) = split_ports_by_side(node, port_usage);
    let port_rows = west_ports
        .len()
        .max(east_ports.len())
        .max(if node.ports.is_empty() { 0 } else { 1 });
    let child_ids = children_by_parent
        .get(&node.id)
        .cloned()
        .unwrap_or_default();
    let children: Vec<Value> = child_ids
        .iter()
        .filter_map(|child_id| nodes_by_id.get(child_id))
        .map(|child| {
            to_elk_node(
                child,
                children_by_parent,
                nodes_by_id,
                port_usage,
                prepared_id_for_elk_id,
            )
        })
        .collect();
    let is_container = node.is_synthetic_container || !children.is_empty();
    let base_width = if is_container { 420.0 } else { IBD_NODE_WIDTH };
    let max_label = node
        .ports
        .iter()
        .map(|port| port.name.len())
        .max()
        .unwrap_or(0);
    let mut width = base_width.max(180.0 + (node.label.len() * 6).max(max_label * 5) as f64);
    let mut height = if is_container {
        ROOT_HEADER_HEIGHT + 72.0
    } else {
        compute_ibd_leaf_height(node, port_rows)
    };
    if is_container && !children.is_empty() {
        let child_width_sum: f64 = children
            .iter()
            .map(|child| {
                child
                    .get("width")
                    .and_then(Value::as_f64)
                    .unwrap_or(IBD_NODE_WIDTH)
            })
            .sum();
        width = width.max((child_width_sum + children.len() as f64 * 72.0).min(1040.0));
        height = ROOT_HEADER_HEIGHT + (58.0 + children.len() as f64 * 14.0).clamp(72.0, 132.0);
    }
    let elk_id = elk_id_for(&node.id);
    prepared_id_for_elk_id.insert(elk_id.clone(), node.id.clone());
    let mut ports_json = Vec::new();
    for (index, port) in west_ports.iter().enumerate() {
        ports_json.push(elk_port_json(&node.id, port, PortSide::West, index));
    }
    for (index, port) in east_ports.iter().enumerate() {
        ports_json.push(elk_port_json(&node.id, port, PortSide::East, index));
    }
    let container_top_inset = ROOT_HEADER_HEIGHT + 20.0;
    let node_layout_options = if children.is_empty() {
        layout_options_json(&[
            ("org.eclipse.elk.portConstraints", "FIXED_ORDER"),
            ("org.eclipse.elk.portAlignment.default", "CENTER"),
        ])
    } else {
        layout_options_json(&[
            (
                "elk.padding",
                &format!(
                    "[top={},left=24,bottom=24,right=24]",
                    container_top_inset as i32
                ),
            ),
            ("elk.direction", "RIGHT"),
            ("org.eclipse.elk.portConstraints", "FIXED_ORDER"),
            ("org.eclipse.elk.portAlignment.default", "CENTER"),
        ])
    };
    json!({
        "id": elk_id,
        "width": width,
        "height": height,
        "ports": ports_json,
        "children": children,
        "layoutOptions": node_layout_options,
    })
}

/// Build the ELK graph input JSON for a canonical interconnection scene (parity with TS `buildInterconnectionElkGraphInput`).
pub fn build_elk_graph_from_scene(scene: &InterconnectionSceneDto) -> Value {
    let prepared_nodes = prepared_nodes_from_scene(scene);
    let nodes_by_id: HashMap<String, PreparedSceneNode> = prepared_nodes
        .iter()
        .map(|node| (node.id.clone(), node.clone()))
        .collect();
    let mut children_by_parent: HashMap<String, Vec<String>> = HashMap::new();
    let mut roots = Vec::new();
    for node in &prepared_nodes {
        if let Some(parent_id) = &node.container_id {
            if nodes_by_id.contains_key(parent_id) {
                children_by_parent
                    .entry(parent_id.clone())
                    .or_default()
                    .push(node.id.clone());
                continue;
            }
        }
        roots.push(node.id.clone());
    }
    let port_usage = build_port_usage(&scene.edges);
    let mut prepared_id_for_elk_id = HashMap::new();
    let children: Vec<Value> = roots
        .iter()
        .filter_map(|root_id| nodes_by_id.get(root_id))
        .map(|node| {
            to_elk_node(
                node,
                &children_by_parent,
                &nodes_by_id,
                &port_usage,
                &mut prepared_id_for_elk_id,
            )
        })
        .collect();
    let edges: Vec<Value> = scene
        .edges
        .iter()
        .filter_map(|edge| {
            let source_node = nodes_by_id.get(&edge.source_node_id)?;
            let target_node = nodes_by_id.get(&edge.target_node_id)?;
            let source_port = connector_port_name(source_node, &edge.source_port_id);
            let target_port = connector_port_name(target_node, &edge.target_port_id);
            let sources = vec![source_port
                .as_ref()
                .map(|name| port_id_for(&source_node.id, name))
                .unwrap_or_else(|| elk_id_for(&source_node.id))];
            let targets = vec![target_port
                .as_ref()
                .map(|name| port_id_for(&target_node.id, name))
                .unwrap_or_else(|| elk_id_for(&target_node.id))];
            Some(json!({
                "id": edge.id,
                "sources": sources,
                "targets": targets,
            }))
        })
        .collect();
    let graph_layout_options = layout_options_json(&[
        ("elk.algorithm", "layered"),
        ("elk.hierarchyHandling", "INCLUDE_CHILDREN"),
        ("elk.direction", "RIGHT"),
        ("elk.spacing.nodeNode", "150"),
        ("elk.layered.spacing.nodeNodeBetweenLayers", "220"),
        ("elk.spacing.edgeNode", "110"),
        ("elk.spacing.edgeEdge", "90"),
        ("elk.edgeRouting", "ORTHOGONAL"),
        ("elk.layered.nodePlacement.strategy", "NETWORK_SIMPLEX"),
        ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
        ("elk.separateConnectedComponents", "true"),
        ("elk.padding", "[top=70,left=70,bottom=70,right=70]"),
        ("org.eclipse.elk.portConstraints", "FIXED_ORDER"),
        ("org.eclipse.elk.portAlignment.default", "CENTER"),
        ("org.eclipse.elk.json.edgeCoords", "ROOT"),
    ]);
    json!({
        "id": "root",
        "layoutOptions": graph_layout_options,
        "children": children,
        "edges": edges,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../shared/diagram-renderer/test-fixtures/interconnection")
            .join(name)
    }

    fn load_scene_from_fixture(name: &str) -> InterconnectionSceneDto {
        let raw = fs::read_to_string(fixture_path(name)).expect("read fixture");
        let payload: Value = serde_json::from_str(&raw).expect("parse fixture");
        let scene = payload
            .get("interconnectionScene")
            .expect("interconnectionScene");
        serde_json::from_value(scene.clone()).expect("deserialize scene")
    }

    fn collect_node_ids(value: &Value, out: &mut Vec<String>) {
        if let Some(id) = value.get("id").and_then(Value::as_str) {
            out.push(id.to_string());
        }
        if let Some(children) = value.get("children").and_then(Value::as_array) {
            for child in children {
                collect_node_ids(child, out);
            }
        }
    }

    fn edge_signatures(value: &Value) -> Vec<(String, String, String)> {
        value
            .get("edges")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|edge| {
                let id = edge.get("id")?.as_str()?.to_string();
                let source = edge.get("sources")?.get(0)?.as_str()?.to_string();
                let target = edge.get("targets")?.get(0)?.as_str()?.to_string();
                Some((id, source, target))
            })
            .collect()
    }

    #[test]
    fn scene_two_part_chain_elk_graph_structure() {
        let scene = load_scene_from_fixture("scene-two-part-chain.json");
        let graph = build_elk_graph_from_scene(&scene);
        let mut node_ids = Vec::new();
        if let Some(children) = graph.get("children").and_then(Value::as_array) {
            for child in children {
                collect_node_ids(child, &mut node_ids);
            }
        }
        assert_eq!(node_ids.len(), 2);
        let edges = edge_signatures(&graph);
        assert_eq!(edges.len(), 1);
        assert!(edges[0].1.contains("__port__out"));
        assert!(edges[0].2.contains("__port__in"));
    }

    #[test]
    fn nested_ring_elk_graph_matches_typescript_golden_when_present() {
        let golden_path = fixture_path("nested-ring-minimal-elk-input.json");
        if !golden_path.exists() {
            return;
        }
        let scene = load_scene_from_fixture("nested-ring-minimal.json");
        let rust_graph = build_elk_graph_from_scene(&scene);
        let golden: Value =
            serde_json::from_str(&fs::read_to_string(golden_path).expect("read golden"))
                .expect("parse golden");
        assert_eq!(
            edge_signatures(&rust_graph),
            edge_signatures(&golden),
            "edge signatures should match TS golden"
        );
    }
}
