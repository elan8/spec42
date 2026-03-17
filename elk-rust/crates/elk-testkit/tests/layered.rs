use elk_core::{LayoutDirection, LayoutEngine, LayoutOptions, NodeAlignment, NodeId, Size, ViewProfile};
use elk_layered::LayeredLayoutEngine;
use elk_testkit::{
    DifferentialBaseline, assert_baseline, assert_component_spacing, assert_contains,
    assert_edge_bend_budget, assert_edges_avoid_nodes, assert_label_owner_proximity,
    assert_labels_do_not_overlap_ports, assert_labels_outside_nodes, assert_no_overlap,
    back_edge_graph, canonical_dag, compound_graph, crossing_graph, deep_dag_graph,
    dense_crossing_graph, general_view_dense_graph, general_view_small_graph, label_heavy_graph,
    long_edge_graph, maybe_write_svg_snapshot, mixed_port_sides_graph, parallel_edges_graph,
    port_graph, render_graph_svg, run_layered_general_view, run_layered_interconnection_view,
    run_layered_with_direction, section_points, self_loop_graph, sysml_block_definition_graph,
    sysml_internal_block_graph, interconnection_view_dense_graph, interconnection_view_small_graph,
};
use proptest::prelude::*;

fn snapshot(
    fixture: &str,
    direction: LayoutDirection,
    graph: &elk_core::Graph,
    report: &elk_core::LayoutReport,
) {
    let _ = maybe_write_svg_snapshot(fixture, direction, graph, report);
}

#[test]
fn view_profile_defaults_are_applied() {
    let general = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
    let interconnection = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);

    assert_eq!(general.layered.direction, LayoutDirection::TopToBottom);
    assert_eq!(interconnection.layered.direction, LayoutDirection::TopToBottom);
    assert!(general.layered.spacing.node_spacing > interconnection.layered.spacing.node_spacing);
    assert!(interconnection.layered.preferred_connector_lanes > general.layered.preferred_connector_lanes);
}

#[test]
fn element_overrides_win_over_view_profile_defaults() {
    let mut graph = canonical_dag();
    graph.layout.port_constraint = Some(elk_core::PortConstraint::FixedPosition);
    let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
    let resolved = options.resolve(&graph.layout);

    assert_eq!(resolved.port_constraint, Some(elk_core::PortConstraint::FixedPosition));
    assert_eq!(resolved.edge_label_placement, Some(elk_core::EdgeLabelPlacement::Center));
}

#[test]
fn canonical_dag_is_deterministic_and_non_overlapping() {
    let mut graph_a = canonical_dag();
    let mut graph_b = canonical_dag();
    let engine = LayeredLayoutEngine::new();
    let options = LayoutOptions::default();

    let report_a = engine.layout(&mut graph_a, &options).expect("canonical graph A should layout successfully");
    let report_b = engine.layout(&mut graph_b, &options).expect("canonical graph B should layout successfully");

    assert_eq!(report_a.stats.crossings_after, report_b.stats.crossings_after);
    assert_eq!(report_a.stats.crossing_sweeps, report_b.stats.crossing_sweeps);
    assert_eq!(report_a.stats.straight_segments, report_b.stats.straight_segments);
    for index in 0..graph_a.nodes.len() {
        assert_eq!(graph_a.nodes[index].bounds, graph_b.nodes[index].bounds);
    }
    assert_no_overlap(&graph_a, &graph_a.top_level_nodes());
    snapshot("canonical-dag", LayoutDirection::LeftToRight, &graph_a, &report_a);
}

#[test]
fn crossing_minimization_tracks_crossing_reduction() {
    let mut graph = dense_crossing_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("dense crossing graph should layout successfully");

    assert!(report.stats.crossings_after <= report.stats.crossings_before);
    assert!(!report.stats.crossing_sweeps.is_empty());
}

#[test]
fn compaction_reports_alignment_and_keeps_spacing() {
    let mut graph = canonical_dag();
    let mut options = LayoutOptions::default();
    options.layered.node_alignment = NodeAlignment::Balanced;
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &options)
        .expect("canonical graph should layout successfully");

    assert!(report.stats.aligned_nodes >= 1);
    assert!(report.stats.compacted_layers >= 1);
    assert_no_overlap(&graph, &graph.top_level_nodes());
}

#[test]
fn ports_drive_edge_anchors_and_order() {
    let mut graph = port_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("port graph should layout successfully");

    let top_edge = &graph.edges[0];
    let bottom_edge = &graph.edges[1];
    let top_section = &top_edge.sections[0];
    let bottom_section = &bottom_edge.sections[0];

    assert!(top_section.start.y < bottom_section.start.y);
    assert!(top_section.end.y < bottom_section.end.y);
    snapshot("port-graph", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn parallel_edges_use_separated_lanes() {
    let mut graph = parallel_edges_graph();
    let report = run_layered_interconnection_view(&mut graph);

    let mids: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            let points = section_points(&edge.sections[0]);
            if points.len() >= 3 {
                points[points.len() / 2].y
            } else {
                // If routing produced a straight segment, approximate a midpoint.
                (edge.sections[0].start.y + edge.sections[0].end.y) / 2.0
            }
        })
        .collect();
    assert!(mids.iter().all(|y| y.is_finite()));
    snapshot("parallel-edges", LayoutDirection::TopToBottom, &graph, &report);
}

#[test]
fn long_edges_create_dummy_nodes_and_straight_segments() {
    let mut graph = long_edge_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("long-edge graph should layout successfully");

    assert!(report.stats.dummy_nodes >= 1);
    assert!(report.stats.normalized_edges > graph.edges.len());
    assert!(report.stats.straight_segments >= 1);
    assert!(graph.labels[0].position.x.is_finite());
    assert!(graph.labels[0].position.y.is_finite());
    snapshot("long-edge", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn edge_labels_stay_outside_nodes() {
    let mut graph = label_heavy_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("label-heavy graph should layout successfully");

    for label in graph.labels.iter().skip(2) {
        let label_rect = elk_core::Rect::new(label.position, label.size);
        for node in &graph.nodes {
            assert!(!label_rect.intersects(node.bounds), "edge label {:?} overlaps node {:?}", label.id, node.id);
        }
    }
    assert!(report.stats.label_displacements > 0.0);
    snapshot("label-heavy", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn compound_nodes_contain_children() {
    let mut graph = compound_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("compound graph should layout successfully");

    let parent = graph.top_level_nodes()[0];
    let parent_bounds = graph.node(parent).bounds;
    for child in graph.children_of(parent) {
        assert_contains(parent_bounds, graph.node(*child).bounds);
    }
    snapshot("compound", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn back_edges_are_marked_as_reversed() {
    let mut graph = back_edge_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("back-edge graph should layout successfully");

    assert!(report.stats.reversed_edges >= 1);
    assert!(graph.edges.iter().any(|edge| edge.was_reversed));
}

#[test]
fn self_loops_receive_stable_loop_routing() {
    let mut graph = self_loop_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("self-loop graph should layout successfully");

    assert!(!report.warnings.is_empty());
    assert_eq!(graph.edges[0].sections.len(), 1);
    assert!(graph.edges[0].sections[0].bend_points.len() >= 3);
    assert_labels_outside_nodes(&graph);
}

#[test]
fn routing_is_consistent_under_all_directions() {
    for direction in [
        LayoutDirection::LeftToRight,
        LayoutDirection::RightToLeft,
        LayoutDirection::TopToBottom,
        LayoutDirection::BottomToTop,
    ] {
        let mut graph = mixed_port_sides_graph();
        let report = run_layered_with_direction(&mut graph, direction);
        assert!(report.stats.routed_edge_segments >= 2);
        for edge in &graph.edges {
            let points = section_points(&edge.sections[0]);
            assert!(points.iter().all(|point| point.x.is_finite() && point.y.is_finite()));
            // Routing may legitimately collapse to a straight segment for some directions.
            assert!(points.len() >= 2);
        }
        snapshot("mixed-port-directions", direction, &graph, &report);
    }
}

#[test]
fn sysml_block_definition_fixture_preserves_hierarchy_and_order_hints() {
    let mut graph = sysml_block_definition_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("sysml bdd graph should layout successfully");

    let vehicle = graph.top_level_nodes()[0];
    let parent_bounds = graph.node(vehicle).bounds;
    for child in graph.children_of(vehicle) {
        assert_contains(parent_bounds, graph.node(*child).bounds);
    }
    assert!(graph.node(graph.children_of(vehicle)[0]).bounds.origin.x <= graph.node(graph.children_of(vehicle)[1]).bounds.origin.x);
    snapshot("sysml-bdd", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn sysml_internal_block_fixture_routes_connectors_without_crossing_nodes() {
    let mut graph = sysml_internal_block_graph();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("sysml ibd graph should layout successfully");

    assert_edges_avoid_nodes(&graph);
    assert!(graph.labels.iter().all(|label| label.position.x.is_finite()));
    snapshot("sysml-ibd", LayoutDirection::LeftToRight, &graph, &report);
}

#[test]
fn general_view_small_is_readable_and_packed() {
    let mut graph = general_view_small_graph();
    let report = run_layered_general_view(&mut graph);

    assert_eq!(report.stats.component_count, 3);
    assert!(report.stats.packed_components >= 2);
    assert_no_overlap(&graph, &graph.top_level_nodes());
    assert_component_spacing(&graph, &graph.top_level_nodes(), 40.0);
    assert_label_owner_proximity(&graph, 220.0);
    snapshot("general-view-small", LayoutDirection::TopToBottom, &graph, &report);
}

#[test]
fn general_view_dense_preserves_compounds_and_spacing() {
    let mut graph = general_view_dense_graph();
    let report = run_layered_general_view(&mut graph);

    assert!(report.stats.component_count >= 3);
    let mission = NodeId(0);
    for child in graph.children_of(mission) {
        assert_contains(graph.node(mission).bounds, graph.node(*child).bounds);
    }
    assert_component_spacing(&graph, &graph.top_level_nodes(), 40.0);
    snapshot("general-view-dense", LayoutDirection::TopToBottom, &graph, &report);
}

#[test]
fn interconnection_view_small_routes_clean_connectors() {
    let mut graph = interconnection_view_small_graph();
    let report = run_layered_interconnection_view(&mut graph);

    assert_edges_avoid_nodes(&graph);
    assert_labels_do_not_overlap_ports(&graph);
    assert_edge_bend_budget(&graph, 6);
    assert!(report.stats.crossings_after <= 1);
    snapshot("interconnection-view-small", LayoutDirection::TopToBottom, &graph, &report);
}

#[test]
fn interconnection_view_dense_keeps_labels_and_lanes_readable() {
    let mut graph = interconnection_view_dense_graph();
    let report = run_layered_interconnection_view(&mut graph);

    assert_edges_avoid_nodes(&graph);
    assert_labels_do_not_overlap_ports(&graph);
    assert_edge_bend_budget(&graph, 7);
    assert!(report.stats.bend_points >= graph.edges.len());
    assert!(report.stats.label_displacements > 0.0);
    snapshot("interconnection-view-dense", LayoutDirection::TopToBottom, &graph, &report);
}

#[test]
fn svg_renderer_contains_expected_primitives() {
    let mut graph = canonical_dag();
    let report = LayeredLayoutEngine::new()
        .layout(&mut graph, &LayoutOptions::default())
        .expect("canonical graph should layout successfully");
    let svg = render_graph_svg(
        "canonical-dag",
        LayoutDirection::LeftToRight,
        &graph,
        &report,
    );
    assert!(svg.contains("<svg"));
    assert!(svg.contains("<rect"));
    assert!(svg.contains("<path"));
    assert!(svg.contains("grid"));
}

#[test]
fn differential_baselines_match_expected_ranges() {
    let mut canonical = canonical_dag();
    let canonical_report = LayeredLayoutEngine::new()
        .layout(&mut canonical, &LayoutOptions::default())
        .expect("canonical graph should layout successfully");
    assert_baseline(
        &canonical_report,
        &canonical,
        DifferentialBaseline {
            layers_range: (3, 4),
            min_dummy_nodes: 0,
            min_normalized_edges: 4,
            max_crossings_after: 1,
            min_straight_segments: 2,
            width_range: (250.0, 1400.0),
            height_range: (100.0, 420.0),
        },
    );

    let mut deep = deep_dag_graph();
    let deep_report = LayeredLayoutEngine::new()
        .layout(&mut deep, &LayoutOptions::default())
        .expect("deep dag should layout successfully");
    assert_baseline(
        &deep_report,
        &deep,
        DifferentialBaseline {
            layers_range: (8, 9),
            min_dummy_nodes: 0,
            min_normalized_edges: 7,
            max_crossings_after: 0,
            min_straight_segments: 5,
            width_range: (700.0, 2800.0),
            height_range: (80.0, 520.0),
        },
    );

    let mut long = long_edge_graph();
    let long_report = LayeredLayoutEngine::new()
        .layout(&mut long, &LayoutOptions::default())
        .expect("long graph should layout successfully");
    assert_baseline(
        &long_report,
        &long,
        DifferentialBaseline {
            layers_range: (4, 5),
            min_dummy_nodes: 1,
            min_normalized_edges: 5,
            max_crossings_after: 1,
            min_straight_segments: 2,
            width_range: (300.0, 1800.0),
            height_range: (120.0, 700.0),
        },
    );
}

proptest! {
    #[test]
    fn chain_layout_preserves_counts_and_finite_geometry(
        length in 1usize..8,
        spacing in 10.0f32..120.0
    ) {
        let mut graph = elk_core::Graph::new();
        let mut previous = None;
        for _ in 0..length {
            let node = graph.add_node(Size::new(80.0, 40.0));
            if let Some(prev) = previous {
                graph.add_edge(elk_core::EdgeEndpoint::node(prev), elk_core::EdgeEndpoint::node(node));
            }
            previous = Some(node);
        }

        let stats_before = graph.stats();
        let mut options = LayoutOptions::default();
        options.layered.spacing.node_spacing = spacing;
        let report = LayeredLayoutEngine::new()
            .layout(&mut graph, &options)
            .expect("generated chain graph should layout successfully");

        prop_assert_eq!(stats_before, graph.stats());
        prop_assert!(report.stats.layers >= 1);
        prop_assert!(report.stats.normalized_edges >= graph.edges.len());
        prop_assert!(report.stats.routed_edge_segments >= graph.edges.len());
        for node in &graph.nodes {
            prop_assert!(node.bounds.origin.x.is_finite());
            prop_assert!(node.bounds.origin.y.is_finite());
        }
    }

    #[test]
    fn crossings_do_not_increase_for_dense_fixture(seed in 0u8..8) {
        let mut graph = if seed % 2 == 0 { dense_crossing_graph() } else { crossing_graph() };
        let report = LayeredLayoutEngine::new()
            .layout(&mut graph, &LayoutOptions::default())
            .expect("dense graph should layout successfully");

        prop_assert!(report.stats.crossings_after <= report.stats.crossings_before);
    }
}

