use std::fs;

use elk_core::{LayoutDirection, LayoutOptions, Point, Rect, Size, ViewProfile};
use elk_graph_json::import_str;
use elk_layered::layout;

fn read_fixture(name: &str) -> String {
    let path = format!("{}/fixtures/elkjson/{}", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(path).expect("fixture should be readable")
}

fn assert_finite_geometry(g: &elk_graph::ElkGraph) {
    for n in &g.nodes {
        assert!(n.geometry.x.is_finite());
        assert!(n.geometry.y.is_finite());
        assert!(n.geometry.width.is_finite());
        assert!(n.geometry.height.is_finite());
    }
    for e in &g.edges {
        for sid in &e.sections {
            let s = &g.edge_sections[sid.index()];
            assert!(s.start.x.is_finite() && s.start.y.is_finite());
            assert!(s.end.x.is_finite() && s.end.y.is_finite());
            for p in &s.bend_points {
                assert!(p.x.is_finite() && p.y.is_finite());
            }
        }
    }
}

fn rects_overlap(a: &elk_graph::ShapeGeometry, b: &elk_graph::ShapeGeometry) -> bool {
    let ax2 = a.x + a.width;
    let ay2 = a.y + a.height;
    let bx2 = b.x + b.width;
    let by2 = b.y + b.height;
    a.x < bx2 && b.x < ax2 && a.y < by2 && b.y < ay2
}

fn assert_children_non_overlapping(g: &elk_graph::ElkGraph, fixture: &str) {
    for parent in &g.nodes {
        let children = &parent.children;
        for i in 0..children.len() {
            for j in (i + 1)..children.len() {
                let a_node = &g.nodes[children[i].index()];
                let b_node = &g.nodes[children[j].index()];
                let a = &a_node.geometry;
                let b = &b_node.geometry;
                assert!(
                    !rects_overlap(a, b),
                    "child nodes overlap in fixture {} parent {:?}: child_a={:?} geom_a=({:.1},{:.1},{:.1},{:.1}) child_b={:?} geom_b=({:.1},{:.1},{:.1},{:.1})",
                    fixture,
                    parent.id
                    ,
                    a_node.id,
                    a.x,
                    a.y,
                    a.width,
                    a.height,
                    b_node.id,
                    b.x,
                    b.y,
                    b.width,
                    b.height
                );
            }
        }
    }
}

fn assert_children_within_parent_bounds(g: &elk_graph::ElkGraph) {
    let mut cache = vec![None; g.nodes.len()];
    for parent in &g.nodes {
        if parent.id == g.root {
            continue;
        }
        let (parent_x, parent_y) = absolute_node_origin(g, parent.id, &mut cache);
        let px0 = parent_x;
        let py0 = parent_y;
        let px1 = px0 + parent.geometry.width;
        let py1 = py0 + parent.geometry.height;
        for child_id in &parent.children {
            let child = &g.nodes[child_id.index()];
            let (child_x, child_y) = absolute_node_origin(g, child.id, &mut cache);
            let cx0 = child_x;
            let cy0 = child_y;
            let cx1 = cx0 + child.geometry.width;
            let cy1 = cy0 + child.geometry.height;
            assert!(
                cx0 >= px0 - 1e-3 && cy0 >= py0 - 1e-3 && cx1 <= px1 + 1e-3 && cy1 <= py1 + 1e-3,
                "child {:?} lies outside parent {:?}: parent=({}, {}, {}, {}), child=({}, {}, {}, {})",
                child.id,
                parent.id,
                px0,
                py0,
                px1,
                py1,
                cx0,
                cy0,
                cx1,
                cy1
            );
        }
    }
}

fn absolute_node_origin(
    g: &elk_graph::ElkGraph,
    node_id: elk_graph::NodeId,
    cache: &mut [Option<(f32, f32)>],
) -> (f32, f32) {
    if let Some(point) = cache[node_id.index()] {
        return point;
    }
    let node = &g.nodes[node_id.index()];
    let local = (node.geometry.x, node.geometry.y);
    let absolute = match node.parent {
        Some(parent_id) if parent_id != g.root => {
            let parent = absolute_node_origin(g, parent_id, cache);
            (parent.0 + local.0, parent.1 + local.1)
        }
        _ => local,
    };
    cache[node_id.index()] = Some(absolute);
    absolute
}

fn expected_port_centers(
    g: &elk_graph::ElkGraph,
    port_id: elk_graph::PortId,
    cache: &mut [Option<(f32, f32)>],
) -> [(f32, f32); 2] {
    let port = &g.ports[port_id.index()];
    let raw_x = port.geometry.x + port.geometry.width / 2.0;
    let raw_y = port.geometry.y + port.geometry.height / 2.0;
    let (node_x, node_y) = absolute_node_origin(g, port.node, cache);
    let local_x = node_x + raw_x;
    let local_y = node_y + raw_y;
    [(raw_x, raw_y), (local_x, local_y)]
}

fn assert_edge_endpoints_match_declared_ports(g: &elk_graph::ElkGraph, tolerance: f32) {
    let mut cache = vec![None; g.nodes.len()];
    for edge in &g.edges {
        if edge.sections.is_empty() {
            continue;
        }
        if let Some(source) = edge.sources.first() {
            if let Some(source_port) = source.port {
                let candidates = expected_port_centers(g, source_port, &mut cache);
                let best = edge
                    .sections
                    .iter()
                    .flat_map(|sid| {
                        let section = &g.edge_sections[sid.index()];
                        candidates.into_iter().map(move |(expected_x, expected_y)| {
                            let start_dx = (section.start.x - expected_x).abs();
                            let start_dy = (section.start.y - expected_y).abs();
                            let end_dx = (section.end.x - expected_x).abs();
                            let end_dy = (section.end.y - expected_y).abs();
                            if start_dx + start_dy <= end_dx + end_dy {
                                (start_dx, start_dy, expected_x, expected_y)
                            } else {
                                (end_dx, end_dy, expected_x, expected_y)
                            }
                        })
                    })
                    .min_by(|(adx, ady, _, _), (bdx, bdy, _, _)| {
                        (adx + ady)
                            .partial_cmp(&(bdx + bdy))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .expect("edge has sections");
                let dx = best.0;
                let dy = best.1;
                let expected_x = best.2;
                let expected_y = best.3;
                assert!(
                    dx <= tolerance && dy <= tolerance,
                    "edge {:?} source endpoint drifted from source port {:?}: expected=({:.2},{:.2}) got=({:.2},{:.2})",
                    edge.id,
                    source_port,
                    expected_x,
                    expected_y,
                    expected_x + dx,
                    expected_y + dy
                );
            }
        }
        if let Some(target) = edge.targets.first() {
            if let Some(target_port) = target.port {
                let candidates = expected_port_centers(g, target_port, &mut cache);
                let best = edge
                    .sections
                    .iter()
                    .flat_map(|sid| {
                        let section = &g.edge_sections[sid.index()];
                        candidates.into_iter().map(move |(expected_x, expected_y)| {
                            let start_dx = (section.start.x - expected_x).abs();
                            let start_dy = (section.start.y - expected_y).abs();
                            let end_dx = (section.end.x - expected_x).abs();
                            let end_dy = (section.end.y - expected_y).abs();
                            if start_dx + start_dy <= end_dx + end_dy {
                                (start_dx, start_dy, expected_x, expected_y)
                            } else {
                                (end_dx, end_dy, expected_x, expected_y)
                            }
                        })
                    })
                    .min_by(|(adx, ady, _, _), (bdx, bdy, _, _)| {
                        (adx + ady)
                            .partial_cmp(&(bdx + bdy))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .expect("edge has sections");
                let dx = best.0;
                let dy = best.1;
                let expected_x = best.2;
                let expected_y = best.3;
                assert!(
                    dx <= tolerance && dy <= tolerance,
                    "edge {:?} target endpoint drifted from target port {:?}: expected=({:.2},{:.2}) got=({:.2},{:.2})",
                    edge.id,
                    target_port,
                    expected_x,
                    expected_y,
                    expected_x + dx,
                    expected_y + dy
                );
            }
        }
    }
}

fn total_bend_count(g: &elk_graph::ElkGraph) -> usize {
    g.edges
        .iter()
        .flat_map(|e| e.sections.iter())
        .map(|sid| g.edge_sections[sid.index()].bend_points.len())
        .sum()
}

fn max_bends_per_edge(g: &elk_graph::ElkGraph) -> usize {
    g.edges
        .iter()
        .map(|e| {
            e.sections
                .iter()
                .map(|sid| g.edge_sections[sid.index()].bend_points.len())
                .sum::<usize>()
        })
        .max()
        .unwrap_or(0)
}

fn route_signature(g: &elk_graph::ElkGraph) -> Vec<(String, usize, usize)> {
    let mut out: Vec<(String, usize, usize)> = g
        .edges
        .iter()
        .map(|e| {
            let bends = e
                .sections
                .iter()
                .map(|sid| g.edge_sections[sid.index()].bend_points.len())
                .sum::<usize>();
            (format!("{:?}", e.id), e.sections.len(), bends)
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn node_rect(g: &elk_graph::ElkGraph, id: elk_graph::NodeId) -> Rect {
    let geom = &g.nodes[id.index()].geometry;
    Rect::new(
        Point::new(geom.x, geom.y),
        Size::new(geom.width.max(0.0), geom.height.max(0.0)),
    )
}

fn count_non_endpoint_node_intrusions(g: &elk_graph::ElkGraph, tolerance: f32) -> usize {
    let mut intrusions = 0usize;
    for edge in &g.edges {
        let source = edge.sources.first().map(|s| s.node);
        let target = edge.targets.first().map(|t| t.node);
        let source_target: Vec<_> = source
            .into_iter()
            .chain(target)
            .filter(|n| *n != g.root)
            .collect();
        for sid in &edge.sections {
            let sec = &g.edge_sections[sid.index()];
            let points: Vec<Point> = std::iter::once(sec.start)
                .chain(sec.bend_points.iter().copied())
                .chain(std::iter::once(sec.end))
                .collect();
            for seg in points.windows(2) {
                let a = seg[0];
                let b = seg[1];
                for node in &g.nodes {
                    if node.id == g.root || source_target.contains(&node.id) {
                        continue;
                    }
                    if source_target.iter().any(|ep| g.is_ancestor(node.id, *ep)) {
                        continue;
                    }
                    if elk_testkit::segment_intersects_rect_interior(a, b, &node_rect(g, node.id), tolerance)
                    {
                        intrusions += 1;
                    }
                }
            }
        }
    }
    intrusions
}

fn count_diagonal_segments(g: &elk_graph::ElkGraph, tolerance: f32) -> usize {
    let mut diagonals = 0usize;
    for edge in &g.edges {
        for sid in &edge.sections {
            let sec = &g.edge_sections[sid.index()];
            let points: Vec<Point> = std::iter::once(sec.start)
                .chain(sec.bend_points.iter().copied())
                .chain(std::iter::once(sec.end))
                .collect();
            for seg in points.windows(2) {
                let a = seg[0];
                let b = seg[1];
                let dx = (a.x - b.x).abs();
                let dy = (a.y - b.y).abs();
                if dx > tolerance && dy > tolerance {
                    diagonals += 1;
                }
            }
        }
    }
    diagonals
}

#[test]
fn view_profile_defaults_are_applied() {
    let general = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
    let interconnection = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);

    assert_eq!(general.layered.direction, LayoutDirection::TopToBottom);
    assert_eq!(interconnection.layered.direction, LayoutDirection::LeftToRight);
    assert!(general.layered.spacing.node_spacing > interconnection.layered.spacing.node_spacing);
    assert!(
        interconnection.layered.preferred_connector_lanes
            > general.layered.preferred_connector_lanes
    );
}

#[test]
fn layered_layout_runs_on_fixture_graphs() {
    for fixture in [
        "direction_down.json",
        "ports_and_constraints.json",
        "port_order_index.json",
        "layer_spacing_large.json",
    ] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let report = layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
        assert!(report.stats.layers >= 1);

        // Bounds are stored on the synthetic root node geometry.
        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());
    }
}

#[test]
fn tree_layout_runs_on_fixture_graphs_via_service() {
    use elk_graph_json::import_str;

    for fixture in ["tree_simple.json", "tree_wide.json", "tree_compound.json"] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let report = elk_service::LayoutService::default_registry()
            .layout(&mut g, &LayoutOptions::default())
            .expect("tree layout should succeed");

        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());

        // Ensure at least one edge was routed when edges exist.
        if !g.edges.is_empty() {
            assert!(
                g.edges.iter().any(|e| !e.sections.is_empty()),
                "expected routed edge sections"
            );
        }

        // Avoid unused warning on report.
        let _ = report;
    }
}

#[test]
fn rectpacking_layout_runs_and_is_non_overlapping() {
    use elk_graph_json::import_str;

    let json = read_fixture("rectpacking_simple.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("rectpacking layout should succeed");

    // Non-overlap for top-level nodes.
    let nodes: Vec<_> = g
        .nodes
        .iter()
        .filter(|n| n.parent == Some(g.root) && n.id != g.root)
        .map(|n| n.id)
        .collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = &g.nodes[nodes[i].index()].geometry;
            let b = &g.nodes[nodes[j].index()].geometry;
            let overlap_x = a.x < b.x + b.width && b.x < a.x + a.width;
            let overlap_y = a.y < b.y + b.height && b.y < a.y + a.height;
            assert!(
                !(overlap_x && overlap_y),
                "expected no overlap between {:?} and {:?}",
                nodes[i],
                nodes[j]
            );
        }
    }

    let root = g.nodes[g.root.index()].geometry;
    assert!(root.width.is_finite());
    assert!(root.height.is_finite());
}

#[test]
fn topdownpacking_layout_runs_on_compound_fixture() {
    let json = read_fixture("topdownpacking_compound.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("topdownpacking layout should succeed");

    let root = g.nodes[g.root.index()].geometry;
    assert!(root.width.is_finite());
    assert!(root.height.is_finite());
    assert!(root.width >= 1.0);
    assert!(root.height >= 1.0);

    // All nodes should have finite geometry.
    for node in &g.nodes {
        assert!(node.geometry.width.is_finite());
        assert!(node.geometry.height.is_finite());
    }
}

#[test]
fn libavoid_routes_edges_and_avoids_obstacles() {
    let json = read_fixture("libavoid_obstacles.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("libavoid layout should succeed");

    // Every edge should have at least one section with finite geometry.
    for edge in &g.edges {
        assert!(!edge.sections.is_empty(), "each edge should have routed sections");
        for &sid in &edge.sections {
            let s = &g.edge_sections[sid.index()];
            assert!(s.start.x.is_finite());
            assert!(s.start.y.is_finite());
            assert!(s.end.x.is_finite());
            assert!(s.end.y.is_finite());
        }
    }

    // Invariant: no routed segment may cross the interior of any node (except the edge endpoints).
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

#[test]
fn libavoid_narrow_corridor_avoids_obstacles() {
    let json = read_fixture("libavoid_narrow.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("libavoid layout should succeed");
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

#[test]
fn layered_with_libavoid_backend_routes_and_avoids_obstacles() {
    let json = read_fixture("layered_libavoid.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    layout(&mut g, &LayoutOptions::default()).expect("layered layout should succeed");

    assert!(
        g.edges.iter().any(|e| !e.sections.is_empty()),
        "at least one edge should have routed sections"
    );
    for edge in &g.edges {
        assert!(!edge.sections.is_empty(), "each edge should have routed sections");
    }
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

#[test]
fn interconnection_real_corpus_invariants_hold() {
    for fixture in [
        "interconnection_real_small.json",
        "interconnection_real_medium.json",
        "interconnection_real_dense.json",
        "interconnection_real_full_drone_like.json",
    ] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let report = layout(&mut g, &options).expect("layout should succeed");

        assert_finite_geometry(&g);
        assert_children_non_overlapping(&g, fixture);
        assert_children_within_parent_bounds(&g);
        assert_edge_endpoints_match_declared_ports(&g, 1.0);
        let diagonal_segments = count_diagonal_segments(&g, 1e-3);
        let diagonal_limit = if fixture == "interconnection_real_full_drone_like.json" {
            40
        } else {
            4
        };
        assert!(
            diagonal_segments <= diagonal_limit,
            "expected low diagonal-segment count for {} (diagonals={}, limit={})",
            fixture,
            diagonal_segments,
            diagonal_limit
        );
        let max_bends = max_bends_per_edge(&g);
        let bend_limit = if fixture == "interconnection_real_full_drone_like.json" {
            30
        } else {
            9
        };
        assert!(
            max_bends <= bend_limit,
            "unexpectedly high bend count for {} (max_bends={}, limit={})",
            fixture,
            max_bends,
            bend_limit
        );
        if fixture == "interconnection_real_full_drone_like.json" {
            let intrusions = count_non_endpoint_node_intrusions(&g, 1e-3);
            assert!(
                total_bend_count(&g) <= 400,
                "global bend budget exceeded for {} (total_bend_count={}, limit={})",
                fixture,
                total_bend_count(&g),
                400
            );
            assert!(
                max_bends <= bend_limit,
                "per-edge bend cap exceeded for {}",
                fixture
            );
            assert!(
                intrusions <= 24,
                "severe edge-through-node intrusions exceeded for {} (intrusions={})",
                fixture,
                intrusions
            );
            assert!(
                !report
                    .warnings
                    .iter()
                    .any(|w| w.contains("endpoint out-of-bounds")),
                "unexpected libavoid endpoint out-of-bounds warning for {}: {:?}",
                fixture,
                report.warnings
            );
            assert!(
                !report
                    .warnings
                    .iter()
                    .any(|w| w.contains("canonicalization_skipped_large_delta")),
                "unexpected large-delta canonicalization skip warning for {}: {:?}",
                fixture,
                report.warnings
            );
            assert!(
                !report.warnings.iter().any(|w| {
                    w.contains("libavoid terminal canonicalization adjusted edge")
                }),
                "unexpected mixed-frame canonicalization adjustment for {}: {:?}",
                fixture,
                report.warnings
            );
            assert!(
                !report.warnings.iter().any(|w| {
                    w.contains("simplified router fallback active")
                        || w.contains("fallback-reason=hard_error")
                }),
                "unexpected fallback activation for {}: {:?}",
                fixture,
                report.warnings
            );
        }
    }
}

#[test]
fn interconnection_option_aliases_are_accepted() {
    let json = r#"{
      "id":"root",
      "layoutOptions":{
        "elk.algorithm":"org.eclipse.elk.layered",
        "org.eclipse.elk.direction":"RIGHT",
        "org.eclipse.elk.edgeRouting":"ORTHOGONAL",
        "org.eclipse.elk.layered.routingBackend":"libavoid",
        "org.eclipse.elk.spacing.nodeNode":32
      },
      "children":[
        {"id":"a","width":100,"height":60},
        {"id":"b","width":100,"height":60},
        {"id":"c","width":100,"height":60}
      ],
      "edges":[
        {"id":"e1","sources":["a"],"targets":["b"],"layoutOptions":{"org.eclipse.elk.edge.bundle":"alpha"}},
        {"id":"e2","sources":["a"],"targets":["b"],"layoutOptions":{"org.eclipse.elk.edge.bundle":"beta"}},
        {"id":"e3","sources":["b"],"targets":["c"]}
      ]
    }"#;
    let mut g = import_str(json).expect("import should succeed").graph;
    layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
    assert!(
        g.edges.iter().all(|e| !e.sections.is_empty()),
        "expected all edges to be routed"
    );
}

#[test]
fn dense_parallel_edges_with_symbolic_bundle_keys_are_routable() {
    let json = r#"{
      "id":"root",
      "layoutOptions":{
        "elk.algorithm":"org.eclipse.elk.layered",
        "elk.direction":"RIGHT",
        "elk.edgeRouting":"ORTHOGONAL"
      },
      "children":[
        {"id":"left","width":120,"height":80},
        {"id":"right","width":120,"height":80}
      ],
      "edges":[
        {"id":"e1","sources":["left"],"targets":["right"],"layoutOptions":{"org.eclipse.elk.layered.edgeBundle":"alpha"}},
        {"id":"e2","sources":["left"],"targets":["right"],"layoutOptions":{"org.eclipse.elk.layered.edgeBundle":"beta"}}
      ]
    }"#;
    let mut g = import_str(json).expect("import should succeed").graph;
    layout(&mut g, &LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView))
        .expect("layout should succeed");
    assert_eq!(g.edges.len(), 2);
    for edge in &g.edges {
        assert!(!edge.sections.is_empty(), "edge should be routed");
    }
    assert!(
        g.edges.iter().all(|e| !e.sections.is_empty()),
        "bundle-keyed dense parallels should remain routable"
    );
}

#[test]
fn interconnection_dense_route_signature_is_stable() {
    let json = read_fixture("interconnection_real_dense.json");
    let mut g1 = import_str(&json).expect("import should succeed").graph;
    let mut g2 = import_str(&json).expect("import should succeed").graph;
    let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
    layout(&mut g1, &options).expect("first layout should succeed");
    layout(&mut g2, &options).expect("second layout should succeed");

    let sig = |g: &elk_graph::ElkGraph| -> Vec<(usize, usize)> {
        g.edges
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let bends = e
                    .sections
                    .iter()
                    .map(|sid| g.edge_sections[sid.index()].bend_points.len())
                    .sum::<usize>();
                (i, bends)
            })
            .collect()
    };
    assert_eq!(sig(&g1), sig(&g2), "dense interconnection route signature drifted");
}

#[test]
fn direct_libavoid_alias_options_are_accepted() {
    let json = r#"{
      "id":"root",
      "layoutOptions":{
        "elk.algorithm":"org.eclipse.elk.libavoid",
        "org.eclipse.elk.libavoid.clearance":6,
        "org.eclipse.elk.libavoid.segmentPenalty":1.5,
        "org.eclipse.elk.libavoid.bendPenalty":9.0
      },
      "children":[
        {"id":"a","x":0,"y":0,"width":60,"height":40},
        {"id":"b","x":240,"y":0,"width":60,"height":40},
        {"id":"block","x":90,"y":10,"width":100,"height":80}
      ],
      "edges":[
        {"id":"e1","sources":["a"],"targets":["b"],"layoutOptions":{"org.eclipse.elk.libavoid.bendPenalty":3.0}}
      ]
    }"#;
    let mut g = import_str(json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("libavoid layout should succeed");
    assert_eq!(g.edges.len(), 1);
    assert!(!g.edges[0].sections.is_empty());
    let sec = &g.edge_sections[g.edges[0].sections[0].index()];
    assert!(sec.start.x.is_finite() && sec.end.x.is_finite());
}

#[test]
fn interconnection_full_drone_like_route_signature_is_stable() {
    let json = read_fixture("interconnection_real_full_drone_like.json");
    let mut g1 = import_str(&json).expect("import should succeed").graph;
    let mut g2 = import_str(&json).expect("import should succeed").graph;
    let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
    layout(&mut g1, &options).expect("first layout should succeed");
    layout(&mut g2, &options).expect("second layout should succeed");

    assert_eq!(
        route_signature(&g1),
        route_signature(&g2),
        "drone-like interconnection route signature drifted"
    );
}
