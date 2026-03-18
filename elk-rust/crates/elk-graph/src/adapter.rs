use std::collections::BTreeMap;

use elk_core::{
    EdgeEndpoint as CoreEndpoint, EdgeRouting, ElementLayoutOptions, Graph as CoreGraph,
    LayoutDirection, Padding, Point, PortConstraint, Size, Spacing,
};

use crate::{EdgeEndpoint, ElkGraph, PropertyValue, ShapeGeometry};

/// Result of adapting an `elk-graph` model into `elk-core::Graph`.
///
/// The adapter also returns mapping tables to transfer layout results back.
pub struct CoreBridge {
    pub core: CoreGraph,
    pub node_map: Vec<elk_core::NodeId>, // elk-graph NodeId.index -> core NodeId
    pub port_map: Vec<elk_core::PortId>, // elk-graph PortId.index -> core PortId
    pub edge_map: Vec<elk_core::EdgeId>, // elk-graph EdgeId.index -> core EdgeId
}

impl CoreBridge {
    /// Transfer layout geometry from `core` back into `elk-graph`.
    ///
    /// This is a best-effort bridge to keep functional parity tests working during migration.
    pub fn apply_core_layout_to_elk_graph(&self, elk: &mut ElkGraph) {
        for (idx, core_id) in self.node_map.iter().enumerate() {
            let node = &mut elk.nodes[idx];
            let b = self.core.node(*core_id).bounds;
            node.geometry = ShapeGeometry {
                x: b.origin.x,
                y: b.origin.y,
                width: b.size.width,
                height: b.size.height,
            };
        }
        for (idx, core_port_id) in self.port_map.iter().enumerate() {
            let port = &mut elk.ports[idx];
            let b = self.core.port(*core_port_id).bounds;
            port.geometry = ShapeGeometry {
                x: b.origin.x,
                y: b.origin.y,
                width: b.size.width,
                height: b.size.height,
            };
            port.side = self.core.port(*core_port_id).side;
        }
        for (idx, core_edge_id) in self.edge_map.iter().enumerate() {
            let edge = &mut elk.edges[idx];
            let core_edge = self.core.edge(*core_edge_id);
            // Only mirror the first section (elk-core edges currently use Vec<sections> too).
            if let Some(section) = core_edge.sections.first() {
                // Ensure the edge has exactly one section in elk-graph.
                edge.sections.clear();
                let sec_id = crate::EdgeSectionId(elk.edge_sections.len());
                elk.edge_sections.push(crate::EdgeSection {
                    id: sec_id,
                    start: section.start,
                    bend_points: section.bend_points.clone(),
                    end: section.end,
                    properties: Default::default(),
                });
                edge.sections.push(sec_id);
            }
        }
    }
}

/// Adapt an `elk-graph` graph into `elk-core::Graph` so existing layout engines can run.
pub fn to_core_graph(elk: &ElkGraph) -> CoreBridge {
    let mut core = CoreGraph::new();

    // Apply graph-level defaults from ELK properties (root layoutOptions in JSON).
    apply_layout_from_props(&elk.properties, &mut core.layout);

    // Allocate nodes (excluding synthetic root semantics in elk-core; we'll import root as a node too).
    let mut node_map: Vec<elk_core::NodeId> = Vec::with_capacity(elk.nodes.len());
    for node in &elk.nodes {
        let id = core.add_node(Size::new(node.geometry.width, node.geometry.height));
        // Keep preferred position (ELK style) to help stable routing until placement runs.
        core.node_mut(id).preferred_position = Some(Point::new(node.geometry.x, node.geometry.y));
        // Apply per-node options.
        apply_layout_from_props(&node.properties, &mut core.node_mut(id).layout);
        node_map.push(id);
    }

    // Apply hierarchy (parents/children) and sizes.
    for node in &elk.nodes {
        if let Some(parent) = node.parent {
            let child = node_map[node.id.index()];
            let parent_core = node_map[parent.index()];
            core.node_mut(child).parent = Some(parent_core);
            core.node_mut(parent_core).children.push(child);
        }
    }

    // Ports
    let mut port_map: Vec<elk_core::PortId> = Vec::with_capacity(elk.ports.len());
    for port in &elk.ports {
        let owner = node_map[port.node.index()];
        let pid = core.add_port(
            owner,
            port.side,
            Size::new(port.geometry.width, port.geometry.height),
        );
        core.port_mut(pid).bounds.origin = Point::new(port.geometry.x, port.geometry.y);
        // Apply per-port options.
        apply_layout_from_props(&port.properties, &mut core.port_mut(pid).layout);
        // Port ordering index (ELK's `port.index`) maps naturally to model_order in our core model.
        if let Some(order) = prop_usize(&port.properties, &["elk.port.index", "org.eclipse.elk.port.index", "port.index"])
        {
            core.port_mut(pid).layout.model_order = Some(order);
        }
        port_map.push(pid);
    }

    // Labels: create all labels, then attach.
    let mut label_map: Vec<elk_core::LabelId> = Vec::with_capacity(elk.labels.len());
    for label in &elk.labels {
        let lid = core.add_label(
            label.text.clone(),
            Size::new(label.geometry.width, label.geometry.height),
        );
        core.labels[lid.index()].position = Point::new(label.geometry.x, label.geometry.y);
        apply_layout_from_props(&label.properties, &mut core.labels[lid.index()].layout);
        label_map.push(lid);
    }
    // Attach labels to owners.
    for node in &elk.nodes {
        let core_node = node_map[node.id.index()];
        for &label in &node.labels {
            core.node_mut(core_node).labels.push(label_map[label.index()]);
        }
    }
    for port in &elk.ports {
        let core_port = port_map[port.id.index()];
        for &label in &port.labels {
            core.port_mut(core_port).labels.push(label_map[label.index()]);
        }
    }
    for edge in &elk.edges {
        // attach later after core edge created
        let _ = edge;
    }

    // Edges: flatten to single source/target for compatibility with elk-core.
    let mut edge_map: Vec<elk_core::EdgeId> = Vec::with_capacity(elk.edges.len());
    for edge in &elk.edges {
        let source = edge
            .sources
            .first()
            .copied()
            .unwrap_or(EdgeEndpoint::node(elk.root));
        let target = edge
            .targets
            .first()
            .copied()
            .unwrap_or(EdgeEndpoint::node(elk.root));

        let core_source = endpoint_to_core(source, &node_map, &port_map);
        let core_target = endpoint_to_core(target, &node_map, &port_map);
        let eid = core.add_edge(core_source, core_target);
        apply_layout_from_props(&edge.properties, &mut core.edge_mut(eid).layout);
        // If sections exist, transfer first section geometry.
        if let Some(sec_id) = edge.sections.first() {
            let sec = &elk.edge_sections[sec_id.index()];
            core.edge_mut(eid).sections = vec![elk_core::EdgeSection {
                start: sec.start,
                bend_points: sec.bend_points.clone(),
                end: sec.end,
            }];
        }
        // Attach edge labels
        for &label in &edge.labels {
            core.edge_mut(eid).labels.push(label_map[label.index()]);
        }
        edge_map.push(eid);
    }

    // If port constraints indicate fixed ordering, ensure we honor per-port indices by sorting.
    // `elk-core` currently interprets `respect_port_order=false` as “use model_order sorting”.
    if core.layout.port_constraint == Some(PortConstraint::FixedOrder) {
        core.layout.respect_port_order = Some(false);
    }

    CoreBridge {
        core,
        node_map,
        port_map,
        edge_map,
    }
}

fn endpoint_to_core(
    endpoint: EdgeEndpoint,
    node_map: &[elk_core::NodeId],
    port_map: &[elk_core::PortId],
) -> CoreEndpoint {
    let node = node_map[endpoint.node.index()];
    let port = endpoint.port.map(|p| port_map[p.index()]);
    CoreEndpoint { node, port }
}

fn apply_layout_from_props(props: &crate::PropertyBag, out: &mut ElementLayoutOptions) {
    let mut by_key: BTreeMap<String, &PropertyValue> = BTreeMap::new();
    for (k, v) in props.iter() {
        by_key.insert(k.0.to_ascii_lowercase(), v);
    }

    // direction
    if let Some(PropertyValue::String(dir)) = find_value(&by_key, &["elk.direction", "org.eclipse.elk.direction"]) {
        out.direction = parse_direction(dir).or(out.direction);
    }

    // edgeRouting
    if let Some(PropertyValue::String(r)) =
        find_value(&by_key, &["elk.edgerouting", "elk.edgeRouting", "org.eclipse.elk.edgeRouting", "org.eclipse.elk.edgerouting"])
    {
        out.edge_routing = parse_edge_routing(r).or(out.edge_routing);
    }

    // portConstraints
    if let Some(PropertyValue::String(pc)) = find_value(
        &by_key,
        &["elk.portconstraints", "elk.portConstraints", "org.eclipse.elk.portConstraints", "org.eclipse.elk.portconstraints"],
    ) {
        out.port_constraint = parse_port_constraint(pc).or(out.port_constraint);
    }

    // padding (uniform)
    if let Some(value) = find_value(&by_key, &["elk.padding", "org.eclipse.elk.padding"]) {
        if let Some(p) = parse_padding(value) {
            out.padding = Some(p);
        }
    }

    // spacing subset
    if let Some(value) = find_value(&by_key, &["elk.spacing.nodenode", "org.eclipse.elk.spacing.nodenode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.node_spacing = f));
        }
    }
    // Edge-node clearance / separation (best-effort mapping to `edge_spacing` in our simplified model).
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgenode", "org.eclipse.elk.spacing.edgenode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.edge_spacing = f));
        }
    }
    if let Some(value) = find_value(
        &by_key,
        &[
            "elk.spacing.nodenodebetweenlayers",
            "org.eclipse.elk.spacing.nodenodebetweenlayers",
            "org.eclipse.elk.alg.layered.spacing.nodenodebetweenlayers",
        ],
    ) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.layer_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgeedge", "org.eclipse.elk.spacing.edgeedge"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.edge_spacing = f));
        }
    }
    // Edge label spacing (map to `label_spacing`).
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgelabel", "org.eclipse.elk.spacing.edgelabel"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_spacing = f));
        }
    }
    // Node label spacing (map to `label_spacing`).
    if let Some(value) = find_value(&by_key, &["elk.spacing.labelnode", "org.eclipse.elk.spacing.labelnode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_spacing = f));
        }
    }
    // Label clearance (map to `label_clearance`).
    if let Some(value) = find_value(&by_key, &["elk.spacing.labellabel", "org.eclipse.elk.spacing.labellabel"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_clearance = f));
        }
    }
    if let Some(value) = find_value(
        &by_key,
        &["elk.spacing.componentcomponent", "org.eclipse.elk.spacing.componentcomponent"],
    ) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.component_spacing = f));
        }
    }
}

fn find_value<'a>(
    by_key: &'a BTreeMap<String, &'a PropertyValue>,
    keys: &[&str],
) -> Option<&'a PropertyValue> {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            return Some(*v);
        }
    }
    None
}

fn prop_usize(props: &crate::PropertyBag, keys: &[&str]) -> Option<usize> {
    let mut by_key: BTreeMap<String, &PropertyValue> = BTreeMap::new();
    for (k, v) in props.iter() {
        by_key.insert(k.0.to_ascii_lowercase(), v);
    }
    let v = find_value(&by_key, keys)?;
    match v {
        PropertyValue::Int(i) => (*i).try_into().ok(),
        PropertyValue::Float(f) => (*f as i64).try_into().ok(),
        PropertyValue::String(s) => s.trim().parse::<usize>().ok(),
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
        "POLYLINE" | "SPLINES" => Some(EdgeRouting::Straight),
        _ => None,
    }
}

fn parse_port_constraint(value: &str) -> Option<PortConstraint> {
    match value.trim().to_ascii_uppercase().as_str() {
        "FREE" | "UNDEFINED" => Some(PortConstraint::Free),
        "FIXED_SIDE" => Some(PortConstraint::FixedSide),
        "FIXED_ORDER" | "FIXED_RATIO" => Some(PortConstraint::FixedOrder),
        "FIXED_POS" => Some(PortConstraint::FixedPosition),
        _ => None,
    }
}

fn parse_padding(value: &PropertyValue) -> Option<Padding> {
    match value {
        PropertyValue::Int(i) => Some(Padding::uniform(*i as f32)),
        PropertyValue::Float(f) => Some(Padding::uniform(*f as f32)),
        PropertyValue::String(s) => s.trim().parse::<f32>().ok().map(Padding::uniform),
        _ => None,
    }
}

fn parse_f32(value: &PropertyValue) -> Option<f32> {
    match value {
        PropertyValue::Int(i) => Some(*i as f32),
        PropertyValue::Float(f) => Some(*f as f32),
        PropertyValue::String(s) => s.trim().parse::<f32>().ok(),
        _ => None,
    }
}

fn merge_spacing(current: Option<Spacing>, f: impl FnOnce(&mut Spacing)) -> Spacing {
    let mut s = current.unwrap_or_default();
    f(&mut s);
    s
}

