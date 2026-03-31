//! Core semantic graph node identity and relationship kinds.

use std::collections::HashMap;

use tower_lsp::lsp_types::{Range, Url};

/// Unique identifier for a node in the semantic graph.
/// Combines document URI and qualified name for workspace-wide uniqueness.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct NodeId {
    pub uri: Url,
    pub qualified_name: String,
}

impl NodeId {
    pub fn new(uri: &Url, qualified_name: impl Into<String>) -> Self {
        Self {
            uri: uri.clone(),
            qualified_name: qualified_name.into(),
        }
    }
}

/// SysML v2 relationship kinds (edges in the graph).
#[allow(dead_code)] // some relationship kinds are staged for upcoming semantic features
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationshipKind {
    Typing,
    Specializes,
    Connection,
    Bind,
    Perform,
    Allocate,
    Satisfy,
    Subject,
    Transition,
    /// `then` initial state in a state composite (`transition` without `first` uses the same resolution path with [`RelationshipKind::Transition`]).
    InitialState,
}

impl RelationshipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipKind::Typing => "typing",
            RelationshipKind::Specializes => "specializes",
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
            RelationshipKind::Satisfy => "satisfy",
            RelationshipKind::Subject => "subject",
            RelationshipKind::Transition => "transition",
            RelationshipKind::InitialState => "initialState",
        }
    }
}

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: String,
    pub name: String,
    pub range: Range,
    pub attributes: HashMap<String, serde_json::Value>,
    pub parent_id: Option<NodeId>,
}
