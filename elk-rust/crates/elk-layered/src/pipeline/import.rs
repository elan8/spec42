use std::collections::BTreeSet;

use elk_core::{LayoutOptions, PortConstraint, Size};
use elk_graph::{EdgeEndpoint, ElkGraph, NodeId, PortId};

use crate::ir::{IrEdge, IrNode, IrNodeKind, IrPortConstraint, LayeredIr};
use crate::pipeline::compound::TEMP_HIERARCHICAL_DUMMY_NODE_KEY;
use crate::pipeline::props::decode_layout_from_props;
use crate::pipeline::util::{label_size, node_abs_origin};

pub(crate) fn import_graph(
    graph: &ElkGraph,
    scope_container: NodeId,
    nodes: &[NodeId],
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
) -> LayeredIr {
    let mut ir = LayeredIr::new();
    let graph_layout = decode_layout_from_props(&graph.properties);
    let graph_defaults = options.resolve(&graph_layout);

    for (order, node_id) in nodes.iter().copied().enumerate() {
        let node = &graph.nodes[node_id.index()];
        let node_layout = decode_layout_from_props(&node.properties);
        let node_options = node_layout.inherit_from(&graph_defaults);
        let label_size = combined_label_size(graph, &node.labels);
        let port_label_size = combined_port_label_size(graph, &node.ports);
        let size = Size::new(
            node.geometry
                .width
                .max(label_size.width)
                .max(port_label_size.width),
            node.geometry.height + label_size.height + port_label_size.height,
        );
        let ports = node
            .ports
            .iter()
            .enumerate()
            .map(|(index, port_id)| {
                let port = &graph.ports[port_id.index()];
                let port_layout = decode_layout_from_props(&port.properties);
                let port_options = port_layout.inherit_from(&node_options);
                IrPortConstraint {
                    port_id: *port_id,
                    side: port.side,
                    order: if port_options
                        .respect_port_order
                        .unwrap_or(options.layered.respect_port_order)
                    {
                        port_options.model_order.unwrap_or(index)
                    } else {
                        index
                    },
                    constraint: port_options
                        .port_constraint
                        .unwrap_or(PortConstraint::FixedSide),
                }
            })
            .collect();

        // ElkGraph uses relative node geometry; compute absolute origin by walking parents.
        let abs = node_abs_origin(graph, node_id);
        let preferred_minor = imported_node_preferred_minor(graph, node_id, abs, &size, options, order);
        let model_order = imported_node_model_order(graph, node_id, preferred_minor, node_options.model_order, order);
        ir.push_node(IrNode {
            kind: IrNodeKind::Real(node_id),
            size,
            position: abs,
            layer: 0,
            order,
            label_size,
            ports,
            desired_minor: preferred_minor,
            aligned: false,
            model_order,
            layer_constraint: node_options.layer_constraint.unwrap_or_default(),
        });
    }

    for (edge_order, edge_id) in graph.nodes[scope_container.index()].edges.iter().copied().enumerate() {
        let edge = &graph.edges[edge_id.index()];
        let Some(source) = edge.sources.first().copied() else {
            continue;
        };
        let Some(target) = edge.targets.first().copied() else {
            continue;
        };
        if graph.nearest_common_ancestor(source.node, target.node) != Some(scope_container) {
            continue;
        }
        let Some(effective_source) =
            elk_alg_common::graph::nearest_ancestor_in_set(graph, source.node, local_nodes)
        else {
            continue;
        };
        let Some(effective_target) =
            elk_alg_common::graph::nearest_ancestor_in_set(graph, target.node, local_nodes)
        else {
            continue;
        };

        if effective_source == effective_target
            && source.node != effective_source
            && target.node != effective_target
        {
            continue;
        }

        let routed_source = remap_endpoint(source, effective_source);
        let routed_target = remap_endpoint(target, effective_target);
        let label_size = combined_label_size(graph, &edge.labels);
        let edge_layout = decode_layout_from_props(&edge.properties);
        let edge_options = edge_layout.inherit_from(&graph_defaults);
        ir.edges.push(IrEdge {
            original_edge: edge_id,
            source,
            target,
            routed_source,
            routed_target,
            effective_source,
            effective_target,
            reversed: false,
            label_ids: edge.labels.clone(),
            label_size,
            chain: Vec::new(),
            label_placeholder: None,
            // Treat as a self-loop only when it truly loops on a node anchor.
            // Port-to-port connections on the same node should still be routed normally.
            self_loop: effective_source == effective_target
                && (source.port.is_none() || target.port.is_none() || source.port == target.port),
            model_order: edge_options.model_order.unwrap_or(edge_order),
            bundle_key: edge_options.edge_bundle_key,
        });
    }

    ir
}

fn remap_endpoint(endpoint: EdgeEndpoint, mapped_node: NodeId) -> EdgeEndpoint {
    // Preserve port endpoints even when the edge is lifted to an effective ancestor node.
    // Cross-hierarchy routing should still anchor on the actual leaf port positions; otherwise
    // edges degenerate to container/node anchors and won't visually connect to ports.
    if endpoint.port.is_some() {
        return endpoint;
    }
    if endpoint.node == mapped_node {
        endpoint
    } else {
        EdgeEndpoint::node(mapped_node)
    }
}

fn imported_node_preferred_minor(
    graph: &ElkGraph,
    node_id: NodeId,
    abs: elk_core::Point,
    size: &Size,
    options: &LayoutOptions,
    order: usize,
) -> f32 {
    if is_temporary_hierarchical_dummy_node(graph, node_id) {
        return match options.layered.direction {
            elk_core::LayoutDirection::LeftToRight | elk_core::LayoutDirection::RightToLeft => {
                abs.y + size.height / 2.0
            }
            elk_core::LayoutDirection::TopToBottom | elk_core::LayoutDirection::BottomToTop => {
                abs.x + size.width / 2.0
            }
        };
    }
    order as f32 * options.layered.spacing.node_spacing
}

fn imported_node_model_order(
    graph: &ElkGraph,
    node_id: NodeId,
    preferred_minor: f32,
    explicit_model_order: Option<usize>,
    order: usize,
) -> usize {
    if is_temporary_hierarchical_dummy_node(graph, node_id) {
        return preferred_minor.max(0.0).round() as usize;
    }
    explicit_model_order.unwrap_or(order)
}

fn is_temporary_hierarchical_dummy_node(graph: &ElkGraph, node_id: NodeId) -> bool {
    graph.nodes[node_id.index()]
        .properties
        .get(&elk_graph::PropertyKey::from(TEMP_HIERARCHICAL_DUMMY_NODE_KEY))
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

pub(crate) fn combined_label_size(graph: &ElkGraph, label_ids: &[elk_graph::LabelId]) -> Size {
    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for label_id in label_ids {
        let size = label_size(graph, *label_id);
        width = width.max(size.width);
        height += size.height;
    }
    Size::new(width, height)
}

fn combined_port_label_size(graph: &ElkGraph, port_ids: &[PortId]) -> Size {
    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for port_id in port_ids {
        let port = &graph.ports[port_id.index()];
        let size = combined_label_size(graph, &port.labels);
        width = width.max(size.width);
        height = height.max(size.height);
    }
    Size::new(width, height)
}

// moved to `elk-alg-common`

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use elk_core::{LayoutDirection, LayoutOptions, PortSide};
    use elk_graph::{ElkGraph, PropertyValue, ShapeGeometry};

    use super::import_graph;

    #[test]
    fn import_graph_prefers_explicit_port_index_when_order_is_respected() {
        let mut graph = ElkGraph::new();
        let node = graph.add_node(
            graph.root,
            ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 140.0,
                height: 120.0,
            },
        );
        let first = graph.add_port(
            node,
            PortSide::East,
            ShapeGeometry {
                x: 140.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
            },
        );
        let second = graph.add_port(
            node,
            PortSide::East,
            ShapeGeometry {
                x: 140.0,
                y: 80.0,
                width: 10.0,
                height: 10.0,
            },
        );
        graph.ports[first.index()]
            .properties
            .insert("elk.port.index", PropertyValue::Int(2));
        graph.ports[second.index()]
            .properties
            .insert("elk.port.index", PropertyValue::Int(0));

        let options = LayoutOptions {
            layered: elk_core::LayeredOptions {
                direction: LayoutDirection::LeftToRight,
                respect_port_order: true,
                ..LayoutOptions::default().layered
            },
            ..LayoutOptions::default()
        };
        let local_nodes = BTreeSet::from([node]);
        let ir = import_graph(&graph, graph.root, &[node], &local_nodes, &options);
        let east_ports = ir.nodes[ir.real_to_ir[&node]]
            .ports
            .iter()
            .filter(|port| port.side == PortSide::East)
            .collect::<Vec<_>>();

        assert_eq!(east_ports.len(), 2);
        assert_eq!(east_ports[0].port_id, first);
        assert_eq!(east_ports[0].order, 2);
        assert_eq!(east_ports[1].port_id, second);
        assert_eq!(east_ports[1].order, 0);
    }
}
