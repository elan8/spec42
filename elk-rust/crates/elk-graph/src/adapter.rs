use std::collections::BTreeMap;

use elk_core::{EdgeEndpoint as CoreEndpoint, Graph as CoreGraph, Point, Size};

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

    // Allocate nodes (excluding synthetic root semantics in elk-core; we'll import root as a node too).
    let mut node_map: Vec<elk_core::NodeId> = Vec::with_capacity(elk.nodes.len());
    for node in &elk.nodes {
        let id = core.add_node(Size::new(node.geometry.width, node.geometry.height));
        // Keep preferred position (ELK style) to help stable routing until placement runs.
        core.node_mut(id).preferred_position = Some(Point::new(node.geometry.x, node.geometry.y));
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

    // Graph-level options compatibility: map known properties into `core.layout`.
    // This is intentionally small; JSON importer already maps a typed subset into `core.layout` in legacy mode.
    apply_core_layout_defaults_from_properties(elk, &mut core);

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

fn apply_core_layout_defaults_from_properties(elk: &ElkGraph, core: &mut CoreGraph) {
    // Look for ELK-ish keys stored as strings in PropertyBag.
    let mut by_key: BTreeMap<String, &PropertyValue> = BTreeMap::new();
    for (k, v) in elk.properties.iter() {
        by_key.insert(k.0.to_ascii_lowercase(), v);
    }
    if let Some(PropertyValue::String(dir)) = by_key.get("elk.direction") {
        let mapped = match dir.trim().to_ascii_uppercase().as_str() {
            "RIGHT" => Some(elk_core::LayoutDirection::LeftToRight),
            "LEFT" => Some(elk_core::LayoutDirection::RightToLeft),
            "DOWN" => Some(elk_core::LayoutDirection::TopToBottom),
            "UP" => Some(elk_core::LayoutDirection::BottomToTop),
            _ => None,
        };
        if let Some(d) = mapped {
            core.layout.direction = Some(d);
        }
    }
}

