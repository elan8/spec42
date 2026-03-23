use std::collections::BTreeMap;

use elk_core::{LayerConstraint, Point, PortConstraint, PortSide, Size};
use elk_graph::{EdgeEndpoint, EdgeId, LabelId, NodeId, PortId};

pub type IrNodeId = usize;

#[derive(Clone, Debug)]
pub struct LayeredIr {
    pub nodes: Vec<IrNode>,
    pub edges: Vec<IrEdge>,
    pub normalized_edges: Vec<NormalizedEdge>,
    pub layers: Vec<Vec<IrNodeId>>,
    pub real_to_ir: BTreeMap<NodeId, IrNodeId>,
}

impl LayeredIr {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            normalized_edges: Vec::new(),
            layers: Vec::new(),
            real_to_ir: BTreeMap::new(),
        }
    }

    pub fn push_node(&mut self, node: IrNode) -> IrNodeId {
        let id = self.nodes.len();
        if let IrNodeKind::Real(node_id) = node.kind {
            self.real_to_ir.insert(node_id, id);
        }
        self.nodes.push(node);
        id
    }
}

#[derive(Clone, Debug)]
pub struct IrNode {
    pub kind: IrNodeKind,
    pub size: Size,
    pub position: Point,
    pub layer: usize,
    pub order: usize,
    pub label_size: Size,
    pub ports: Vec<IrPortConstraint>,
    pub desired_minor: f32,
    pub aligned: bool,
    pub model_order: usize,
    pub layer_constraint: LayerConstraint,
}

impl IrNode {
    #[must_use]
    #[allow(dead_code)]
    pub fn center(&self) -> Point {
        Point::new(
            self.position.x + self.size.width / 2.0,
            self.position.y + self.size.height / 2.0,
        )
    }
}

#[derive(Clone, Debug)]
pub enum IrNodeKind {
    Real(NodeId),
    Dummy {
        edge_index: usize,
        segment_index: usize,
    },
    LabelPlaceholder {
        edge_index: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrPortConstraint {
    pub port_id: PortId,
    pub side: PortSide,
    pub order: usize,
    pub constraint: PortConstraint,
}

#[derive(Clone, Debug)]
pub struct IrEdge {
    pub original_edge: EdgeId,
    pub source: EdgeEndpoint,
    pub target: EdgeEndpoint,
    pub routed_source: EdgeEndpoint,
    pub routed_target: EdgeEndpoint,
    pub effective_source: NodeId,
    pub effective_target: NodeId,
    pub reversed: bool,
    pub label_ids: Vec<LabelId>,
    pub label_size: Size,
    pub chain: Vec<IrNodeId>,
    pub label_placeholder: Option<IrNodeId>,
    pub self_loop: bool,
    pub model_order: usize,
    /// Edges with the same key and shared source or target connect at the same point on nodes.
    pub bundle_key: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct NormalizedEdge {
    pub original_edge: EdgeId,
    pub edge_index: usize,
    pub from: IrNodeId,
    pub to: IrNodeId,
    pub segment_order: usize,
    pub lane: i32,
}
