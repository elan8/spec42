#![forbid(unsafe_code)]
#![doc = "Import/export support for ELK Graph JSON."]

use std::collections::HashMap;

use elk_core::{Point, Rect};
use elk_graph::{EdgeEndpoint, ElkGraph, PropertyBag, PropertyValue, ShapeGeometry};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum JsonId {
    Str(String),
    Int(i64),
}

impl JsonId {
    fn from_value(value: &Value) -> Result<Self, JsonIoError> {
        match value {
            Value::String(s) => Ok(JsonId::Str(s.clone())),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(JsonId::Int(i))
                } else {
                    Err(JsonIoError::InvalidId(format!(
                        "numeric id must be an integer, got {n}"
                    )))
                }
            }
            other => Err(JsonIoError::InvalidId(format!(
                "id must be a string or integer, got {other}"
            ))),
        }
    }
}

#[derive(Debug)]
pub enum JsonIoError {
    Json(serde_json::Error),
    ExpectedObject(&'static str),
    MissingField(&'static str),
    InvalidId(String),
    UnknownReference(String),
}

impl From<serde_json::Error> for JsonIoError {
    fn from(value: serde_json::Error) -> Self {
        JsonIoError::Json(value)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImportWarnings {
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub struct ImportResult {
    pub graph: ElkGraph,
    pub warnings: ImportWarnings,
}

/// Import an ELK Graph JSON string into `elk_core::Graph`.
///
/// Notes (initial compatibility layer):
/// - Supports node/port/edge/label hierarchy.
/// - Supports modern `sources`/`targets` edges and legacy `source`/`target` edges.
/// - Hyperedges are not represented; when multiple sources/targets exist, the first valid endpoints are used.
pub fn import_str(input: &str) -> Result<ImportResult, JsonIoError> {
    let value: Value = serde_json::from_str(input)?;
    let root = value
        .as_object()
        .ok_or(JsonIoError::ExpectedObject("top-level graph"))?;

    let mut ctx = ImportCtx::default();
    let mut graph = ElkGraph::new();

    // Root node already exists in `ElkGraph::new()`. Populate it from JSON.
    apply_layout_options(root, &mut graph.properties, &mut ctx.warnings);
    let root_id = graph.root;
    import_node_object(&mut ctx, &mut graph, root_id, None, root)?;
    ctx.root = Some(graph.root);

    // Edge lists are scoped per hierarchy level. Walk the JSON tree and import `edges` at each node.
    import_edges_recursive(&mut ctx, &mut graph, root)?;

    Ok(ImportResult {
        graph,
        warnings: ImportWarnings {
            warnings: ctx.warnings,
        },
    })
}

/// Export `elk_core::Graph` into ELK Graph JSON (modern encoding).
///
/// This exporter is intentionally minimal and focuses on emitting a JSON structure that ELK’s importer
/// can read back. Layout options and coordinate modes are deferred.
pub fn export_to_value(graph: &elk_core::Graph) -> Value {
    let mut node_ids: HashMap<elk_core::NodeId, String> = HashMap::new();
    let mut port_ids: HashMap<elk_core::PortId, String> = HashMap::new();

    // Create a synthetic root node to mirror ELK JSON’s top-level node object.
    let root_id = "root".to_string();
    let mut root = serde_json::Map::new();
    root.insert("id".to_string(), Value::String(root_id));

    // Nodes/ports/children
    let children: Vec<Value> = graph
        .top_level_nodes()
        .into_iter()
        .map(|n| export_node(graph, n, &mut node_ids, &mut port_ids))
        .collect();
    if !children.is_empty() {
        root.insert("children".to_string(), Value::Array(children));
    }

    // Edges live on the containing node in ELK; we export all edges at the root for now.
    let edges: Vec<Value> = graph
        .edges
        .iter()
        .map(|e| export_edge(graph, e.id, &node_ids, &port_ids))
        .collect();
    if !edges.is_empty() {
        root.insert("edges".to_string(), Value::Array(edges));
    }

    Value::Object(root)
}

pub fn export_pretty(graph: &elk_core::Graph) -> String {
    serde_json::to_string_pretty(&export_to_value(graph)).unwrap_or_else(|_| "{}".to_string())
}

/// Export an `elk_graph::ElkGraph` into ELK Graph JSON (modern encoding).
///
/// IDs are synthesized from indices (`n1`, `p0`, `e0`, ...). The exporter focuses on structural
/// fidelity and lossless properties round-tripping rather than preserving original JSON IDs.
pub fn export_elk_graph_to_value(graph: &ElkGraph) -> Value {
    fn node_id_str(graph: &ElkGraph, node: elk_graph::NodeId) -> String {
        if node == graph.root {
            "root".to_string()
        } else {
            format!("n{}", node.index())
        }
    }
    fn port_id_str(port: elk_graph::PortId) -> String {
        format!("p{}", port.index())
    }
    fn edge_id_str(edge: elk_graph::EdgeId) -> String {
        format!("e{}", edge.index())
    }
    fn label_id_str(label: elk_graph::LabelId) -> String {
        format!("l{}", label.index())
    }
    fn property_to_json(v: &PropertyValue) -> Value {
        match v {
            PropertyValue::Null => Value::Null,
            PropertyValue::Bool(b) => Value::Bool(*b),
            PropertyValue::Int(i) => Value::from(*i),
            PropertyValue::Float(f) => Value::from(*f),
            PropertyValue::String(s) => Value::String(s.clone()),
            PropertyValue::Array(arr) => Value::Array(arr.iter().map(property_to_json).collect()),
            PropertyValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (k, v) in obj {
                    map.insert(k.clone(), property_to_json(v));
                }
                Value::Object(map)
            }
        }
    }
    fn write_layout_options(out: &mut serde_json::Map<String, Value>, props: &PropertyBag) {
        if props.is_empty() {
            return;
        }
        let mut map = serde_json::Map::new();
        for (k, v) in props.iter() {
            map.insert(k.0.clone(), property_to_json(v));
        }
        out.insert("layoutOptions".to_string(), Value::Object(map));
    }

    fn export_label(graph: &ElkGraph, label: elk_graph::LabelId) -> Value {
        let l = &graph.labels[label.index()];
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), Value::String(label_id_str(label)));
        obj.insert("text".to_string(), Value::String(l.text.clone()));
        obj.insert("x".to_string(), Value::from(l.geometry.x));
        obj.insert("y".to_string(), Value::from(l.geometry.y));
        obj.insert("width".to_string(), Value::from(l.geometry.width));
        obj.insert("height".to_string(), Value::from(l.geometry.height));
        write_layout_options(&mut obj, &l.properties);
        Value::Object(obj)
    }

    fn export_port(graph: &ElkGraph, port: elk_graph::PortId) -> Value {
        let p = &graph.ports[port.index()];
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), Value::String(port_id_str(port)));
        obj.insert("x".to_string(), Value::from(p.geometry.x));
        obj.insert("y".to_string(), Value::from(p.geometry.y));
        obj.insert("width".to_string(), Value::from(p.geometry.width));
        obj.insert("height".to_string(), Value::from(p.geometry.height));

        // Ensure `port.side` is preserved even if not present in properties.
        if p.properties.get(&elk_graph::PropertyKey("port.side".to_string())).is_none() {
            let side = match p.side {
                elk_core::PortSide::North => "NORTH",
                elk_core::PortSide::South => "SOUTH",
                elk_core::PortSide::East => "EAST",
                elk_core::PortSide::West => "WEST",
            };
            let mut props = p.properties.clone();
            props.insert("port.side", PropertyValue::String(side.to_string()));
            write_layout_options(&mut obj, &props);
        } else {
            write_layout_options(&mut obj, &p.properties);
        }

        if !p.labels.is_empty() {
            obj.insert(
                "labels".to_string(),
                Value::Array(p.labels.iter().copied().map(|l| export_label(graph, l)).collect()),
            );
        }
        Value::Object(obj)
    }

    fn export_edge_section(graph: &ElkGraph, sec: elk_graph::EdgeSectionId) -> Value {
        let s = &graph.edge_sections[sec.index()];
        let mut obj = serde_json::Map::new();
        let mut start = serde_json::Map::new();
        start.insert("x".to_string(), Value::from(s.start.x));
        start.insert("y".to_string(), Value::from(s.start.y));
        let mut end = serde_json::Map::new();
        end.insert("x".to_string(), Value::from(s.end.x));
        end.insert("y".to_string(), Value::from(s.end.y));
        obj.insert("startPoint".to_string(), Value::Object(start));
        obj.insert("endPoint".to_string(), Value::Object(end));
        if !s.bend_points.is_empty() {
            let bps = s
                .bend_points
                .iter()
                .map(|p| {
                    let mut m = serde_json::Map::new();
                    m.insert("x".to_string(), Value::from(p.x));
                    m.insert("y".to_string(), Value::from(p.y));
                    Value::Object(m)
                })
                .collect();
            obj.insert("bendPoints".to_string(), Value::Array(bps));
        }
        write_layout_options(&mut obj, &s.properties);
        Value::Object(obj)
    }

    fn export_edge(graph: &ElkGraph, edge: elk_graph::EdgeId) -> Value {
        let e = &graph.edges[edge.index()];
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), Value::String(edge_id_str(edge)));
        obj.insert(
            "sources".to_string(),
            Value::Array(
                e.sources
                    .iter()
                    .map(|ep| {
                        if let Some(pid) = ep.port {
                            Value::String(port_id_str(pid))
                        } else {
                            Value::String(node_id_str(graph, ep.node))
                        }
                    })
                    .collect(),
            ),
        );
        obj.insert(
            "targets".to_string(),
            Value::Array(
                e.targets
                    .iter()
                    .map(|ep| {
                        if let Some(pid) = ep.port {
                            Value::String(port_id_str(pid))
                        } else {
                            Value::String(node_id_str(graph, ep.node))
                        }
                    })
                    .collect(),
            ),
        );
        write_layout_options(&mut obj, &e.properties);
        if !e.labels.is_empty() {
            obj.insert(
                "labels".to_string(),
                Value::Array(e.labels.iter().copied().map(|l| export_label(graph, l)).collect()),
            );
        }
        if !e.sections.is_empty() {
            obj.insert(
                "sections".to_string(),
                Value::Array(
                    e.sections
                        .iter()
                        .copied()
                        .map(|s| export_edge_section(graph, s))
                        .collect(),
                ),
            );
        }
        Value::Object(obj)
    }

    fn export_node(graph: &ElkGraph, node: elk_graph::NodeId) -> Value {
        let n = &graph.nodes[node.index()];
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), Value::String(node_id_str(graph, node)));
        obj.insert("x".to_string(), Value::from(n.geometry.x));
        obj.insert("y".to_string(), Value::from(n.geometry.y));
        obj.insert("width".to_string(), Value::from(n.geometry.width));
        obj.insert("height".to_string(), Value::from(n.geometry.height));
        write_layout_options(&mut obj, &n.properties);

        if !n.labels.is_empty() {
            obj.insert(
                "labels".to_string(),
                Value::Array(n.labels.iter().copied().map(|l| export_label(graph, l)).collect()),
            );
        }
        if !n.ports.is_empty() {
            obj.insert(
                "ports".to_string(),
                Value::Array(n.ports.iter().copied().map(|p| export_port(graph, p)).collect()),
            );
        }
        if !n.edges.is_empty() {
            obj.insert(
                "edges".to_string(),
                Value::Array(n.edges.iter().copied().map(|e| export_edge(graph, e)).collect()),
            );
        }
        if !n.children.is_empty() {
            obj.insert(
                "children".to_string(),
                Value::Array(
                    n.children
                        .iter()
                        .copied()
                        .map(|c| export_node(graph, c))
                        .collect(),
                ),
            );
        }
        Value::Object(obj)
    }

    export_node(graph, graph.root)
}

pub fn export_elk_graph_pretty(graph: &ElkGraph) -> String {
    serde_json::to_string_pretty(&export_elk_graph_to_value(graph))
        .unwrap_or_else(|_| "{}".to_string())
}

#[derive(Default)]
struct ImportCtx {
    root: Option<elk_graph::NodeId>,
    warnings: Vec<String>,
    node_ids: HashMap<JsonId, elk_graph::NodeId>,
    port_ids: HashMap<JsonId, elk_graph::PortId>,
}

fn import_node_object(
    ctx: &mut ImportCtx,
    graph: &mut ElkGraph,
    node_id: elk_graph::NodeId,
    parent: Option<elk_graph::NodeId>,
    obj: &serde_json::Map<String, Value>,
) -> Result<(), JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let json_id = JsonId::from_value(id_value)?;
    ctx.node_ids.insert(json_id, node_id);

    // Geometry.
    let x = get_f32(obj, "x").unwrap_or(0.0);
    let y = get_f32(obj, "y").unwrap_or(0.0);
    let w = get_f32(obj, "width").unwrap_or(0.0);
    let h = get_f32(obj, "height").unwrap_or(0.0);
    graph.nodes[node_id.index()].geometry = ShapeGeometry {
        x,
        y,
        width: w,
        height: h,
    };
    graph.nodes[node_id.index()].parent = parent;

    apply_layout_options(obj, &mut graph.nodes[node_id.index()].properties, &mut ctx.warnings);

    // Labels
    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else { continue };
            let label_id = import_label_object(graph, label_obj);
            graph.attach_label_to_node(node_id, label_id);
        }
    }

    // Ports
    if let Some(Value::Array(ports)) = obj.get("ports") {
        for port in ports {
            let Some(port_obj) = port.as_object() else { continue };
            import_port_object(ctx, graph, node_id, port_obj)?;
        }
    }

    // Children
    if let Some(Value::Array(children)) = obj.get("children") {
        for child in children {
            let Some(child_obj) = child.as_object() else { continue };
            let child_geom = ShapeGeometry {
                x: get_f32(child_obj, "x").unwrap_or(0.0),
                y: get_f32(child_obj, "y").unwrap_or(0.0),
                width: get_f32(child_obj, "width").unwrap_or(0.0),
                height: get_f32(child_obj, "height").unwrap_or(0.0),
            };
            let child_id = graph.add_node(node_id, child_geom);
            import_node_object(ctx, graph, child_id, Some(node_id), child_obj)?;
        }
    }

    Ok(())
}

fn import_port_object(
    ctx: &mut ImportCtx,
    graph: &mut ElkGraph,
    node_id: elk_graph::NodeId,
    obj: &serde_json::Map<String, Value>,
) -> Result<(), JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let json_id = JsonId::from_value(id_value)?;

    let geom = ShapeGeometry {
        x: get_f32(obj, "x").unwrap_or(0.0),
        y: get_f32(obj, "y").unwrap_or(0.0),
        width: get_f32(obj, "width").unwrap_or(0.0),
        height: get_f32(obj, "height").unwrap_or(0.0),
    };
    // Default port side; may be overridden by layout options.
    let port_id = graph.add_port(node_id, elk_core::PortSide::East, geom);
    ctx.port_ids.insert(json_id, port_id);

    apply_layout_options(obj, &mut graph.ports[port_id.index()].properties, &mut ctx.warnings);
    if let Some(side) = parse_port_side_from_props(&graph.ports[port_id.index()].properties) {
        graph.ports[port_id.index()].side = side;
    }

    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else { continue };
            let label_id = import_label_object(graph, label_obj);
            graph.attach_label_to_port(port_id, label_id);
        }
    }

    Ok(())
}

fn import_label_object(graph: &mut ElkGraph, obj: &serde_json::Map<String, Value>) -> elk_graph::LabelId {
    let text = obj
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let geom = ShapeGeometry {
        x: get_f32(obj, "x").unwrap_or(0.0),
        y: get_f32(obj, "y").unwrap_or(0.0),
        width: get_f32(obj, "width").unwrap_or(0.0),
        height: get_f32(obj, "height").unwrap_or(0.0),
    };
    let id = graph.add_label(text, geom);
    apply_layout_options(obj, &mut graph.labels[id.index()].properties, &mut Vec::new());
    id
}

fn import_edges_recursive(
    ctx: &mut ImportCtx,
    graph: &mut ElkGraph,
    node_obj: &serde_json::Map<String, Value>,
) -> Result<(), JsonIoError> {
    // Find container node id (this JSON object corresponds to some node id we registered).
    let container_json_id = node_obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let container = ctx
        .node_ids
        .get(&JsonId::from_value(container_json_id)?)
        .copied()
        .ok_or_else(|| JsonIoError::UnknownReference("edge container node not found".to_string()))?;

    if let Some(Value::Array(edges)) = node_obj.get("edges") {
        for edge_value in edges {
            let Some(edge_obj) = edge_value.as_object() else { continue };
            import_edge_object(ctx, graph, container, edge_obj)?;
        }
    }

    if let Some(Value::Array(children)) = node_obj.get("children") {
        for child in children {
            let Some(child_obj) = child.as_object() else { continue };
            import_edges_recursive(ctx, graph, child_obj)?;
        }
    }

    Ok(())
}

fn import_edge_object(
    ctx: &mut ImportCtx,
    graph: &mut ElkGraph,
    container: elk_graph::NodeId,
    obj: &serde_json::Map<String, Value>,
) -> Result<(), JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let _json_id = JsonId::from_value(id_value)?;

    let (sources, targets) = if obj.contains_key("sources") || obj.contains_key("targets") {
        resolve_modern_edge_endpoints(ctx, graph, obj)?
    } else {
        resolve_legacy_edge_endpoints(ctx, obj)?
    };

    let edge_id = graph.add_edge(container, sources, targets);
    apply_layout_options(obj, &mut graph.edges[edge_id.index()].properties, &mut ctx.warnings);

    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else { continue };
            let label_id = import_label_object(graph, label_obj);
            graph.attach_label_to_edge(edge_id, label_id);
        }
    }

    if let Some((start, bends, end)) = import_edge_section(obj) {
        let _ = graph.add_edge_section(edge_id, start, bends, end);
    }

    Ok(())
}

fn resolve_modern_edge_endpoints(
    ctx: &ImportCtx,
    graph: &ElkGraph,
    obj: &serde_json::Map<String, Value>,
) -> Result<(Vec<EdgeEndpoint>, Vec<EdgeEndpoint>), JsonIoError> {
    let sources = obj
        .get("sources")
        .and_then(|v| v.as_array())
        .ok_or(JsonIoError::MissingField("sources"))?;
    let targets = obj
        .get("targets")
        .and_then(|v| v.as_array())
        .ok_or(JsonIoError::MissingField("targets"))?;

    let src = sources
        .iter()
        .filter_map(|v| endpoint_by_id(ctx, graph, v))
        .collect::<Vec<_>>();
    let tgt = targets
        .iter()
        .filter_map(|v| endpoint_by_id(ctx, graph, v))
        .collect::<Vec<_>>();

    if src.is_empty() || tgt.is_empty() {
        return Err(JsonIoError::UnknownReference(
            "edge has no resolvable source or target endpoint".to_string(),
        ));
    }

    Ok((src, tgt))
}

fn resolve_legacy_edge_endpoints(
    ctx: &ImportCtx,
    obj: &serde_json::Map<String, Value>,
) -> Result<(Vec<EdgeEndpoint>, Vec<EdgeEndpoint>), JsonIoError> {
    let source_node_id = obj.get("source").ok_or(JsonIoError::MissingField("source"))?;
    let target_node_id = obj.get("target").ok_or(JsonIoError::MissingField("target"))?;
    let source_node = ctx
        .node_ids
        .get(&JsonId::from_value(source_node_id)?)
        .copied()
        .ok_or_else(|| JsonIoError::UnknownReference("unknown legacy edge source node".to_string()))?;
    let target_node = ctx
        .node_ids
        .get(&JsonId::from_value(target_node_id)?)
        .copied()
        .ok_or_else(|| JsonIoError::UnknownReference("unknown legacy edge target node".to_string()))?;

    let source_port = obj
        .get("sourcePort")
        .and_then(|v| JsonId::from_value(v).ok())
        .and_then(|id| ctx.port_ids.get(&id).copied());
    let target_port = obj
        .get("targetPort")
        .and_then(|v| JsonId::from_value(v).ok())
        .and_then(|id| ctx.port_ids.get(&id).copied());

    Ok((
        vec![EdgeEndpoint {
            node: source_node,
            port: source_port,
        }],
        vec![EdgeEndpoint {
            node: target_node,
            port: target_port,
        }],
    ))
}

fn endpoint_by_id(ctx: &ImportCtx, graph: &ElkGraph, id: &Value) -> Option<EdgeEndpoint> {
    let json_id = JsonId::from_value(id).ok()?;
    if let Some(&port_id) = ctx.port_ids.get(&json_id) {
        let node = graph.ports[port_id.index()].node;
        return Some(EdgeEndpoint {
            node,
            port: Some(port_id),
        });
    }
    if let Some(&node_id) = ctx.node_ids.get(&json_id) {
        return Some(EdgeEndpoint { node: node_id, port: None });
    }
    None
}

fn import_edge_section(obj: &serde_json::Map<String, Value>) -> Option<(Point, Vec<Point>, Point)> {
    // Modern sections encoding: use first section if present.
    if let Some(Value::Array(sections)) = obj.get("sections") {
        if let Some(first) = sections.first().and_then(|v| v.as_object()) {
            let start = first.get("startPoint").and_then(|v| v.as_object())?;
            let end = first.get("endPoint").and_then(|v| v.as_object())?;
            let start_pt = Point::new(
                get_f32(start, "x").unwrap_or(0.0),
                get_f32(start, "y").unwrap_or(0.0),
            );
            let end_pt = Point::new(
                get_f32(end, "x").unwrap_or(0.0),
                get_f32(end, "y").unwrap_or(0.0),
            );
            let mut bends = Vec::new();
            if let Some(Value::Array(bp)) = first.get("bendPoints") {
                for p in bp {
                    if let Some(pobj) = p.as_object() {
                        bends.push(Point::new(
                            get_f32(pobj, "x").unwrap_or(0.0),
                            get_f32(pobj, "y").unwrap_or(0.0),
                        ));
                    }
                }
            }
            return Some((start_pt, bends, end_pt));
        }
    }

    // Legacy encoding.
    let has_legacy =
        obj.contains_key("sourcePoint") || obj.contains_key("targetPoint") || obj.contains_key("bendPoints");
    if !has_legacy {
        return None;
    }
    let start_pt = obj
        .get("sourcePoint")
        .and_then(|v| v.as_object())
        .map(|p| Point::new(get_f32(p, "x").unwrap_or(0.0), get_f32(p, "y").unwrap_or(0.0)))
        .unwrap_or_default();
    let end_pt = obj
        .get("targetPoint")
        .and_then(|v| v.as_object())
        .map(|p| Point::new(get_f32(p, "x").unwrap_or(0.0), get_f32(p, "y").unwrap_or(0.0)))
        .unwrap_or_default();
    let mut bends = Vec::new();
    if let Some(Value::Array(bp)) = obj.get("bendPoints") {
        for p in bp {
            if let Some(pobj) = p.as_object() {
                bends.push(Point::new(
                    get_f32(pobj, "x").unwrap_or(0.0),
                    get_f32(pobj, "y").unwrap_or(0.0),
                ));
            }
        }
    }
    Some((start_pt, bends, end_pt))
}

fn apply_layout_options(obj: &serde_json::Map<String, Value>, out: &mut PropertyBag, warnings: &mut Vec<String>) {
    // Legacy first.
    if let Some(Value::Object(props)) = obj.get("properties") {
        for (k, v) in props {
            out.insert(k.as_str(), json_to_property(v));
        }
    }
    // `layoutOptions` overrides legacy `properties`.
    if let Some(Value::Object(opts)) = obj.get("layoutOptions") {
        for (k, v) in opts {
            out.insert(k.as_str(), json_to_property(v));
        }
    }

    // Best-effort warning surface: remind when options exist but are not yet bridged into engines.
    if (obj.get("layoutOptions").is_some() || obj.get("properties").is_some()) && warnings.is_empty() {
        // no-op placeholder; we keep warnings vector for future richer diagnostics.
    }
}

fn json_to_property(v: &Value) -> PropertyValue {
    match v {
        Value::Null => PropertyValue::Null,
        Value::Bool(b) => PropertyValue::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                PropertyValue::Int(i)
            } else {
                PropertyValue::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        Value::String(s) => PropertyValue::String(s.clone()),
        Value::Array(arr) => PropertyValue::Array(arr.iter().map(json_to_property).collect()),
        Value::Object(obj) => {
            let mut out = std::collections::BTreeMap::new();
            for (k, v) in obj {
                out.insert(k.clone(), json_to_property(v));
            }
            PropertyValue::Object(out)
        }
    }
}

fn parse_port_side_from_props(props: &PropertyBag) -> Option<elk_core::PortSide> {
    for (k, v) in props.iter() {
        let key = k.0.to_ascii_lowercase();
        if !key.ends_with("port.side") && !key.ends_with("elk.port.side") && !key.ends_with("side") {
            continue;
        }
        let s = match v {
            PropertyValue::String(s) => s.as_str(),
            _ => continue,
        };
        let upper = s.trim().to_ascii_uppercase();
        return match upper.as_str() {
            "NORTH" => Some(elk_core::PortSide::North),
            "SOUTH" => Some(elk_core::PortSide::South),
            "EAST" => Some(elk_core::PortSide::East),
            "WEST" => Some(elk_core::PortSide::West),
            _ => None,
        };
    }
    None
}

fn get_f32(map: &serde_json::Map<String, Value>, key: &str) -> Option<f32> {
    map.get(key).and_then(|v| match v {
        Value::Number(n) => n.as_f64().map(|f| f as f32),
        _ => None,
    })
}

fn export_node(
    graph: &elk_core::Graph,
    node_id: elk_core::NodeId,
    node_ids: &mut HashMap<elk_core::NodeId, String>,
    port_ids: &mut HashMap<elk_core::PortId, String>,
) -> Value {
    let node = graph.node(node_id);
    let id = node_ids
        .entry(node_id)
        .or_insert_with(|| format!("n{}", node_id.index()))
        .clone();

    let mut obj = serde_json::Map::new();
    obj.insert("id".to_string(), Value::String(id));

    let Rect { origin, size } = node.bounds;
    obj.insert("x".to_string(), Value::from(origin.x));
    obj.insert("y".to_string(), Value::from(origin.y));
    obj.insert("width".to_string(), Value::from(size.width));
    obj.insert("height".to_string(), Value::from(size.height));

    if !node.labels.is_empty() {
        obj.insert(
            "labels".to_string(),
            Value::Array(
                node.labels
                    .iter()
                    .map(|id| export_label(graph, *id))
                    .collect(),
            ),
        );
    }
    if !node.ports.is_empty() {
        obj.insert(
            "ports".to_string(),
            Value::Array(
                node.ports
                    .iter()
                    .map(|pid| export_port(graph, *pid, port_ids))
                    .collect(),
            ),
        );
    }
    if !node.children.is_empty() {
        obj.insert(
            "children".to_string(),
            Value::Array(
                node.children
                    .iter()
                    .map(|cid| export_node(graph, *cid, node_ids, port_ids))
                    .collect(),
            ),
        );
    }

    Value::Object(obj)
}

fn export_port(
    graph: &elk_core::Graph,
    port_id: elk_core::PortId,
    port_ids: &mut HashMap<elk_core::PortId, String>,
) -> Value {
    let port = graph.port(port_id);
    let id = port_ids
        .entry(port_id)
        .or_insert_with(|| format!("p{}", port_id.index()))
        .clone();
    let mut obj = serde_json::Map::new();
    obj.insert("id".to_string(), Value::String(id));
    obj.insert("x".to_string(), Value::from(port.bounds.origin.x));
    obj.insert("y".to_string(), Value::from(port.bounds.origin.y));
    obj.insert("width".to_string(), Value::from(port.bounds.size.width));
    obj.insert("height".to_string(), Value::from(port.bounds.size.height));
    if !port.labels.is_empty() {
        obj.insert(
            "labels".to_string(),
            Value::Array(
                port.labels
                    .iter()
                    .map(|id| export_label(graph, *id))
                    .collect(),
            ),
        );
    }
    Value::Object(obj)
}

fn export_edge(
    graph: &elk_core::Graph,
    edge_id: elk_core::EdgeId,
    node_ids: &HashMap<elk_core::NodeId, String>,
    port_ids: &HashMap<elk_core::PortId, String>,
) -> Value {
    let edge = graph.edge(edge_id);
    let mut obj = serde_json::Map::new();
    obj.insert("id".to_string(), Value::String(format!("e{}", edge_id.index())));

    let source_id = endpoint_id(edge.source, node_ids, port_ids);
    let target_id = endpoint_id(edge.target, node_ids, port_ids);
    obj.insert("sources".to_string(), Value::Array(vec![Value::String(source_id)]));
    obj.insert("targets".to_string(), Value::Array(vec![Value::String(target_id)]));

    if !edge.labels.is_empty() {
        obj.insert(
            "labels".to_string(),
            Value::Array(
                edge.labels
                    .iter()
                    .map(|id| export_label(graph, *id))
                    .collect(),
            ),
        );
    }

    if let Some(section) = edge.sections.first() {
        obj.insert(
            "sections".to_string(),
            Value::Array(vec![export_section(section)]),
        );
    }

    Value::Object(obj)
}

fn endpoint_id(
    endpoint: elk_core::EdgeEndpoint,
    node_ids: &HashMap<elk_core::NodeId, String>,
    port_ids: &HashMap<elk_core::PortId, String>,
) -> String {
    if let Some(port) = endpoint.port {
        port_ids
            .get(&port)
            .cloned()
            .unwrap_or_else(|| format!("p{}", port.index()))
    } else {
        node_ids
            .get(&endpoint.node)
            .cloned()
            .unwrap_or_else(|| format!("n{}", endpoint.node.index()))
    }
}

fn export_section(section: &elk_core::EdgeSection) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("id".to_string(), Value::String("s0".to_string()));
    obj.insert(
        "startPoint".to_string(),
        Value::Object(point_obj(section.start)),
    );
    obj.insert(
        "endPoint".to_string(),
        Value::Object(point_obj(section.end)),
    );
    if !section.bend_points.is_empty() {
        obj.insert(
            "bendPoints".to_string(),
            Value::Array(section.bend_points.iter().copied().map(|p| Value::Object(point_obj(p))).collect()),
        );
    }
    Value::Object(obj)
}

fn point_obj(p: Point) -> serde_json::Map<String, Value> {
    let mut m = serde_json::Map::new();
    m.insert("x".to_string(), Value::from(p.x));
    m.insert("y".to_string(), Value::from(p.y));
    m
}

fn export_label(graph: &elk_core::Graph, label_id: elk_core::LabelId) -> Value {
    let label = &graph.labels[label_id.index()];
    let mut obj = serde_json::Map::new();
    obj.insert("text".to_string(), Value::String(label.text.clone()));
    obj.insert("x".to_string(), Value::from(label.position.x));
    obj.insert("y".to_string(), Value::from(label.position.y));
    obj.insert("width".to_string(), Value::from(label.size.width));
    obj.insert("height".to_string(), Value::from(label.size.height));
    Value::Object(obj)
}

#[cfg(test)]
mod tests {
    use elk_core::{Graph, LayoutEngine, LayoutOptions, Size};
    use elk_layered::LayeredLayoutEngine;

    use super::*;

    #[test]
    fn imports_basic_hierarchy_ports_edges_and_labels() {
        let json = r#"
        {
          "id": "root",
          "children": [
            {
              "id": "a",
              "x": 10, "y": 20, "width": 80, "height": 40,
              "labels": [{"text":"A","width":10,"height":10}],
              "ports": [{"id":"pa","width":8,"height":8}],
              "children": [
                {"id":"a1","x": 5, "y": 6, "width": 30, "height": 20}
              ]
            },
            {
              "id": 2,
              "width": 70, "height": 30,
              "ports": [{"id": 3, "width": 8, "height": 8}]
            }
          ],
          "edges": [
            {"id":"e1","sources":["a"],"targets":[2],"labels":[{"text":"L","width":12,"height":8}]},
            {"id":"e2","source":"a","sourcePort":"pa","target":2,"targetPort":3}
          ]
        }
        "#;

        let imported = import_str(json).expect("import should succeed");
        let g = imported.graph;
        assert_eq!(g.nodes.len(), 4, "root, a, a1, node 2");
        assert_eq!(g.ports.len(), 2, "pa, port 3");
        assert_eq!(g.edges.len(), 2);
        assert_eq!(g.labels.len(), 2);
    }

    #[test]
    fn exported_json_round_trips_basic_graph() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(80.0, 40.0));
        let b = graph.add_node(Size::new(80.0, 40.0));
        graph.add_edge(elk_core::EdgeEndpoint::node(a), elk_core::EdgeEndpoint::node(b));

        let json = export_pretty(&graph);
        let imported = import_str(&json).expect("re-import should work");
        assert_eq!(imported.graph.edges.len(), 1);
        assert_eq!(imported.graph.nodes.len(), 3, "root + 2 nodes");
    }

    #[test]
    fn imported_graph_can_be_laid_out_with_layered() {
        let json = r#"
        {
          "id":"root",
          "children":[
            {"id":"a","width":80,"height":40},
            {"id":"b","width":80,"height":40}
          ],
          "edges":[{"id":"e1","sources":["a"],"targets":["b"]}]
        }
        "#;

        let mut imported = import_str(json).expect("import should succeed").graph;
        let _report = elk_layered::layout(&mut imported, &LayoutOptions::default())
            .expect("layout should succeed");
        let root = imported.nodes[imported.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());
    }
}

