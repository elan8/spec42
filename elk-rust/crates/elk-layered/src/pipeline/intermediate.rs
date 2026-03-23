use elk_core::LayoutOptions;
use elk_graph::ElkGraph;

use crate::pipeline::compound::{postprocess_cross_hierarchy_edges, CompoundRoutingMap};
use crate::pipeline::hierarchical_ports::run_hierarchical_port_postprocessing;
use crate::pipeline::routing::{reconcile_explicit_port_terminals, relayout_all_ports};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AfterPhaseFiveProcessor {
    HierarchicalPortOrthogonalEdgeRouter,
}

fn collect_after_phase_five_processors(map: &CompoundRoutingMap) -> Vec<AfterPhaseFiveProcessor> {
    if map.edges.is_empty() {
        Vec::new()
    } else {
        vec![AfterPhaseFiveProcessor::HierarchicalPortOrthogonalEdgeRouter]
    }
}

pub(crate) fn run_after_phase_five_processors(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
) {
    for processor in collect_after_phase_five_processors(map) {
        match processor {
            AfterPhaseFiveProcessor::HierarchicalPortOrthogonalEdgeRouter => {
                run_hierarchical_port_postprocessing(graph, map);
            }
        }
    }
}

pub(crate) fn run_post_routing_processors(
    graph: &mut ElkGraph,
    map: &CompoundRoutingMap,
    warnings: &mut Vec<String>,
) {
    run_after_phase_five_processors(graph, map);
    postprocess_cross_hierarchy_edges(graph, map, warnings);
}

pub(crate) fn run_final_geometry_processors(graph: &mut ElkGraph, options: &LayoutOptions) {
    let changed_ports = relayout_all_ports(graph, options);
    reconcile_explicit_port_terminals(graph, &changed_ports);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::compound::{CompoundRouteRecord, CompoundRoutingMap};
    use elk_graph::{EdgeEndpoint, ElkGraph, ShapeGeometry};
    use std::collections::BTreeMap;

    #[test]
    fn no_hierarchical_processors_when_map_is_empty() {
        let map = CompoundRoutingMap::default();
        assert!(collect_after_phase_five_processors(&map).is_empty());
    }

    #[test]
    fn hierarchical_processor_activates_for_cross_hierarchy_edges() {
        let mut graph = ElkGraph::new();
        let source = graph.add_node(graph.root, ShapeGeometry::default());
        let target = graph.add_node(graph.root, ShapeGeometry::default());
        let edge = graph.add_edge(
            graph.root,
            vec![EdgeEndpoint::node(source)],
            vec![EdgeEndpoint::node(target)],
        );
        let mut map = CompoundRoutingMap {
            edges: BTreeMap::new(),
            temporary_ports: Vec::new(),
        };
        map.edges.insert(
            edge,
            CompoundRouteRecord {
                original_source: EdgeEndpoint::node(source),
                original_target: EdgeEndpoint::node(target),
                routed_source: EdgeEndpoint::node(source),
                routed_target: EdgeEndpoint::node(target),
                effective_source: source,
                effective_target: target,
            },
        );

        assert_eq!(
            collect_after_phase_five_processors(&map),
            vec![AfterPhaseFiveProcessor::HierarchicalPortOrthogonalEdgeRouter]
        );
    }
}
