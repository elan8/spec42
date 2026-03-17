use std::time::Duration;

use crate::{Graph, LayoutError, LayoutOptions};

pub trait LayoutEngine {
    fn layout(
        &self,
        graph: &mut Graph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError>;
}

#[derive(Clone, Debug, Default)]
pub struct LayoutReport {
    pub warnings: Vec<String>,
    pub stats: LayoutStats,
}

#[derive(Clone, Debug, Default)]
pub struct LayoutStats {
    pub reversed_edges: usize,
    pub layers: usize,
    pub normalized_edges: usize,
    pub dummy_nodes: usize,
    pub crossings_before: usize,
    pub crossings_after: usize,
    pub crossing_sweeps: Vec<usize>,
    pub straight_segments: usize,
    pub routed_edge_segments: usize,
    pub aligned_nodes: usize,
    pub compacted_layers: usize,
    pub component_count: usize,
    pub packed_components: usize,
    pub bend_points: usize,
    pub label_displacements: f32,
    pub phases: Vec<LayoutPhaseStat>,
}

#[derive(Clone, Debug)]
pub struct LayoutPhaseStat {
    pub name: &'static str,
    pub duration: Duration,
}
