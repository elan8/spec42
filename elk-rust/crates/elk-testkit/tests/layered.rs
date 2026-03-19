use std::fs;

use elk_core::{LayoutDirection, LayoutOptions, ViewProfile};
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

fn assert_children_non_overlapping(g: &elk_graph::ElkGraph) {
    for parent in &g.nodes {
        let children = &parent.children;
        for i in 0..children.len() {
            for j in (i + 1)..children.len() {
                let a = &g.nodes[children[i].index()].geometry;
                let b = &g.nodes[children[j].index()].geometry;
                assert!(!rects_overlap(a, b), "child nodes overlap in parent {:?}", parent.id);
            }
        }
    }
}

fn assert_children_within_parent_bounds(g: &elk_graph::ElkGraph) {
    fn absolute_origin(
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
                let parent = absolute_origin(g, parent_id, cache);
                (parent.0 + local.0, parent.1 + local.1)
            }
            _ => local,
        };
        cache[node_id.index()] = Some(absolute);
        absolute
    }

    let mut cache = vec![None; g.nodes.len()];
    for parent in &g.nodes {
        if parent.id == g.root {
            continue;
        }
        let (parent_x, parent_y) = absolute_origin(g, parent.id, &mut cache);
        let px0 = parent_x;
        let py0 = parent_y;
        let px1 = px0 + parent.geometry.width;
        let py1 = py0 + parent.geometry.height;
        for child_id in &parent.children {
            let child = &g.nodes[child_id.index()];
            let (child_x, child_y) = absolute_origin(g, child.id, &mut cache);
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
    ] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        layout(&mut g, &options).expect("layout should succeed");

        assert_finite_geometry(&g);
        assert_children_non_overlapping(&g);
        assert_children_within_parent_bounds(&g);
        let max_bends = g
            .edges
            .iter()
            .flat_map(|e| e.sections.iter())
            .map(|sid| g.edge_sections[sid.index()].bend_points.len())
            .max()
            .unwrap_or(0);
        assert!(max_bends <= 8, "unexpectedly high bend count for {}", fixture);
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

