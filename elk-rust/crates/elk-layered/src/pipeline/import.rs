use std::collections::BTreeSet;

use elk_core::{EdgeEndpoint, Graph, LayoutOptions, NodeId, PortConstraint, Size};

use crate::ir::{IrEdge, IrNode, IrNodeKind, IrPortConstraint, LayeredIr};

pub(crate) fn import_graph(
    graph: &Graph,
    nodes: &[NodeId],
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
) -> LayeredIr {
    let mut ir = LayeredIr::new();
    let graph_defaults = options.resolve(&graph.layout);

    for (order, node_id) in nodes.iter().copied().enumerate() {
        let node = graph.node(node_id);
        let node_options = node.layout.inherit_from(&graph_defaults);
        let label_size = combined_label_size(graph, &node.labels);
        let port_label_size = combined_port_label_size(graph, &node.ports);
        let size = Size::new(
            node.bounds
                .size
                .width
                .max(label_size.width)
                .max(port_label_size.width),
            node.bounds.size.height + label_size.height + port_label_size.height,
        );
        let ports = node
            .ports
            .iter()
            .enumerate()
            .map(|(index, port_id)| {
                let port = graph.port(*port_id);
                let port_options = port.layout.inherit_from(&node_options);
                IrPortConstraint {
                    side: port.side,
                    order: if port_options
                        .respect_port_order
                        .unwrap_or(options.layered.respect_port_order)
                    {
                        index
                    } else {
                        port_options.model_order.unwrap_or(index)
                    },
                    constraint: port_options
                        .port_constraint
                        .unwrap_or(PortConstraint::FixedSide),
                }
            })
            .collect();

        ir.push_node(IrNode {
            kind: IrNodeKind::Real(node_id),
            size,
            position: node.preferred_position.unwrap_or_default(),
            layer: 0,
            order,
            label_size,
            ports,
            desired_minor: order as f32 * options.layered.spacing.node_spacing,
            aligned: false,
            model_order: node_options.model_order.unwrap_or(order),
            layer_constraint: node_options.layer_constraint.unwrap_or_default(),
        });
    }

    for (edge_order, edge) in graph.edges.iter().enumerate() {
        let Some(effective_source) = graph.nearest_ancestor_in_set(edge.source.node, local_nodes)
        else {
            continue;
        };
        let Some(effective_target) = graph.nearest_ancestor_in_set(edge.target.node, local_nodes)
        else {
            continue;
        };

        if effective_source == effective_target
            && edge.source.node != effective_source
            && edge.target.node != effective_target
        {
            continue;
        }

        let routed_source = remap_endpoint(edge.source, effective_source);
        let routed_target = remap_endpoint(edge.target, effective_target);
        let label_size = combined_label_size(graph, &edge.labels);
        let edge_options = edge.layout.inherit_from(&graph_defaults);
        ir.edges.push(IrEdge {
            original_edge: edge.id,
            source: edge.source,
            target: edge.target,
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
                && (edge.source.port.is_none()
                    || edge.target.port.is_none()
                    || edge.source.port == edge.target.port),
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

pub(crate) fn combined_label_size(graph: &Graph, label_ids: &[elk_core::LabelId]) -> Size {
    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for label_id in label_ids {
        let label = &graph.labels[label_id.index()];
        width = width.max(label.size.width);
        height += label.size.height;
    }
    Size::new(width, height)
}

fn combined_port_label_size(graph: &Graph, port_ids: &[elk_core::PortId]) -> Size {
    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for port_id in port_ids {
        let port = graph.port(*port_id);
        let size = combined_label_size(graph, &port.labels);
        width = width.max(size.width);
        height = height.max(size.height);
    }
    Size::new(width, height)
}
