#![forbid(unsafe_code)]
#![doc = "Import/export support for ELK Graph JSON."]

use std::collections::HashMap;

use elk_core::{
    EdgeEndpoint, EdgeRouting, ElementLayoutOptions, Graph, HierarchyHandling, LayerConstraint,
    LayoutDirection, NodeAlignment, Padding, Point, PortConstraint, PortSide, Rect, Size, Spacing,
};
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
    pub graph: Graph,
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
    let mut graph = Graph::new();

    // Root is represented as a node in ELK JSON; we import it as a node so edges that reference it
    // can be resolved. Its layout options are treated as *graph-level defaults* (ELK semantics).
    apply_layout_options(root, &mut graph.layout, None, &mut ctx.warnings);
    let root_node = import_node_object(&mut ctx, &mut graph, None, root)?;
    ctx.root = Some(root_node);

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
pub fn export_to_value(graph: &Graph) -> Value {
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

pub fn export_pretty(graph: &Graph) -> String {
    serde_json::to_string_pretty(&export_to_value(graph)).unwrap_or_else(|_| "{}".to_string())
}

#[derive(Default)]
struct ImportCtx {
    root: Option<elk_core::NodeId>,
    warnings: Vec<String>,
    node_ids: HashMap<JsonId, elk_core::NodeId>,
    port_ids: HashMap<JsonId, elk_core::PortId>,
}

fn import_node_object(
    ctx: &mut ImportCtx,
    graph: &mut Graph,
    parent: Option<elk_core::NodeId>,
    obj: &serde_json::Map<String, Value>,
) -> Result<elk_core::NodeId, JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let json_id = JsonId::from_value(id_value)?;

    let w = get_f32(obj, "width").unwrap_or(0.0);
    let h = get_f32(obj, "height").unwrap_or(0.0);
    let node_id = if let Some(parent) = parent {
        graph.add_child_node(parent, Size::new(w, h))
    } else {
        graph.add_node(Size::new(w, h))
    };
    ctx.node_ids.insert(json_id, node_id);

    // Layout options apply to this node.
    let layout = &mut graph.node_mut(node_id).layout;
    apply_layout_options(obj, layout, None, &mut ctx.warnings);

    // Prefer ELK-style relative positioning for hierarchy. Store it as preferred_position.
    let x = get_f32(obj, "x").unwrap_or(0.0);
    let y = get_f32(obj, "y").unwrap_or(0.0);
    graph.node_mut(node_id).preferred_position = Some(Point::new(x, y));

    // Ports
    if let Some(Value::Array(ports)) = obj.get("ports") {
        for port in ports {
            let Some(port_obj) = port.as_object() else {
                continue;
            };
            import_port_object(ctx, graph, node_id, port_obj)?;
        }
    }

    // Labels
    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else {
                continue;
            };
            import_label_object(graph, Owner::Node(node_id), label_obj);
        }
    }

    // Children
    if let Some(Value::Array(children)) = obj.get("children") {
        for child in children {
            let Some(child_obj) = child.as_object() else {
                continue;
            };
            import_node_object(ctx, graph, Some(node_id), child_obj)?;
        }
    }

    Ok(node_id)
}

fn import_port_object(
    ctx: &mut ImportCtx,
    graph: &mut Graph,
    node_id: elk_core::NodeId,
    obj: &serde_json::Map<String, Value>,
) -> Result<elk_core::PortId, JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let json_id = JsonId::from_value(id_value)?;

    let w = get_f32(obj, "width").unwrap_or(0.0);
    let h = get_f32(obj, "height").unwrap_or(0.0);

    // ELK JSON does not encode port side as a dedicated field. Default to East for now.
    let port_id = graph.add_port(node_id, PortSide::East, Size::new(w, h));
    ctx.port_ids.insert(json_id, port_id);

    let x = get_f32(obj, "x").unwrap_or(0.0);
    let y = get_f32(obj, "y").unwrap_or(0.0);
    graph.port_mut(port_id).bounds.origin = Point::new(x, y);

    // Layout options apply to the port; port side can be encoded via `elk.port.side` / `org.eclipse.elk.port.side`.
    let side_override =
        apply_layout_options(obj, &mut graph.port_mut(port_id).layout, Some(OptionTarget::Port), &mut ctx.warnings);
    if let Some(side) = side_override {
        graph.port_mut(port_id).side = side;
    }

    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else {
                continue;
            };
            import_label_object(graph, Owner::Port(port_id), label_obj);
        }
    }

    Ok(port_id)
}

enum Owner {
    Node(elk_core::NodeId),
    Port(elk_core::PortId),
    Edge(elk_core::EdgeId),
}

fn import_label_object(graph: &mut Graph, owner: Owner, obj: &serde_json::Map<String, Value>) {
    let text = obj
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let w = get_f32(obj, "width").unwrap_or(0.0);
    let h = get_f32(obj, "height").unwrap_or(0.0);
    let id = graph.add_label(text, Size::new(w, h));
    let x = get_f32(obj, "x").unwrap_or(0.0);
    let y = get_f32(obj, "y").unwrap_or(0.0);
    graph.labels[id.index()].position = Point::new(x, y);
    apply_layout_options(obj, &mut graph.labels[id.index()].layout, Some(OptionTarget::Label), &mut Vec::new());

    match owner {
        Owner::Node(node_id) => graph.node_mut(node_id).labels.push(id),
        Owner::Port(port_id) => graph.port_mut(port_id).labels.push(id),
        Owner::Edge(edge_id) => graph.edge_mut(edge_id).labels.push(id),
    }
}

fn import_edges_recursive(
    ctx: &mut ImportCtx,
    graph: &mut Graph,
    node_obj: &serde_json::Map<String, Value>,
) -> Result<(), JsonIoError> {
    if let Some(Value::Array(edges)) = node_obj.get("edges") {
        for edge_value in edges {
            let Some(edge_obj) = edge_value.as_object() else {
                continue;
            };
            import_edge_object(ctx, graph, edge_obj)?;
        }
    }

    if let Some(Value::Array(children)) = node_obj.get("children") {
        for child in children {
            let Some(child_obj) = child.as_object() else {
                continue;
            };
            import_edges_recursive(ctx, graph, child_obj)?;
        }
    }

    Ok(())
}

fn import_edge_object(
    ctx: &mut ImportCtx,
    graph: &mut Graph,
    obj: &serde_json::Map<String, Value>,
) -> Result<elk_core::EdgeId, JsonIoError> {
    let id_value = obj.get("id").ok_or(JsonIoError::MissingField("id"))?;
    let _json_id = JsonId::from_value(id_value)?;

    let (source, target, warning) = if obj.contains_key("sources") || obj.contains_key("targets") {
        resolve_modern_edge_endpoints(ctx, graph, obj)?
    } else {
        resolve_legacy_edge_endpoints(ctx, obj)?
    };
    if let Some(w) = warning {
        ctx.warnings.push(w);
    }

    let edge_id = graph.add_edge(source, target);
    apply_layout_options(obj, &mut graph.edge_mut(edge_id).layout, Some(OptionTarget::Edge), &mut ctx.warnings);

    // Labels
    if let Some(Value::Array(labels)) = obj.get("labels") {
        for label in labels {
            let Some(label_obj) = label.as_object() else {
                continue;
            };
            import_label_object(graph, Owner::Edge(edge_id), label_obj);
        }
    }

    // Route sections
    if let Some(section) = import_edge_section(obj) {
        graph.edge_mut(edge_id).sections = vec![section];
    }

    Ok(edge_id)
}

#[derive(Clone, Copy, Debug)]
enum OptionTarget {
    Port,
    Edge,
    Label,
}

/// Extract `layoutOptions` (preferred) or legacy `properties` (fallback) and apply recognized
/// options to the given `ElementLayoutOptions`.
///
/// Returns a port side override when `target == Port` and a known port side option is present.
fn apply_layout_options(
    obj: &serde_json::Map<String, Value>,
    out: &mut ElementLayoutOptions,
    target: Option<OptionTarget>,
    warnings: &mut Vec<String>,
) -> Option<PortSide> {
    let mut merged: Vec<(&str, String)> = Vec::new();

    // Legacy first.
    if let Some(Value::Object(props)) = obj.get("properties") {
        for (k, v) in props {
            if let Some(val) = json_primitive_to_string(v) {
                merged.push((k.as_str(), val));
            }
        }
    }
    // `layoutOptions` overrides legacy `properties`.
    if let Some(Value::Object(opts)) = obj.get("layoutOptions") {
        for (k, v) in opts {
            if let Some(val) = json_primitive_to_string(v) {
                merged.push((k.as_str(), val));
            }
        }
    }

    if merged.is_empty() {
        return None;
    }

    let mut port_side_override = None;

    for (raw_key, raw_val) in merged {
        let key = normalize_option_key(raw_key);
        let val = raw_val.trim();

        // Direction (CoreOptions.DIRECTION) - examples: "elk.direction": "RIGHT"
        if key.ends_with("direction") {
            if let Some(dir) = parse_direction(val) {
                out.direction = Some(dir);
            }
            continue;
        }

        // Edge routing (CoreOptions.EDGE_ROUTING) - values: ORTHOGONAL, POLYLINE, SPLINES
        if key.ends_with("edgerouting") {
            if let Some(r) = parse_edge_routing(val) {
                out.edge_routing = Some(r);
            }
            continue;
        }

        // Port constraints (CoreOptions.PORT_CONSTRAINTS) - values: FREE, FIXED_SIDE, FIXED_ORDER, FIXED_POS, FIXED_RATIO
        if key.ends_with("portconstraints") {
            if let Some(pc) = parse_port_constraints(val) {
                out.port_constraint = Some(pc);
            }
            continue;
        }

        // Layer constraint (Layered) - values: FIRST, LAST, NONE
        if key.ends_with("layerconstraint") {
            if let Some(lc) = parse_layer_constraint(val) {
                out.layer_constraint = Some(lc);
            }
            continue;
        }

        // Hierarchy handling - values often: INCLUDE_CHILDREN / IGNORE_CHILDREN
        if key.ends_with("hierarchyhandling") {
            if let Some(hh) = parse_hierarchy_handling(val) {
                out.hierarchy_handling = Some(hh);
            }
            continue;
        }

        // Node alignment (layered placement) - values: START/CENTER/END/BALANCED
        if key.ends_with("nodealignment") {
            if let Some(na) = parse_node_alignment(val) {
                out.node_alignment = Some(na);
            }
            continue;
        }

        // Padding - common key: "elk.padding" / "org.eclipse.elk.padding"
        if key.ends_with("padding") {
            if let Some(p) = parse_padding(val) {
                out.padding = Some(p);
            }
            continue;
        }

        // Spacing - map a few core spacing keys.
        if key.contains("spacing") {
            // `org.eclipse.elk.spacing.nodeNode`
            if key.ends_with("spacing.nodenode") {
                if let Some(f) = parse_f32(val) {
                    out.spacing = Some(merge_spacing(out.spacing, |s| s.node_spacing = f));
                }
                continue;
            }
            // `org.eclipse.elk.spacing.nodeNodeBetweenLayers` (layered often uses between-layers for layer gap)
            if key.ends_with("spacing.nodenodebetweenlayers") {
                if let Some(f) = parse_f32(val) {
                    out.spacing = Some(merge_spacing(out.spacing, |s| s.layer_spacing = f));
                }
                continue;
            }
            // `org.eclipse.elk.spacing.edgeEdge`
            if key.ends_with("spacing.edgeedge") {
                if let Some(f) = parse_f32(val) {
                    out.spacing = Some(merge_spacing(out.spacing, |s| s.edge_spacing = f));
                }
                continue;
            }
            // `org.eclipse.elk.spacing.componentComponent`
            if key.ends_with("spacing.componentcomponent") {
                if let Some(f) = parse_f32(val) {
                    out.spacing = Some(merge_spacing(out.spacing, |s| s.component_spacing = f));
                }
                continue;
            }
        }

        // Port side (Core option group `port.side`).
        if matches!(target, Some(OptionTarget::Port))
            && (key.ends_with("port.side") || key.ends_with("side"))
        {
            if let Some(side) = parse_port_side(val) {
                port_side_override = Some(side);
            }
            continue;
        }

        // Keep unknown options for later parity work (for now: just surface a warning once in debug scenarios).
        if key.starts_with("elk.") || key.starts_with("org.eclipse.elk.") {
            // Only warn for a small set of keys we *expect* to support soon.
            if matches!(
                key.as_str(),
                "elk.direction"
                    | "elk.edgerouting"
                    | "elk.portconstraints"
                    | "elk.padding"
                    | "elk.spacing.nodenode"
            ) {
                warnings.push(format!("unsupported ELK option {raw_key}={val}"));
            }
        }
    }

    port_side_override
}

fn normalize_option_key(key: &str) -> String {
    // Preserve dots, lower-case, drop whitespace and underscores for forgiving matching.
    key.trim()
        .to_ascii_lowercase()
        .replace('_', "")
        .replace(' ', "")
}

fn json_primitive_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => None,
        _ => None,
    }
}

fn parse_direction(value: &str) -> Option<LayoutDirection> {
    match value.trim().to_ascii_uppercase().as_str() {
        "RIGHT" => Some(LayoutDirection::LeftToRight),
        "LEFT" => Some(LayoutDirection::RightToLeft),
        "DOWN" => Some(LayoutDirection::TopToBottom),
        "UP" => Some(LayoutDirection::BottomToTop),
        _ => None,
    }
}

fn parse_edge_routing(value: &str) -> Option<EdgeRouting> {
    match value.trim().to_ascii_uppercase().as_str() {
        "ORTHOGONAL" => Some(EdgeRouting::Orthogonal),
        // `POLYLINE` and `SPLINES` are approximated as straight segments in the current model.
        "POLYLINE" | "SPLINES" => Some(EdgeRouting::Straight),
        _ => None,
    }
}

fn parse_port_constraints(value: &str) -> Option<PortConstraint> {
    match value.trim().to_ascii_uppercase().as_str() {
        "FREE" | "UNDEFINED" => Some(PortConstraint::Free),
        "FIXED_SIDE" => Some(PortConstraint::FixedSide),
        "FIXED_ORDER" | "FIXED_RATIO" => Some(PortConstraint::FixedOrder),
        "FIXED_POS" => Some(PortConstraint::FixedPosition),
        _ => None,
    }
}

fn parse_layer_constraint(value: &str) -> Option<LayerConstraint> {
    match value.trim().to_ascii_uppercase().as_str() {
        "FIRST" => Some(LayerConstraint::First),
        "LAST" => Some(LayerConstraint::Last),
        "NONE" => Some(LayerConstraint::None),
        _ => None,
    }
}

fn parse_hierarchy_handling(value: &str) -> Option<HierarchyHandling> {
    match value.trim().to_ascii_uppercase().as_str() {
        "INCLUDE_CHILDREN" => Some(HierarchyHandling::IncludeChildren),
        "IGNORE_CHILDREN" => Some(HierarchyHandling::IgnoreChildren),
        _ => None,
    }
}

fn parse_node_alignment(value: &str) -> Option<NodeAlignment> {
    match value.trim().to_ascii_uppercase().as_str() {
        "START" => Some(NodeAlignment::Start),
        "CENTER" => Some(NodeAlignment::Center),
        "END" => Some(NodeAlignment::End),
        "BALANCED" => Some(NodeAlignment::Balanced),
        _ => None,
    }
}

fn parse_padding(value: &str) -> Option<Padding> {
    // Common cases: "12" or "12.0". If ELK serializes a struct, try to recover the first number.
    if let Some(f) = parse_f32(value) {
        return Some(Padding::uniform(f));
    }
    let first_num = value
        .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .find(|s| !s.is_empty())
        .and_then(parse_f32);
    first_num.map(Padding::uniform)
}

fn parse_port_side(value: &str) -> Option<PortSide> {
    match value.trim().to_ascii_uppercase().as_str() {
        "NORTH" => Some(PortSide::North),
        "SOUTH" => Some(PortSide::South),
        "EAST" => Some(PortSide::East),
        "WEST" => Some(PortSide::West),
        _ => None,
    }
}

fn parse_f32(value: &str) -> Option<f32> {
    value.trim().parse::<f32>().ok()
}

fn merge_spacing(current: Option<Spacing>, f: impl FnOnce(&mut Spacing)) -> Spacing {
    let mut s = current.unwrap_or_default();
    f(&mut s);
    s
}

fn resolve_modern_edge_endpoints(
    ctx: &ImportCtx,
    graph: &Graph,
    obj: &serde_json::Map<String, Value>,
) -> Result<(EdgeEndpoint, EdgeEndpoint, Option<String>), JsonIoError> {
    let sources = obj
        .get("sources")
        .and_then(|v| v.as_array())
        .ok_or(JsonIoError::MissingField("sources"))?;
    let targets = obj
        .get("targets")
        .and_then(|v| v.as_array())
        .ok_or(JsonIoError::MissingField("targets"))?;

    let mut warnings = None;

    let source = pick_first_endpoint(ctx, graph, sources).ok_or_else(|| {
        JsonIoError::UnknownReference("edge has no resolvable source endpoint".to_string())
    })?;
    let target = pick_first_endpoint(ctx, graph, targets).ok_or_else(|| {
        JsonIoError::UnknownReference("edge has no resolvable target endpoint".to_string())
    })?;

    if sources.len() > 1 || targets.len() > 1 {
        warnings = Some(format!(
            "hyperedge endpoints not supported yet; using first source/target (sources={}, targets={})",
            sources.len(),
            targets.len()
        ));
    }

    Ok((source, target, warnings))
}

fn resolve_legacy_edge_endpoints(
    ctx: &ImportCtx,
    obj: &serde_json::Map<String, Value>,
) -> Result<(EdgeEndpoint, EdgeEndpoint, Option<String>), JsonIoError> {
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
        EdgeEndpoint {
            node: source_node,
            port: source_port,
        },
        EdgeEndpoint {
            node: target_node,
            port: target_port,
        },
        None,
    ))
}

fn pick_first_endpoint(ctx: &ImportCtx, graph: &Graph, ids: &[Value]) -> Option<EdgeEndpoint> {
    for id in ids {
        let json_id = JsonId::from_value(id).ok()?;
        if let Some(&port_id) = ctx.port_ids.get(&json_id) {
            let node = graph.port(port_id).node;
            return Some(EdgeEndpoint {
                node,
                port: Some(port_id),
            });
        }
        if let Some(&node_id) = ctx.node_ids.get(&json_id) {
            return Some(EdgeEndpoint { node: node_id, port: None });
        }
    }
    None
}

fn import_edge_section(obj: &serde_json::Map<String, Value>) -> Option<elk_core::EdgeSection> {
    // Modern sections encoding: use first section if present.
    if let Some(Value::Array(sections)) = obj.get("sections") {
        if let Some(first) = sections.first().and_then(|v| v.as_object()) {
            let start = first.get("startPoint").and_then(|v| v.as_object())?;
            let end = first.get("endPoint").and_then(|v| v.as_object())?;
            let start_pt = Point::new(get_f32(start, "x").unwrap_or(0.0), get_f32(start, "y").unwrap_or(0.0));
            let end_pt = Point::new(get_f32(end, "x").unwrap_or(0.0), get_f32(end, "y").unwrap_or(0.0));
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
            return Some(elk_core::EdgeSection {
                start: start_pt,
                bend_points: bends,
                end: end_pt,
            });
        }
    }

    // Legacy encoding.
    let has_legacy = obj.contains_key("sourcePoint") || obj.contains_key("targetPoint") || obj.contains_key("bendPoints");
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
    Some(elk_core::EdgeSection {
        start: start_pt,
        bend_points: bends,
        end: end_pt,
    })
}

fn get_f32(map: &serde_json::Map<String, Value>, key: &str) -> Option<f32> {
    map.get(key).and_then(|v| match v {
        Value::Number(n) => n.as_f64().map(|f| f as f32),
        _ => None,
    })
}

fn export_node(
    graph: &Graph,
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
    graph: &Graph,
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
    graph: &Graph,
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
    endpoint: EdgeEndpoint,
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

fn export_label(graph: &Graph, label_id: elk_core::LabelId) -> Value {
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
    use elk_core::{LayoutEngine, LayoutOptions, Size};
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
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));

        let json = export_pretty(&graph);
        let imported = import_str(&json).expect("re-import should work");
        assert_eq!(imported.graph.edges.len(), 1);
        assert_eq!(imported.graph.nodes.len(), 3, "includes imported synthetic root node");
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
        let report = LayeredLayoutEngine::new()
            .layout(&mut imported, &LayoutOptions::default())
            .expect("layout should succeed");
        assert!(report.stats.layers >= 1);
        assert!(imported.bounds.size.width.is_finite());
    }
}

