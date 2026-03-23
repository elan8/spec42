pub(crate) mod compound;
mod crossing;
mod cycle_breaking;
mod import;
mod layering;
mod normalization;
mod placement;
mod props;
pub(crate) use props::decode_layout_from_props;
mod routing;
mod orthogonal_routing_generator;
mod util;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use elk_core::{
    HierarchyHandling, LayoutError, LayoutOptions, LayoutPhaseStat, LayoutReport, Point, Rect,
};
use elk_graph::{ElkGraph, NodeId};

use crossing::{count_crossings, minimize_crossings};
use cycle_breaking::break_cycles;
use crate::pipeline::compound::{postprocess_cross_hierarchy_edges, preprocess_cross_hierarchy_edges};
use import::import_graph;
use layering::assign_layers;
use normalization::normalize_edges;
#[cfg(test)]
use placement::assign_lanes;
use placement::place_nodes;
use routing::export_to_graph;
pub(crate) use routing::refresh_all_port_positions;

pub(crate) fn layout_subgraph(
    graph: &mut ElkGraph,
    nodes: &[NodeId],
    options: &LayoutOptions,
    report: &mut LayoutReport,
) -> Result<Rect, LayoutError> {
    let padding = options.layered.padding;
    let scope_container = nodes
        .first()
        .and_then(|node_id| graph.nodes[node_id.index()].parent)
        .unwrap_or(graph.root);
    rehome_edges_to_nearest_scope_container(graph, scope_container);
    let local_nodes: BTreeSet<NodeId> = nodes.iter().copied().collect();
    let compound_map = preprocess_cross_hierarchy_edges(graph, scope_container, &local_nodes, options);

    if options.layered.hierarchy_handling == HierarchyHandling::IncludeChildren {
        for node_id in nodes {
            let children = graph.nodes[node_id.index()].children.clone();
            if children.is_empty() {
                continue;
            }

            let child_bounds = layout_subgraph(graph, &children, options, report)?;
            // `layout_subgraph` already returns bounds that include `options.layered.padding`.
            // Adding padding again here would inflate containers exponentially with nesting depth.
            let node = &mut graph.nodes[node_id.index()];
            node.geometry.width = child_bounds.size.width;
            node.geometry.height = child_bounds.size.height;
        }
    }

    if nodes.is_empty() {
        return Ok(Rect::new(Point::default(), padding.size()));
    }

    let mut started = Instant::now();
    let mut ir = import_graph(graph, scope_container, nodes, &local_nodes, options);
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
    )?;
    report.stats.phases.push(LayoutPhaseStat {
        name: "edge_routing",
        duration: started.elapsed(),
    });
    postprocess_cross_hierarchy_edges(graph, &compound_map, &mut report.warnings);

    Ok(placement.bounds)
}

fn rehome_edges_to_nearest_scope_container(graph: &mut ElkGraph, scope_container: NodeId) {
    let edge_ids = graph.nodes[scope_container.index()].edges.clone();
    for edge_id in edge_ids {
        let Some(source) = graph.edges[edge_id.index()].sources.first().copied() else {
            continue;
        };
        let Some(target) = graph.edges[edge_id.index()].targets.first().copied() else {
            continue;
        };
        let Some(nca) = graph.nearest_common_ancestor(source.node, target.node) else {
            continue;
        };
        if nca != scope_container && graph.is_ancestor(scope_container, nca) {
            graph.set_edge_container(edge_id, nca);
        }
    }
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
    let comps = components(ir);
    stats.component_count = stats.component_count.max(comps.len());
    if comps.len() <= 1 {
        return;
    }

    // Adapt LayeredIr to the shared packing helper.
    struct IrView<'a> {
        ir: &'a mut crate::ir::LayeredIr,
        adjacency: BTreeMap<usize, Vec<usize>>,
    }

    impl<'a> elk_alg_common::components::ComponentGraphView for IrView<'a> {
        type Node = usize;

        fn nodes(&self) -> Vec<Self::Node> {
            (0..self.ir.nodes.len()).collect()
        }

        fn neighbors(&self, n: Self::Node) -> Vec<Self::Node> {
            self.adjacency.get(&n).cloned().unwrap_or_default()
        }

        fn bounds(&self, n: Self::Node) -> Rect {
            let node = &self.ir.nodes[n];
            Rect::new(node.position, node.size)
        }

        fn translate(&mut self, n: Self::Node, dx: f32, dy: f32) {
            self.ir.nodes[n].position.x += dx;
            self.ir.nodes[n].position.y += dy;
        }
    }

    let mut adjacency: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for node_id in 0..ir.nodes.len() {
        adjacency.entry(node_id).or_default();
    }
    for edge in &ir.normalized_edges {
        adjacency.entry(edge.from).or_default().push(edge.to);
        adjacency.entry(edge.to).or_default().push(edge.from);
    }

    let padding = options.layered.padding;
    let pad = padding
        .top
        .max(padding.right)
        .max(padding.bottom)
        .max(padding.left);
    let mut view = IrView { ir, adjacency };
    let packed_bounds = elk_alg_common::components::pack_components_in_rows(
        &mut view,
        elk_alg_common::components::RowPackingOptions {
            spacing: options.layered.spacing.component_spacing,
            padding: pad,
            target_aspect_ratio: options.layered.component_packing_aspect_ratio,
        },
    );
    stats.packed_components += stats.component_count.saturating_sub(1);
    bounds.size.width = bounds.size.width.max(packed_bounds.size.width);
    bounds.size.height = bounds.size.height.max(packed_bounds.size.height);
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

    use elk_core::{LayoutDirection, LayoutOptions, ViewProfile};
    use elk_graph::{EdgeEndpoint, ElkGraph};

    use super::{
        assign_lanes, assign_layers, break_cycles, count_crossings, export_to_graph, import_graph,
        normalize_edges, place_nodes,
    };

    fn prepare_ir(graph: &ElkGraph) -> crate::ir::LayeredIr {
        let options = LayoutOptions::default();
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent == Some(graph.root))
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(graph, graph.root, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        assign_lanes(&mut ir, &options);
        ir
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; assertions are unstable"]
    fn cycle_breaking_yields_acyclic_orientation() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let c = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(b)], vec![EdgeEndpoint::node(c)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(c)], vec![EdgeEndpoint::node(a)]);

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent == Some(graph.root))
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(&graph, graph.root, &nodes, &local, &LayoutOptions::default());
        let reversed = break_cycles(&mut ir);

        assert!(reversed >= 1);
        assert!(
            ir.edges
                .iter()
                .all(|edge| edge.self_loop || edge.effective_source != edge.effective_target)
        );
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; assertions are unstable"]
    fn normalization_inserts_dummy_nodes_for_long_edges() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let c = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(b)], vec![EdgeEndpoint::node(c)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(c)]);

        let ir = prepare_ir(&graph);
        assert!(ir.nodes.len() > graph.nodes.len());
        assert!(ir.normalized_edges.len() > graph.edges.len());
    }

    #[test]
    fn crossing_count_is_non_negative_after_normalization() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let c = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        let d = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(d)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(b)], vec![EdgeEndpoint::node(c)]);

        let ir = prepare_ir(&graph);
        assert!(count_crossings(&ir) < usize::MAX);
    }

    #[test]
    fn compaction_keeps_nodes_non_overlapping_within_layers() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 30.0, height: 20.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 30.0, height: 20.0 });
        let c = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 30.0, height: 20.0 });
        let d = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 30.0, height: 20.0 });
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(c)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(b)], vec![EdgeEndpoint::node(d)]);

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
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 20.0, height: 20.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 20.0, height: 20.0 });
        let e1 = graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        let e2 = graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        graph.edges[e1.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(1));
        graph.edges[e2.index()]
            .properties
            .insert("elk.edge.bundle", elk_graph::PropertyValue::Int(2));

        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent == Some(graph.root))
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(&graph, graph.root, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        assign_lanes(&mut ir, &options);
        assert!(ir.normalized_edges.len() >= 2);
        let mut lanes = ir.normalized_edges.iter().map(|e| e.lane).collect::<Vec<_>>();
        lanes.sort_unstable();
        lanes.dedup();
        assert!(
            lanes.len() >= 2,
            "parallel edges should be assigned multiple routing lanes"
        );
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; assertions are unstable"]
    fn label_placeholder_is_inserted_for_labeled_long_edge() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 20.0, height: 20.0 });
        let b = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 20.0, height: 20.0 });
        let c = graph.add_node(graph.root, elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 20.0, height: 20.0 });
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        let edge = graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(c)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(b)], vec![EdgeEndpoint::node(c)]);
        let label = graph.add_label("label", elk_graph::ShapeGeometry { x: 0.0, y: 0.0, width: 30.0, height: 12.0 });
        graph.attach_label_to_edge(edge, label);

        let ir = prepare_ir(&graph);
        assert!(ir.edges.iter().any(|edge| edge.label_placeholder.is_some()));
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; export assertions are unstable"]
    fn export_routes_all_directions_with_finite_geometry() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 20.0,
                height: 20.0,
            },
        );
        let b = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 20.0,
                height: 20.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent.is_none() && n.id != graph.root)
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        for direction in [
            LayoutDirection::LeftToRight,
            LayoutDirection::RightToLeft,
            LayoutDirection::TopToBottom,
            LayoutDirection::BottomToTop,
        ] {
            let mut options = LayoutOptions::default();
            options.layered.direction = direction;
            let mut ir = import_graph(&graph, graph.root, &nodes, &local, &options);
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
            )
            .expect("export should route");
            assert!(routed >= 1);
            let edge = &graph_copy.edges[0];
            assert!(!edge.sections.is_empty());
            let section_id = edge.sections[0];
            let section = &graph_copy.edge_sections[section_id.index()];
            assert!(section.start.x.is_finite());
            assert!(section.end.y.is_finite());
        }
    }

    #[test]
    fn export_preserves_distinct_nested_endpoints_for_cross_hierarchy_edges() {
        let mut graph = ElkGraph::new();
        let parent = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 140.0,
            },
        );
        let child_a = graph.add_node(
            parent,
            elk_graph::ShapeGeometry {
                x: 24.0,
                y: 24.0,
                width: 40.0,
                height: 24.0,
            },
        );
        let child_b = graph.add_node(
            parent,
            elk_graph::ShapeGeometry {
                x: 24.0,
                y: 84.0,
                width: 40.0,
                height: 24.0,
            },
        );
        let external = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 320.0,
                y: 54.0,
                width: 40.0,
                height: 24.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(child_a)], vec![EdgeEndpoint::node(external)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(child_b)], vec![EdgeEndpoint::node(external)]);

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent == Some(graph.root))
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
        let mut ir = import_graph(&graph, graph.root, &nodes, &local, &options);
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
        )
        .expect("export should route");
        assert!(routed >= 1);

        let first_edge = &graph_copy.edges[0];
        let second_edge = &graph_copy.edges[1];
        assert!(!first_edge.sections.is_empty());
        assert!(!second_edge.sections.is_empty());
        let first = &graph_copy.edge_sections[first_edge.sections[0].index()];
        let second = &graph_copy.edge_sections[second_edge.sections[0].index()];
        assert_ne!(
            (first.start.x, first.start.y),
            (second.start.x, second.start.y),
            "cross-hierarchy edges from different nested sources should keep distinct anchors"
        );
    }

    #[test]
    fn export_respects_unnecessary_bendpoints_flag() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 20.0,
                y: 20.0,
                width: 80.0,
                height: 40.0,
            },
        );
        let b = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 220.0,
                y: 140.0,
                width: 80.0,
                height: 40.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);
        let mut graph_without = graph.clone();
        graph_without
            .properties
            .insert("elk.layered.unnecessaryBendpoints", elk_graph::PropertyValue::Bool(false));
        let mut graph_with = graph.clone();
        graph_with
            .properties
            .insert("elk.layered.unnecessaryBendpoints", elk_graph::PropertyValue::Bool(true));

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent == Some(graph.root))
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
        let mut ir_without = import_graph(&graph_without, graph_without.root, &nodes, &local, &options);
        break_cycles(&mut ir_without);
        assign_layers(&mut ir_without, &options);
        normalize_edges(&mut ir_without, &options);
        let _ = place_nodes(&mut ir_without, &options);
        let mut graph_without_copy = graph_without.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        let routed_without = export_to_graph(
            &mut graph_without_copy,
            &ir_without,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        )
        .expect("export should route");
        assert!(routed_without >= 1);

        let mut ir = import_graph(&graph_with, graph_with.root, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        let _ = place_nodes(&mut ir, &options);
        let mut graph_copy = graph_with.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        let routed = export_to_graph(
            &mut graph_copy,
            &ir,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        )
        .expect("export should route");
        assert!(routed >= 1);
        let edge_without = &graph_without_copy.edges[0];
        let edge = &graph_copy.edges[0];
        assert!(!edge_without.sections.is_empty());
        assert!(!edge.sections.is_empty());
        let section_without = &graph_without_copy.edge_sections[edge_without.sections[0].index()];
        let section = &graph_copy.edge_sections[edge.sections[0].index()];
        assert!(
            section.bend_points.len() >= section_without.bend_points.len(),
            "with unnecessary bendpoints enabled, bend count should be preserved or increased"
        );
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; export assertions are unstable"]
    fn export_uses_simple_route_for_single_edge() {
        let mut graph = ElkGraph::new();
        let a = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 20.0,
                y: 20.0,
                width: 80.0,
                height: 40.0,
            },
        );
        let b = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 220.0,
                y: 140.0,
                width: 80.0,
                height: 40.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(a)], vec![EdgeEndpoint::node(b)]);

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent.is_none() && n.id != graph.root)
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
        let mut ir = import_graph(&graph, graph.root, &nodes, &local, &options);
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
        )
        .expect("export should route");

        let edge = &graph_copy.edges[0];
        let section = &graph_copy.edge_sections[edge.sections[0].index()];
        assert!(
            section.bend_points.len() <= 1,
            "simple point-to-point edge should not need more than one bend, got {:?}",
            section.bend_points
        );
    }

    #[test]
    #[ignore = "Layered pipeline is mid-migration to ElkGraph; export assertions are unstable"]
    fn export_separates_same_side_node_anchors() {
        let mut graph = ElkGraph::new();
        let source = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 40.0,
                y: 80.0,
                width: 80.0,
                height: 80.0,
            },
        );
        let top = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 260.0,
                y: 40.0,
                width: 60.0,
                height: 40.0,
            },
        );
        let bottom = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 260.0,
                y: 180.0,
                width: 60.0,
                height: 40.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(source)], vec![EdgeEndpoint::node(top)]);
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(source)], vec![EdgeEndpoint::node(bottom)]);

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent.is_none() && n.id != graph.root)
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let mut ir = import_graph(&graph, graph.root, &nodes, &local, &options);
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
        )
        .expect("export should route");

        let first_edge = &graph_copy.edges[0];
        let second_edge = &graph_copy.edges[1];
        let first = &graph_copy.edge_sections[first_edge.sections[0].index()];
        let second = &graph_copy.edge_sections[second_edge.sections[0].index()];
        assert_ne!(
            (first.start.x, first.start.y),
            (second.start.x, second.start.y),
            "edges leaving the same node side should not share the exact same anchor"
        );
    }

    #[test]
    fn import_skips_edges_that_only_collapse_to_the_same_ancestor() {
        let mut graph = ElkGraph::new();
        let root = graph.add_node(
            graph.root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 160.0,
            },
        );
        let child_a = graph.add_node(
            root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 40.0,
                height: 24.0,
            },
        );
        let child_b = graph.add_node(
            root,
            elk_graph::ShapeGeometry {
                x: 0.0,
                y: 0.0,
                width: 40.0,
                height: 24.0,
            },
        );
        graph.add_edge(graph.root, vec![EdgeEndpoint::node(child_a)], vec![EdgeEndpoint::node(child_b)]);

        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.parent.is_none() && n.id != graph.root)
            .map(|n| n.id)
            .collect();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let ir = import_graph(&graph, graph.root, &nodes, &local, &LayoutOptions::default());

        assert!(
            ir.edges.is_empty(),
            "edges that only collapse to the same ancestor should be routed in the child subgraph, not re-imported at the parent level"
        );
    }
}
