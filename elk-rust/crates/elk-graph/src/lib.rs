#![forbid(unsafe_code)]
#![doc = "ELK-like graph data model (ported from org.eclipse.elk.graph)."]

mod ids;
mod model;
mod properties;

pub use ids::{EdgeId, EdgeSectionId, LabelId, NodeId, PortId};
pub use model::{
    Edge, EdgeEndpoint, EdgeSection, ElkGraph, Label, Node, Port, ShapeGeometry,
};
pub use properties::{PropertyBag, PropertyKey, PropertyValue};

