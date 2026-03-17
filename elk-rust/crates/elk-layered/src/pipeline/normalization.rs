use elk_core::{LayerConstraint, LayoutOptions, Point, Size};

use crate::ir::{IrNode, IrNodeKind, LayeredIr, NormalizedEdge};

pub(crate) fn normalize_edges(ir: &mut LayeredIr, options: &LayoutOptions) {
    ir.normalized_edges.clear();

    for edge_index in 0..ir.edges.len() {
        let source_ir = ir.real_to_ir[&ir.edges[edge_index].effective_source];
        let target_ir = ir.real_to_ir[&ir.edges[edge_index].effective_target];
        let source_layer = ir.nodes[source_ir].layer;
        let target_layer = ir.nodes[target_ir].layer;
        ir.edges[edge_index].chain.clear();
        ir.edges[edge_index].label_placeholder = None;

        if ir.edges[edge_index].self_loop {
            continue;
        }

        let mut previous = source_ir;
        let mut segment_order = 0usize;
        if target_layer <= source_layer + 1 {
            ir.normalized_edges.push(NormalizedEdge {
                original_edge: ir.edges[edge_index].original_edge,
                edge_index,
                from: source_ir,
                to: target_ir,
                segment_order,
                lane: 0,
            });
            continue;
        }

        let span = target_layer - source_layer;
        let label_layer = if !ir.edges[edge_index].label_ids.is_empty() {
            Some(source_layer + span / 2)
        } else {
            None
        };

        for intermediate_layer in (source_layer + 1)..target_layer {
            let is_label_placeholder = label_layer == Some(intermediate_layer);
            let placeholder_size = if is_label_placeholder {
                Size::new(
                    ir.edges[edge_index]
                        .label_size
                        .width
                        .max(options.layered.spacing.segment_spacing),
                    ir.edges[edge_index]
                        .label_size
                        .height
                        .max(options.layered.spacing.segment_spacing),
                )
            } else {
                Size::new(
                    options.layered.spacing.segment_spacing,
                    options.layered.spacing.segment_spacing,
                )
            };
            let dummy_id = ir.push_node(IrNode {
                kind: if is_label_placeholder {
                    IrNodeKind::LabelPlaceholder { edge_index }
                } else {
                    IrNodeKind::Dummy {
                        edge_index,
                        segment_index: segment_order,
                    }
                },
                size: placeholder_size,
                position: Point::default(),
                layer: intermediate_layer,
                order: ir.layers[intermediate_layer].len(),
                label_size: if is_label_placeholder {
                    ir.edges[edge_index].label_size
                } else {
                    Size::default()
                },
                ports: Vec::new(),
                desired_minor: 0.0,
                aligned: false,
                model_order: ir.edges[edge_index].model_order,
                layer_constraint: LayerConstraint::None,
            });
            if is_label_placeholder {
                ir.edges[edge_index].label_placeholder = Some(dummy_id);
            }
            ir.layers[intermediate_layer].push(dummy_id);
            ir.edges[edge_index].chain.push(dummy_id);
            ir.normalized_edges.push(NormalizedEdge {
                original_edge: ir.edges[edge_index].original_edge,
                edge_index,
                from: previous,
                to: dummy_id,
                segment_order,
                lane: 0,
            });
            previous = dummy_id;
            segment_order += 1;
        }

        ir.normalized_edges.push(NormalizedEdge {
            original_edge: ir.edges[edge_index].original_edge,
            edge_index,
            from: previous,
            to: target_ir,
            segment_order,
            lane: 0,
        });
    }
}
