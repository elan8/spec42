use std::collections::BTreeMap;

use elk_core::PortSide;
use elk_graph::{EdgeId, ElkGraph, PortId, PropertyValue};

use crate::pipeline::compound::{
    place_hierarchical_port_on_boundary_at_tangent, CompoundRoutingMap,
};
use crate::pipeline::util::endpoint_abs_center;

/// Java layered runs a dedicated after-phase-5 processor for hierarchical ports.
/// This module provides the same structure for the Rust pipeline:
/// 1. refresh hierarchical dummy coordinates
/// 2. align routed edge terminals to those coordinates
/// 3. correct slanted first/last segments caused by the coordinate fix
/// 4. hide temporary routing-only hierarchical ports
pub(crate) fn run_hierarchical_port_postprocessing(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
) {
    refresh_hierarchical_port_coordinates(graph, map);
    correct_hierarchical_port_route_terminals(graph, map);
    correct_slanted_edge_segments(graph, map);
    hide_temporary_hierarchical_ports(graph, map);
}

pub(crate) fn refresh_hierarchical_port_coordinates(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    let mut tangent_samples: BTreeMap<elk_graph::PortId, (PortSide, elk_graph::NodeId, f32, usize)> =
        BTreeMap::new();

    for record in map.edges.values() {
        for (routed_endpoint, original_endpoint) in [
            (record.routed_source, record.original_source),
            (record.routed_target, record.original_target),
        ] {
            let Some(port_id) = routed_endpoint.port else {
                continue;
            };
            let side = graph.ports[port_id.index()].side;
            let sample = endpoint_abs_center(graph, original_endpoint);
            let tangent = match side {
                PortSide::North | PortSide::South => {
                    sample.x - graph.nodes[routed_endpoint.node.index()].geometry.x
                }
                PortSide::East | PortSide::West => {
                    sample.y - graph.nodes[routed_endpoint.node.index()].geometry.y
                }
            };
            let entry = tangent_samples
                .entry(port_id)
                .or_insert((side, routed_endpoint.node, 0.0, 0));
            entry.2 += tangent;
            entry.3 += 1;
        }
    }

    for (port_id, (side, node_id, tangent_sum, count)) in tangent_samples {
        if count == 0 {
            continue;
        }
        let tangent = tangent_sum / count as f32;
        place_hierarchical_port_on_boundary_at_tangent(graph, node_id, port_id, side, tangent);
    }
}

pub(crate) fn correct_hierarchical_port_route_terminals(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    for (edge_id, record) in &map.edges {
        let section_ids = graph.edges[edge_id.index()].sections.clone();
        if section_ids.is_empty() {
            continue;
        }

        if let Some(port_id) = record.routed_source.port {
            let anchor = endpoint_abs_center(graph, record.routed_source);
            if let Some(first_section_id) = section_ids.first().copied() {
                let section = &mut graph.edge_sections[first_section_id.index()];
                section.start = anchor;
                if let Some(first_bend) = section.bend_points.first_mut() {
                    match graph.ports[port_id.index()].side {
                        PortSide::East | PortSide::West => first_bend.y = anchor.y,
                        PortSide::North | PortSide::South => first_bend.x = anchor.x,
                    }
                }
            }
        }

        if let Some(port_id) = record.routed_target.port {
            let anchor = endpoint_abs_center(graph, record.routed_target);
            if let Some(last_section_id) = section_ids.last().copied() {
                let section = &mut graph.edge_sections[last_section_id.index()];
                section.end = anchor;
                if let Some(last_bend) = section.bend_points.last_mut() {
                    match graph.ports[port_id.index()].side {
                        PortSide::East | PortSide::West => last_bend.y = anchor.y,
                        PortSide::North | PortSide::South => last_bend.x = anchor.x,
                    }
                }
            }
        }
    }
}

pub(crate) fn hide_temporary_hierarchical_ports(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    for (_, port_id) in &map.temporary_ports {
        if let Some(port) = graph.ports.get_mut(port_id.index()) {
            port.properties
                .insert("spec42.temporary_hierarchical_port", PropertyValue::Bool(true));
        }
    }
}

fn correct_slanted_edge_segments(graph: &mut ElkGraph, map: &CompoundRoutingMap) {
    for (edge_id, record) in &map.edges {
        correct_slanted_segment_at_endpoint(graph, *edge_id, record.routed_source.port, true);
        correct_slanted_segment_at_endpoint(graph, *edge_id, record.routed_target.port, false);
    }
}

fn correct_slanted_segment_at_endpoint(
    graph: &mut ElkGraph,
    edge_id: EdgeId,
    port_id: Option<PortId>,
    is_source: bool,
) {
    let Some(port_id) = port_id else {
        return;
    };
    let side = graph.ports[port_id.index()].side;
    let anchor = endpoint_abs_center(
        graph,
        elk_graph::EdgeEndpoint::port(graph.ports[port_id.index()].node, port_id),
    );
    let section_ids = graph.edges[edge_id.index()].sections.clone();
    if section_ids.is_empty() {
        return;
    }
    let section_id = if is_source {
        section_ids.first().copied()
    } else {
        section_ids.last().copied()
    };
    let Some(section_id) = section_id else {
        return;
    };
    let section = &mut graph.edge_sections[section_id.index()];
    let bend = if is_source {
        section.bend_points.first_mut()
    } else {
        section.bend_points.last_mut()
    };
    let Some(bend) = bend else {
        return;
    };
    match side {
        PortSide::East | PortSide::West => bend.y = anchor.y,
        PortSide::North | PortSide::South => bend.x = anchor.x,
    }
}
