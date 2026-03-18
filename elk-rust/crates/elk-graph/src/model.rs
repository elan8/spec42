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
}

