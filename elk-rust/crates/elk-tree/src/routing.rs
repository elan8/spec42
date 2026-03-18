use elk_core::{LayoutOptions, LayoutReport, Point};
use elk_graph::{ElkGraph, EdgeEndpoint};

use crate::build_tree::TreeModel;

pub fn route_tree_edges(
    graph: &mut ElkGraph,
    tree: &TreeModel,
    options: &LayoutOptions,
    report: &mut LayoutReport,
) -> Result<(), elk_core::LayoutError> {
    let orth = options.layered.edge_routing == elk_core::EdgeRouting::Orthogonal;

    for edge_id in &tree.edges {
        let edge = &graph.edges[edge_id.index()];
        let Some(src) = edge.sources.first().copied() else { continue };
        let Some(tgt) = edge.targets.first().copied() else { continue };
        let start = node_center(graph, src.node);
        let end = node_center(graph, tgt.node);

        let mut bends = Vec::new();
        if orth && (start.x - end.x).abs() > f32::EPSILON && (start.y - end.y).abs() > f32::EPSILON {
            bends.push(Point::new(end.x, start.y));
        }
        bends = elk_alg_common::geometry::dedup_points(bends);

        let e = &mut graph.edges[edge_id.index()];
        e.sections.clear();
        let _ = graph.add_edge_section(*edge_id, start, bends, end);
    }

    report.warnings.push("elk-tree: simplified tree router active".to_string());
    Ok(())
}

fn node_center(graph: &ElkGraph, node: elk_graph::NodeId) -> Point {
    let n = &graph.nodes[node.index()];
    Point::new(n.geometry.x + n.geometry.width / 2.0, n.geometry.y + n.geometry.height / 2.0)
}

#[allow(dead_code)]
fn endpoint_center(graph: &ElkGraph, endpoint: EdgeEndpoint) -> Point {
    if let Some(port) = endpoint.port {
        let p = &graph.ports[port.index()];
        Point::new(p.geometry.x + p.geometry.width / 2.0, p.geometry.y + p.geometry.height / 2.0)
    } else {
        node_center(graph, endpoint.node)
    }
}

