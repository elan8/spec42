use std::collections::HashSet;

use crate::layout::{
    compute_layout, evaluate, DiagramGraph, DiagramLayout, LayoutConfig, LayoutPhaseKind,
    LayoutPhaseReport, LayoutPipelineReport, Result,
};

pub(crate) fn layout(graph: &DiagramGraph, config: &LayoutConfig) -> Result<DiagramLayout> {
    layout_with_report(graph, config).map(|(layout, _)| layout)
}

pub(crate) fn layout_with_report(
    graph: &DiagramGraph,
    config: &LayoutConfig,
) -> Result<(DiagramLayout, LayoutPipelineReport)> {
    let normalized = normalize_graph(graph);
    let computed = compute_layout(&normalized, config)?;
    let mut phases = vec![LayoutPhaseReport {
        phase: LayoutPhaseKind::Normalize,
        node_count: normalized.nodes.len(),
        edge_count: normalized.edges.len(),
        notes: normalization_notes(graph, &normalized),
    }];

    phases.push(LayoutPhaseReport {
        phase: LayoutPhaseKind::Measure,
        node_count: normalized.nodes.len(),
        edge_count: normalized.edges.len(),
        notes: vec![
            format!("root direction: {:?}", config.root_layer_direction),
            format!("child direction: {:?}", config.layer_direction),
            format!("view profile: {:?}", config.view_profile),
        ],
    });
    phases[1].notes.extend(elk_phase_notes(&computed.report));
    let layout = computed.layout;

    phases.push(LayoutPhaseReport {
        phase: LayoutPhaseKind::PlaceNodes,
        node_count: layout.nodes.len(),
        edge_count: normalized.edges.len(),
        notes: vec![
            format!(
                "canvas after placement: {:.1} x {:.1}",
                layout.width, layout.height
            ),
            format!("layers: {}", computed.report.stats.layers),
            format!("components: {}", computed.report.stats.component_count),
        ],
    });

    phases.push(LayoutPhaseReport {
        phase: LayoutPhaseKind::RouteEdges,
        node_count: layout.nodes.len(),
        edge_count: layout.edges.len(),
        notes: vec![
            "elk layered orthogonal routing".to_string(),
            format!("bend points: {}", computed.report.stats.bend_points),
            format!("straight segments: {}", computed.report.stats.straight_segments),
        ],
    });

    let metrics = evaluate(&layout);
    phases.push(LayoutPhaseReport {
        phase: LayoutPhaseKind::Evaluate,
        node_count: layout.nodes.len(),
        edge_count: layout.edges.len(),
        notes: vec![
            format!("edge crossings: {}", metrics.edge_crossing_count),
            format!(
                "edge-node intrusions: {}",
                metrics.edge_node_intrusion_count
            ),
        ],
    });

    Ok((
        layout,
        LayoutPipelineReport {
            phases,
            metrics,
            warnings: computed.warnings,
        },
    ))
}

fn normalize_graph(graph: &DiagramGraph) -> DiagramGraph {
    let mut nodes = graph.nodes.clone();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));

    let known_nodes: HashSet<&str> = nodes.iter().map(|node| node.id.as_str()).collect();
    let mut seen_edges = HashSet::new();
    let mut edges = graph
        .edges
        .iter()
        .filter(|edge| {
            known_nodes.contains(edge.source_node.as_str())
                && known_nodes.contains(edge.target_node.as_str())
        })
        .filter(|edge| {
            seen_edges.insert((
                edge.id.as_str(),
                edge.source_node.as_str(),
                edge.target_node.as_str(),
                edge.kind.as_str(),
            ))
        })
        .cloned()
        .collect::<Vec<_>>();
    edges.sort_by(|left, right| {
        left.source_node
            .cmp(&right.source_node)
            .then_with(|| left.target_node.cmp(&right.target_node))
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.id.cmp(&right.id))
    });

    DiagramGraph { nodes, edges }
}

fn normalization_notes(original: &DiagramGraph, normalized: &DiagramGraph) -> Vec<String> {
    let mut notes = Vec::new();
    if original.nodes.len() != normalized.nodes.len() {
        notes.push(format!(
            "filtered {} dangling nodes",
            original.nodes.len().saturating_sub(normalized.nodes.len())
        ));
    }
    if original.edges.len() != normalized.edges.len() {
        notes.push(format!(
            "filtered or deduplicated {} edges",
            original.edges.len().saturating_sub(normalized.edges.len())
        ));
    }
    if notes.is_empty() {
        notes.push("stable sort by node and edge id".to_string());
    }
    notes
}

fn elk_phase_notes(report: &elk_core::LayoutReport) -> Vec<String> {
    report
        .stats
        .phases
        .iter()
        .map(|phase| {
            format!(
                "elk phase `{}`: {} ms",
                phase.name,
                phase.duration.as_millis()
            )
        })
        .collect()
}

