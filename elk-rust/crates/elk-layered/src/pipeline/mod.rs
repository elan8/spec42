pub(crate) mod compound;
mod crossing;
mod cycle_breaking;
mod import;
mod layering;
mod normalization;
mod placement;
mod routing;
mod orthogonal_routing_generator;
mod util;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use elk_core::{
    Graph, HierarchyHandling, LayoutDirection, LayoutError, LayoutOptions, LayoutPhaseStat,
    LayoutReport, NodeId, Point, Rect, Size,
};

use crossing::{count_crossings, minimize_crossings};
use cycle_breaking::break_cycles;
use import::import_graph;
use layering::assign_layers;
use normalization::normalize_edges;
#[cfg(test)]
use placement::assign_lanes;
use placement::place_nodes;
use routing::export_to_graph;

pub(crate) fn layout_subgraph(
    graph: &mut Graph,
    nodes: &[NodeId],
    options: &LayoutOptions,
    report: &mut LayoutReport,
) -> Result<Rect, LayoutError> {
    let padding = options.layered.padding;
    let local_nodes: BTreeSet<NodeId> = nodes.iter().copied().collect();

    if options.layered.hierarchy_handling == HierarchyHandling::IncludeChildren {
        for node_id in nodes {
            let children = graph.children_of(*node_id).to_vec();
            if children.is_empty() {
                continue;
            }

            let child_bounds = layout_subgraph(graph, &children, options, report)?;
            let node = graph.node_mut(*node_id);
            node.bounds.size = Size::new(
                child_bounds.size.width + padding.left + padding.right,
                child_bounds.size.height + padding.top + padding.bottom,
            );
        }
    }

    if nodes.is_empty() {
        return Ok(Rect::new(Point::default(), padding.size()));
    }

    let mut started = Instant::now();
    let mut ir = import_graph(graph, nodes, &local_nodes, options);
    report.stats.phases.push(LayoutPhaseStat {
        name: "import_ir",
        duration: started.elapsed(),
    });

    started = Instant::now();
    let reversed_edges = break_cycles(&mut ir);
    report.stats.reversed_edges += reversed_edges;
    report.stats.phases.push(LayoutPhaseStat {
        name: "cycle_breaking",
        duration: started.elapsed(),
    });

    started = Instant::now();
    assign_layers(&mut ir, options);
    report.stats.layers = report.stats.layers.max(ir.layers.len());
    report.stats.phases.push(LayoutPhaseStat {
        name: "layer_assignment",
        duration: started.elapsed(),
    });

    started = Instant::now();
    normalize_edges(&mut ir, options);
    report.stats.normalized_edges += ir.normalized_edges.len();
    report.stats.dummy_nodes += ir
        .nodes
        .iter()
        .filter(|node| !matches!(node.kind, crate::ir::IrNodeKind::Real(_)))
        .count();
    report.stats.phases.push(LayoutPhaseStat {
        name: "dummy_insertion",
        duration: started.elapsed(),
    });

    started = Instant::now();
    let before = count_crossings(&ir);
    minimize_crossings(&mut ir, &mut report.stats.crossing_sweeps);
    let after = count_crossings(&ir);
    report.stats.crossings_before += before;
    report.stats.crossings_after += after;
    report.stats.phases.push(LayoutPhaseStat {
        name: "crossing_minimization",
        duration: started.elapsed(),
    });

    started = Instant::now();
    let mut placement = place_nodes(&mut ir, options);
    report.stats.aligned_nodes += placement.aligned_nodes;
    report.stats.compacted_layers += placement.compacted_layers;
    report.stats.straight_segments += placement.straight_segments;
    if options.layered.component_packing {
        pack_components(&mut ir, options, &mut report.stats, &mut placement.bounds);
    } else {
        report.stats.component_count = report.stats.component_count.max(count_components(&ir));
    }
    report.stats.phases.push(LayoutPhaseStat {
        name: "node_placement",
        duration: started.elapsed(),
    });

    started = Instant::now();
    report.stats.routed_edge_segments += export_to_graph(
        graph,
        &ir,
        &local_nodes,
        options,
        &mut report.warnings,
        &mut report.stats,
    );
    report.stats.phases.push(LayoutPhaseStat {
        name: "edge_routing",
        duration: started.elapsed(),
    });

    Ok(placement.bounds)
}

fn count_components(ir: &crate::ir::LayeredIr) -> usize {
    components(ir).len()
}

fn pack_components(
    ir: &mut crate::ir::LayeredIr,
    options: &LayoutOptions,
    stats: &mut elk_core::LayoutStats,
    bounds: &mut Rect,
) {
    let components = components(ir);
    stats.component_count = stats.component_count.max(components.len());
    if components.len() <= 1 {
        return;
    }

    let direction = options.layered.direction;
    let major_horizontal = matches!(
        direction,
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft
    );
    let mut cursor_minor = if major_horizontal {
        options.layered.padding.top
    } else {
        options.layered.padding.left
    };
    let spacing = options.layered.spacing.component_spacing;

    let mut component_bounds: Vec<_> = components
        .iter()
        .map(|component| {
            let min_x = component
                .iter()
                .map(|node_id| ir.nodes[*node_id].position.x)
                .fold(f32::MAX, f32::min);
            let min_y = component
                .iter()
                .map(|node_id| ir.nodes[*node_id].position.y)
                .fold(f32::MAX, f32::min);
            let max_x = component
                .iter()
                .map(|node_id| ir.nodes[*node_id].position.x + ir.nodes[*node_id].size.width)
                .fold(0.0, f32::max);
            let max_y = component
                .iter()
                .map(|node_id| ir.nodes[*node_id].position.y + ir.nodes[*node_id].size.height)
                .fold(0.0, f32::max);
            (
                Rect::new(
                    Point::new(min_x, min_y),
                    Size::new(max_x - min_x, max_y - min_y),
                ),
                component.clone(),
            )
        })
        .collect();

    component_bounds.sort_by(|(left, _), (right, _)| {
        let left_major = if major_horizontal {
            left.origin.x
        } else {
            left.origin.y
        };
        let right_major = if major_horizontal {
            right.origin.x
        } else {
            right.origin.y
        };
        left_major
            .partial_cmp(&right_major)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut max_minor_extent: f32 = 0.0;
    for (component_rect, component) in component_bounds {
        let current_minor = if major_horizontal {
            component_rect.origin.y
        } else {
            component_rect.origin.x
        };
        let delta_minor = cursor_minor - current_minor;
        for node_id in component {
            if major_horizontal {
                ir.nodes[node_id].position.y += delta_minor;
            } else {
                ir.nodes[node_id].position.x += delta_minor;
            }
        }
        cursor_minor += if major_horizontal {
            component_rect.size.height
        } else {
            component_rect.size.width
        } + spacing;
        max_minor_extent = max_minor_extent.max(cursor_minor);
        stats.packed_components += 1;
    }

    if major_horizontal {
        bounds.size.height = bounds
            .size
            .height
            .max(max_minor_extent + options.layered.padding.bottom);
    } else {
        bounds.size.width = bounds
            .size
            .width
            .max(max_minor_extent + options.layered.padding.right);
    }
}

fn components(ir: &crate::ir::LayeredIr) -> Vec<Vec<usize>> {
    let mut adjacency: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for node_id in 0..ir.nodes.len() {
        adjacency.entry(node_id).or_default();
    }
    for edge in &ir.normalized_edges {
        adjacency.entry(edge.from).or_default().push(edge.to);
        adjacency.entry(edge.to).or_default().push(edge.from);
    }

    let mut seen = BTreeSet::new();
    let mut components = Vec::new();
    for node_id in 0..ir.nodes.len() {
        if seen.contains(&node_id) {
            continue;
        }
        let mut queue = VecDeque::from([node_id]);
        let mut component = Vec::new();
        seen.insert(node_id);
        while let Some(current) = queue.pop_front() {
            component.push(current);
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if seen.insert(*neighbor) {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        components.push(component);
    }
    components
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use elk_core::{EdgeEndpoint, Graph, LayoutDirection, LayoutOptions, Size, ViewProfile};

    use super::{
        assign_lanes, assign_layers, break_cycles, count_crossings, export_to_graph, import_graph,
        normalize_edges, place_nodes,
    };

    fn prepare_ir(graph: &Graph) -> crate::ir::LayeredIr {
        let options = LayoutOptions::default();
        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        assign_lanes(&mut ir, &options);
        ir
    }

    #[test]
    fn cycle_breaking_yields_acyclic_orientation() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(10.0, 10.0));
        let b = graph.add_node(Size::new(10.0, 10.0));
        let c = graph.add_node(Size::new(10.0, 10.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
        graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
        graph.add_edge(EdgeEndpoint::node(c), EdgeEndpoint::node(a));

        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(&graph, &nodes, &local, &LayoutOptions::default());
        let reversed = break_cycles(&mut ir);

        assert!(reversed >= 1);
        assert!(
            ir.edges
                .iter()
                .all(|edge| edge.self_loop || edge.effective_source != edge.effective_target)
        );
    }

    #[test]
    fn normalization_inserts_dummy_nodes_for_long_edges() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(10.0, 10.0));
        let b = graph.add_node(Size::new(10.0, 10.0));
        let c = graph.add_node(Size::new(10.0, 10.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
        graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(c));

        let ir = prepare_ir(&graph);
        assert!(ir.nodes.len() > graph.nodes.len());
        assert!(ir.normalized_edges.len() > graph.edges.len());
    }

    #[test]
    fn crossing_count_is_non_negative_after_normalization() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(10.0, 10.0));
        let b = graph.add_node(Size::new(10.0, 10.0));
        let c = graph.add_node(Size::new(10.0, 10.0));
        let d = graph.add_node(Size::new(10.0, 10.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(d));
        graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));

        let ir = prepare_ir(&graph);
        assert!(count_crossings(&ir) < usize::MAX);
    }

    #[test]
    fn compaction_keeps_nodes_non_overlapping_within_layers() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(30.0, 20.0));
        let b = graph.add_node(Size::new(30.0, 20.0));
        let c = graph.add_node(Size::new(30.0, 20.0));
        let d = graph.add_node(Size::new(30.0, 20.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(c));
        graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(d));

        let mut ir = prepare_ir(&graph);
        let _summary = place_nodes(&mut ir, &LayoutOptions::default());
        for layer in &ir.layers {
            for window in layer.windows(2) {
                assert!(
                    ir.nodes[window[0]].position.y + ir.nodes[window[0]].size.height
                        <= ir.nodes[window[1]].position.y + 80.0
                );
            }
        }
    }

    #[test]
    fn lane_assignment_separates_parallel_segments() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(20.0, 20.0));
        let b = graph.add_node(Size::new(20.0, 20.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));

        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(&graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        assign_lanes(&mut ir, &options);
        assert_eq!(ir.normalized_edges.len(), 2);
        assert_ne!(ir.normalized_edges[0].lane, ir.normalized_edges[1].lane);
    }

    #[test]
    fn label_placeholder_is_inserted_for_labeled_long_edge() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(20.0, 20.0));
        let b = graph.add_node(Size::new(20.0, 20.0));
        let c = graph.add_node(Size::new(20.0, 20.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
        let edge = graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(c));
        graph.add_edge(EdgeEndpoint::node(b), EdgeEndpoint::node(c));
        graph.add_edge_label(edge, "label", Size::new(30.0, 12.0));

        let ir = prepare_ir(&graph);
        assert!(ir.edges.iter().any(|edge| edge.label_placeholder.is_some()));
    }

    #[test]
    fn export_routes_all_directions_with_finite_geometry() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(20.0, 20.0));
        let b = graph.add_node(Size::new(20.0, 20.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));
        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        for direction in [
            LayoutDirection::LeftToRight,
            LayoutDirection::RightToLeft,
            LayoutDirection::TopToBottom,
            LayoutDirection::BottomToTop,
        ] {
            let mut options = LayoutOptions::default();
            options.layered.direction = direction;
            let mut ir = import_graph(&graph, &nodes, &local, &options);
            break_cycles(&mut ir);
            assign_layers(&mut ir, &options);
            normalize_edges(&mut ir, &options);
            let _ = place_nodes(&mut ir, &options);
            let mut graph_copy = graph.clone();
            let mut warnings = Vec::new();
            let mut stats = elk_core::LayoutStats::default();
            let routed = export_to_graph(
                &mut graph_copy,
                &ir,
                &local,
                &options,
                &mut warnings,
                &mut stats,
            );
            assert!(routed >= 1);
            assert!(graph_copy.edges[0].sections[0].start.x.is_finite());
            assert!(graph_copy.edges[0].sections[0].end.y.is_finite());
        }
    }

    #[test]
    fn export_preserves_distinct_nested_endpoints_for_cross_hierarchy_edges() {
        let mut graph = Graph::new();
        let parent = graph.add_node(Size::new(180.0, 140.0));
        let child_a = graph.add_child_node(parent, Size::new(40.0, 24.0));
        let child_b = graph.add_child_node(parent, Size::new(40.0, 24.0));
        let external = graph.add_node(Size::new(40.0, 24.0));
        graph.node_mut(child_a).preferred_position = Some(elk_core::Point::new(24.0, 24.0));
        graph.node_mut(child_b).preferred_position = Some(elk_core::Point::new(24.0, 84.0));
        graph.node_mut(external).preferred_position = Some(elk_core::Point::new(320.0, 54.0));
        graph.add_edge(EdgeEndpoint::node(child_a), EdgeEndpoint::node(external));
        graph.add_edge(EdgeEndpoint::node(child_b), EdgeEndpoint::node(external));

        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
        let mut ir = import_graph(&graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        let _ = place_nodes(&mut ir, &options);
        let mut graph_copy = graph.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        export_to_graph(
            &mut graph_copy,
            &ir,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        );

        let first = &graph_copy.edges[0].sections[0];
        let second = &graph_copy.edges[1].sections[0];
        assert_ne!(
            (first.start.x, first.start.y),
            (second.start.x, second.start.y),
            "cross-hierarchy edges from different nested sources should keep distinct anchors"
        );
    }

    #[test]
    fn export_uses_simple_route_for_single_edge() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(80.0, 40.0));
        let b = graph.add_node(Size::new(80.0, 40.0));
        graph.node_mut(a).preferred_position = Some(elk_core::Point::new(20.0, 20.0));
        graph.node_mut(b).preferred_position = Some(elk_core::Point::new(220.0, 140.0));
        graph.add_edge(EdgeEndpoint::node(a), EdgeEndpoint::node(b));

        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
        let mut ir = import_graph(&graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        let _ = place_nodes(&mut ir, &options);
        let mut graph_copy = graph.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        export_to_graph(
            &mut graph_copy,
            &ir,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        );

        let section = &graph_copy.edges[0].sections[0];
        assert!(
            section.bend_points.len() <= 1,
            "simple point-to-point edge should not need more than one bend, got {:?}",
            section.bend_points
        );
    }

    #[test]
    fn export_separates_same_side_node_anchors() {
        let mut graph = Graph::new();
        let source = graph.add_node(Size::new(80.0, 80.0));
        let top = graph.add_node(Size::new(60.0, 40.0));
        let bottom = graph.add_node(Size::new(60.0, 40.0));
        graph.node_mut(source).preferred_position = Some(elk_core::Point::new(40.0, 80.0));
        graph.node_mut(top).preferred_position = Some(elk_core::Point::new(260.0, 40.0));
        graph.node_mut(bottom).preferred_position = Some(elk_core::Point::new(260.0, 180.0));
        graph.add_edge(EdgeEndpoint::node(source), EdgeEndpoint::node(top));
        graph.add_edge(EdgeEndpoint::node(source), EdgeEndpoint::node(bottom));

        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let mut ir = import_graph(&graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        let _ = place_nodes(&mut ir, &options);
        let mut graph_copy = graph.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        export_to_graph(
            &mut graph_copy,
            &ir,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        );

        let first = &graph_copy.edges[0].sections[0];
        let second = &graph_copy.edges[1].sections[0];
        assert_ne!(
            (first.start.x, first.start.y),
            (second.start.x, second.start.y),
            "edges leaving the same node side should not share the exact same anchor"
        );
    }

    #[test]
    fn import_skips_edges_that_only_collapse_to_the_same_ancestor() {
        let mut graph = Graph::new();
        let root = graph.add_node(Size::new(200.0, 160.0));
        let child_a = graph.add_child_node(root, Size::new(40.0, 24.0));
        let child_b = graph.add_child_node(root, Size::new(40.0, 24.0));
        graph.add_edge(EdgeEndpoint::node(child_a), EdgeEndpoint::node(child_b));

        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let ir = import_graph(&graph, &nodes, &local, &LayoutOptions::default());

        assert!(
            ir.edges.is_empty(),
            "edges that only collapse to the same ancestor should be routed in the child subgraph, not re-imported at the parent level"
        );
    }
}
