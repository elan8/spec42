use std::collections::BTreeSet;

use crate::{
    ContentAlignment, EdgeId, EdgeLabelPlacement, ElementLayoutOptions, GraphId, LabelId, NodeId,
    NodeLabelPlacement, Point, PortConstraint, PortId, PortLabelPlacement, Rect, Size,
};

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug)]
pub struct Graph {
    pub id: GraphId,
    pub nodes: Vec<Node>,
    pub ports: Vec<Port>,
    pub edges: Vec<Edge>,
    pub labels: Vec<Label>,
    pub bounds: Rect,
    pub layout: ElementLayoutOptions,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: GraphId(0),
            nodes: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            labels: Vec::new(),
            bounds: Rect::default(),
            layout: ElementLayoutOptions::default(),
        }
    }

    #[must_use]
    pub fn add_node(&mut self, size: Size) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node::new(id, size));
        id
    }

    pub fn add_child_node(&mut self, parent: NodeId, size: Size) -> NodeId {
        let child = self.add_node(size);
        self.node_mut(child).parent = Some(parent);
        self.node_mut(parent).children.push(child);
        child
    }

    pub fn add_port(&mut self, node: NodeId, side: PortSide, size: Size) -> PortId {
        let id = PortId(self.ports.len());
        self.ports.push(Port::new(id, node, side, size));
        self.node_mut(node).ports.push(id);
        id
    }

    pub fn add_edge(&mut self, source: EdgeEndpoint, target: EdgeEndpoint) -> EdgeId {
        let id = EdgeId(self.edges.len());
        self.edges.push(Edge::new(id, source, target));
        id
    }

    pub fn add_label(&mut self, text: impl Into<String>, size: Size) -> LabelId {
        let id = LabelId(self.labels.len());
        self.labels.push(Label::new(id, text.into(), size));
        id
    }

    pub fn add_node_label(&mut self, node: NodeId, text: impl Into<String>, size: Size) -> LabelId {
        let label = self.add_label(text, size);
        self.labels[label.index()].layout.node_label_placement =
            Some(NodeLabelPlacement::OutsideTopCenter);
        self.node_mut(node).labels.push(label);
        label
    }

    pub fn add_port_label(&mut self, port: PortId, text: impl Into<String>, size: Size) -> LabelId {
        let label = self.add_label(text, size);
        self.labels[label.index()].layout.port_label_placement = Some(PortLabelPlacement::Outside);
        self.port_mut(port).labels.push(label);
        label
    }

    pub fn add_edge_label(&mut self, edge: EdgeId, text: impl Into<String>, size: Size) -> LabelId {
        let label = self.add_label(text, size);
        self.labels[label.index()].layout.edge_label_placement = Some(EdgeLabelPlacement::Center);
        self.edge_mut(edge).labels.push(label);
        label
    }

    #[must_use]
    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id.index()]
    }

    #[must_use]
    pub fn node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.index()]
    }

    #[must_use]
    pub fn port(&self, id: PortId) -> &Port {
        &self.ports[id.index()]
    }

    #[must_use]
    pub fn port_mut(&mut self, id: PortId) -> &mut Port {
        &mut self.ports[id.index()]
    }

    #[must_use]
    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id.index()]
    }

    #[must_use]
    pub fn edge_mut(&mut self, id: EdgeId) -> &mut Edge {
        &mut self.edges[id.index()]
    }

    #[must_use]
    pub fn top_level_nodes(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .filter(|node| node.parent.is_none())
            .map(|node| node.id)
            .collect()
    }

    #[must_use]
    pub fn children_of(&self, parent: NodeId) -> &[NodeId] {
        &self.node(parent).children
    }

    #[must_use]
    pub fn nearest_ancestor_in_set(
        &self,
        node: NodeId,
        local_nodes: &BTreeSet<NodeId>,
    ) -> Option<NodeId> {
        let mut current = Some(node);
        while let Some(candidate) = current {
            if local_nodes.contains(&candidate) {
                return Some(candidate);
            }
            current = self.node(candidate).parent;
        }
        None
    }

    #[must_use]
    pub fn stats(&self) -> GraphStats {
        GraphStats {
            node_count: self.nodes.len(),
            port_count: self.ports.len(),
            edge_count: self.edges.len(),
            label_count: self.labels.len(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphStats {
    pub node_count: usize,
    pub port_count: usize,
    pub edge_count: usize,
    pub label_count: usize,
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub ports: Vec<PortId>,
    pub labels: Vec<LabelId>,
    pub bounds: Rect,
    pub layout: ElementLayoutOptions,
    pub preferred_position: Option<Point>,
}

impl Node {
    #[must_use]
    pub fn new(id: NodeId, size: Size) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            ports: Vec::new(),
            labels: Vec::new(),
            bounds: Rect::new(Point::default(), size),
            layout: ElementLayoutOptions {
                content_alignment: Some(ContentAlignment::Center),
                ..ElementLayoutOptions::default()
            },
            preferred_position: None,
        }
    }
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug)]
pub struct Port {
    pub id: PortId,
    pub node: NodeId,
    pub side: PortSide,
    pub bounds: Rect,
    pub labels: Vec<LabelId>,
    pub layout: ElementLayoutOptions,
    pub is_hierarchical: bool,
}

impl Port {
    #[must_use]
    pub fn new(id: PortId, node: NodeId, side: PortSide, size: Size) -> Self {
        Self {
            id,
            node,
            side,
            bounds: Rect::new(Point::default(), size),
            labels: Vec::new(),
            layout: ElementLayoutOptions {
                port_constraint: Some(PortConstraint::FixedSide),
                ..ElementLayoutOptions::default()
            },
            is_hierarchical: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortSide {
    North,
    South,
    East,
    West,
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub source: EdgeEndpoint,
    pub target: EdgeEndpoint,
    pub labels: Vec<LabelId>,
    pub sections: Vec<EdgeSection>,
    pub was_reversed: bool,
    pub layout: ElementLayoutOptions,
}

impl Edge {
    #[must_use]
    pub fn new(id: EdgeId, source: EdgeEndpoint, target: EdgeEndpoint) -> Self {
        Self {
            id,
            source,
            target,
            labels: Vec::new(),
            sections: Vec::new(),
            was_reversed: false,
            layout: ElementLayoutOptions::default(),
        }
    }
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeEndpoint {
    pub node: NodeId,
    pub port: Option<PortId>,
}

impl EdgeEndpoint {
    #[must_use]
    pub const fn node(node: NodeId) -> Self {
        Self { node, port: None }
    }

    #[must_use]
    pub const fn port(node: NodeId, port: PortId) -> Self {
        Self {
            node,
            port: Some(port),
        }
    }
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeSection {
    pub start: Point,
    pub bend_points: Vec<Point>,
    pub end: Point,
}

#[deprecated(note = "Legacy graph model. New work should use the ELK-like model in the `elk-graph` crate.")]
#[derive(Clone, Debug)]
pub struct Label {
    pub id: LabelId,
    pub text: String,
    pub size: Size,
    pub position: Point,
    pub layout: ElementLayoutOptions,
}

impl Label {
    #[must_use]
    pub fn new(id: LabelId, text: String, size: Size) -> Self {
        Self {
            id,
            text,
            size,
            position: Point::default(),
            layout: ElementLayoutOptions::default(),
        }
    }
}
