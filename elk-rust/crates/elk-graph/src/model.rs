use elk_core::{Point, PortSide, Rect, Size};

use crate::ids::{EdgeId, EdgeSectionId, LabelId, NodeId, PortId};
use crate::properties::PropertyBag;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for ShapeGeometry {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl ShapeGeometry {
    #[must_use]
    pub fn rect(self) -> Rect {
        Rect::new(Point::new(self.x, self.y), Size::new(self.width, self.height))
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    pub id: LabelId,
    pub text: String,
    pub geometry: ShapeGeometry,
    pub properties: PropertyBag,
}

#[derive(Clone, Debug)]
pub struct Port {
    pub id: PortId,
    pub node: NodeId,
    pub side: PortSide,
    pub geometry: ShapeGeometry,
    pub labels: Vec<LabelId>,
    pub properties: PropertyBag,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct EdgeSection {
    pub id: EdgeSectionId,
    pub start: Point,
    pub bend_points: Vec<Point>,
    pub end: Point,
    pub properties: PropertyBag,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub sources: Vec<EdgeEndpoint>,
    pub targets: Vec<EdgeEndpoint>,
    pub sections: Vec<EdgeSectionId>,
    pub labels: Vec<LabelId>,
    pub properties: PropertyBag,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub ports: Vec<PortId>,
    /// Edges contained at this node (ELK JSON allows edges at any level).
    pub edges: Vec<EdgeId>,
    pub labels: Vec<LabelId>,
    pub geometry: ShapeGeometry,
    pub properties: PropertyBag,
}

#[derive(Clone, Debug)]
pub struct ElkGraph {
    pub nodes: Vec<Node>,
    pub ports: Vec<Port>,
    pub edges: Vec<Edge>,
    pub edge_sections: Vec<EdgeSection>,
    pub labels: Vec<Label>,
    pub root: NodeId,
    pub properties: PropertyBag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElkGraphValidationError {
    OutOfBoundsId { kind: &'static str, index: usize, len: usize },
    InvalidRoot { index: usize, len: usize },
    ParentMismatch { child: NodeId, parent: NodeId },
    MissingChildLink { parent: NodeId, child: NodeId },
    PortNodeMismatch { port: PortId, port_node: NodeId, listed_in: NodeId },
    MissingPortLink { node: NodeId, port: PortId },
    MissingEdgeLink { node: NodeId, edge: EdgeId },
    InvalidEdgeEndpointNode { edge: EdgeId, node: NodeId },
    InvalidEdgeEndpointPort { edge: EdgeId, port: PortId },
    EdgeSectionMissing { edge: EdgeId, section: EdgeSectionId },
}

impl ElkGraph {
    #[must_use]
    pub fn new() -> Self {
        let mut g = Self {
            nodes: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            edge_sections: Vec::new(),
            labels: Vec::new(),
            root: NodeId(0),
            properties: PropertyBag::default(),
        };
        let root = g.add_node_internal(None, ShapeGeometry::default());
        g.root = root;
        g
    }

    #[must_use]
    pub fn add_node(&mut self, parent: NodeId, geometry: ShapeGeometry) -> NodeId {
        self.add_node_internal(Some(parent), geometry)
    }

    fn add_node_internal(&mut self, parent: Option<NodeId>, geometry: ShapeGeometry) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node {
            id,
            parent,
            children: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            labels: Vec::new(),
            geometry,
            properties: PropertyBag::default(),
        });
        if let Some(p) = parent {
            self.nodes[p.index()].children.push(id);
        }
        id
    }

    #[must_use]
    pub fn add_port(&mut self, node: NodeId, side: PortSide, geometry: ShapeGeometry) -> PortId {
        let id = PortId(self.ports.len());
        self.ports.push(Port {
            id,
            node,
            side,
            geometry,
            labels: Vec::new(),
            properties: PropertyBag::default(),
        });
        self.nodes[node.index()].ports.push(id);
        id
    }

    #[must_use]
    pub fn add_label(&mut self, text: impl Into<String>, geometry: ShapeGeometry) -> LabelId {
        let id = LabelId(self.labels.len());
        self.labels.push(Label {
            id,
            text: text.into(),
            geometry,
            properties: PropertyBag::default(),
        });
        id
    }

    pub fn attach_label_to_node(&mut self, node: NodeId, label: LabelId) {
        self.nodes[node.index()].labels.push(label);
    }

    pub fn attach_label_to_port(&mut self, port: PortId, label: LabelId) {
        self.ports[port.index()].labels.push(label);
    }

    pub fn attach_label_to_edge(&mut self, edge: EdgeId, label: LabelId) {
        self.edges[edge.index()].labels.push(label);
    }

    #[must_use]
    pub fn add_edge(
        &mut self,
        container: NodeId,
        sources: Vec<EdgeEndpoint>,
        targets: Vec<EdgeEndpoint>,
    ) -> EdgeId {
        let id = EdgeId(self.edges.len());
        self.edges.push(Edge {
            id,
            sources,
            targets,
            sections: Vec::new(),
            labels: Vec::new(),
            properties: PropertyBag::default(),
        });
        self.nodes[container.index()].edges.push(id);
        id
    }

    #[must_use]
    pub fn add_edge_section(
        &mut self,
        edge: EdgeId,
        start: Point,
        bend_points: Vec<Point>,
        end: Point,
    ) -> EdgeSectionId {
        let id = EdgeSectionId(self.edge_sections.len());
        self.edge_sections.push(EdgeSection {
            id,
            start,
            bend_points,
            end,
            properties: PropertyBag::default(),
        });
        self.edges[edge.index()].sections.push(id);
        id
    }

    #[must_use]
    pub fn children_of(&self, node: NodeId) -> &[NodeId] {
        &self.nodes[node.index()].children
    }

    #[must_use]
    pub fn ports_of(&self, node: NodeId) -> &[PortId] {
        &self.nodes[node.index()].ports
    }

    #[must_use]
    pub fn edges_in(&self, node: NodeId) -> &[EdgeId] {
        &self.nodes[node.index()].edges
    }

    #[must_use]
    pub fn node_labels(&self, node: NodeId) -> &[LabelId] {
        &self.nodes[node.index()].labels
    }

    #[must_use]
    pub fn port_labels(&self, port: PortId) -> &[LabelId] {
        &self.ports[port.index()].labels
    }

    #[must_use]
    pub fn edge_labels(&self, edge: EdgeId) -> &[LabelId] {
        &self.edges[edge.index()].labels
    }

    #[must_use]
    pub fn is_ancestor(&self, ancestor: NodeId, node: NodeId) -> bool {
        if ancestor == node {
            return true;
        }
        let mut cur = Some(node);
        while let Some(n) = cur {
            if n == ancestor {
                return true;
            }
            cur = self.nodes[n.index()].parent;
        }
        false
    }

    #[must_use]
    pub fn nearest_common_ancestor(&self, a: NodeId, b: NodeId) -> Option<NodeId> {
        let mut seen = std::collections::BTreeSet::new();
        let mut cur = Some(a);
        while let Some(n) = cur {
            seen.insert(n);
            cur = self.nodes[n.index()].parent;
        }
        let mut cur = Some(b);
        while let Some(n) = cur {
            if seen.contains(&n) {
                return Some(n);
            }
            cur = self.nodes[n.index()].parent;
        }
        None
    }

    /// Move `node` under `new_parent`, keeping hierarchy invariants.
    pub fn move_node(&mut self, node: NodeId, new_parent: NodeId) {
        let old_parent = self.nodes[node.index()].parent;
        if old_parent == Some(new_parent) {
            return;
        }
        if let Some(p) = old_parent {
            self.nodes[p.index()].children.retain(|&c| c != node);
        }
        self.nodes[new_parent.index()].children.push(node);
        self.nodes[node.index()].parent = Some(new_parent);
    }

    /// Change which node contains an edge (updates `Node.edges` lists).
    pub fn set_edge_container(&mut self, edge: EdgeId, new_container: NodeId) {
        // Remove from any existing container lists.
        for n in &mut self.nodes {
            n.edges.retain(|&e| e != edge);
        }
        self.nodes[new_container.index()].edges.push(edge);
    }

    pub fn validate(&self) -> Result<(), ElkGraphValidationError> {
        let n_len = self.nodes.len();
        if self.root.index() >= n_len {
            return Err(ElkGraphValidationError::InvalidRoot {
                index: self.root.index(),
                len: n_len,
            });
        }

        // Nodes: parent/children consistency + port/edge/label ids in-bounds.
        for node in &self.nodes {
            let node_id = node.id;
            if node_id.index() >= n_len {
                return Err(ElkGraphValidationError::OutOfBoundsId {
                    kind: "NodeId",
                    index: node_id.index(),
                    len: n_len,
                });
            }
            if let Some(p) = node.parent {
                if p.index() >= n_len {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "NodeId(parent)",
                        index: p.index(),
                        len: n_len,
                    });
                }
                if !self.nodes[p.index()].children.contains(&node_id) {
                    return Err(ElkGraphValidationError::MissingChildLink {
                        parent: p,
                        child: node_id,
                    });
                }
            }
            for &c in &node.children {
                if c.index() >= n_len {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "NodeId(child)",
                        index: c.index(),
                        len: n_len,
                    });
                }
                if self.nodes[c.index()].parent != Some(node_id) {
                    return Err(ElkGraphValidationError::ParentMismatch {
                        child: c,
                        parent: node_id,
                    });
                }
            }

            for &pid in &node.ports {
                if pid.index() >= self.ports.len() {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "PortId",
                        index: pid.index(),
                        len: self.ports.len(),
                    });
                }
                let port = &self.ports[pid.index()];
                if port.node != node_id {
                    return Err(ElkGraphValidationError::PortNodeMismatch {
                        port: pid,
                        port_node: port.node,
                        listed_in: node_id,
                    });
                }
            }

            for &eid in &node.edges {
                if eid.index() >= self.edges.len() {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "EdgeId",
                        index: eid.index(),
                        len: self.edges.len(),
                    });
                }
            }

            for &lid in &node.labels {
                if lid.index() >= self.labels.len() {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "LabelId",
                        index: lid.index(),
                        len: self.labels.len(),
                    });
                }
            }
        }

        // Ports: labels in-bounds and node exists.
        for port in &self.ports {
            if port.node.index() >= n_len {
                return Err(ElkGraphValidationError::OutOfBoundsId {
                    kind: "NodeId(port.node)",
                    index: port.node.index(),
                    len: n_len,
                });
            }
            if !self.nodes[port.node.index()].ports.contains(&port.id) {
                return Err(ElkGraphValidationError::MissingPortLink {
                    node: port.node,
                    port: port.id,
                });
            }
            for &lid in &port.labels {
                if lid.index() >= self.labels.len() {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "LabelId(port.labels)",
                        index: lid.index(),
                        len: self.labels.len(),
                    });
                }
            }
        }

        // Edges: endpoints ids + section refs.
        for edge in &self.edges {
            for ep in edge.sources.iter().chain(edge.targets.iter()) {
                if ep.node.index() >= n_len {
                    return Err(ElkGraphValidationError::InvalidEdgeEndpointNode {
                        edge: edge.id,
                        node: ep.node,
                    });
                }
                if let Some(pid) = ep.port {
                    if pid.index() >= self.ports.len() {
                        return Err(ElkGraphValidationError::InvalidEdgeEndpointPort {
                            edge: edge.id,
                            port: pid,
                        });
                    }
                }
            }
            for &sid in &edge.sections {
                if sid.index() >= self.edge_sections.len() {
                    return Err(ElkGraphValidationError::EdgeSectionMissing {
                        edge: edge.id,
                        section: sid,
                    });
                }
            }
            for &lid in &edge.labels {
                if lid.index() >= self.labels.len() {
                    return Err(ElkGraphValidationError::OutOfBoundsId {
                        kind: "LabelId(edge.labels)",
                        index: lid.index(),
                        len: self.labels.len(),
                    });
                }
            }
        }

        // Labels: in-bounds ids already covered via containers; keep as-is.
        Ok(())
    }
}

impl Default for ElkGraph {
    fn default() -> Self {
        Self::new()
    }
}

