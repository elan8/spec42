#![forbid(unsafe_code)]
#![doc = "Core graph model, geometry, options, and layout traits for elk-rust."]

mod error;
mod core_options;
mod geometry;
mod graph;
mod ids;
mod layout;
mod options;

pub use error::LayoutError;
pub use core_options::{
    CoreOptionPipeline, CoreOptionPreflight, CoreOptionScope, CorePropertyValue, CoreValidationIssue,
    CoreValidationIssueKind,
};
pub use geometry::{Point, Rect, Size};
#[allow(deprecated)]
pub use graph::{Edge, EdgeEndpoint, EdgeSection, Graph, GraphStats, Label, Node, Port, PortSide};
pub use ids::{EdgeId, GraphId, LabelId, NodeId, PortId};
pub use layout::{LayoutEngine, LayoutPhaseStat, LayoutReport, LayoutStats};
pub use options::{
    ContentAlignment, EdgeLabelPlacement, EdgeRouting, ElementLayoutOptions, HierarchyHandling,
    LayerConstraint, LayeredOptions, LayoutDirection, LayoutOptions, NodeAlignment,
    NodeLabelPlacement, Padding, PortConstraint, PortLabelPlacement, Spacing, ViewProfile,
};
