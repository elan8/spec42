//! Core semantic graph node identity and relationship kinds.

use std::collections::HashMap;

use crate::semantic::text_span::TextRange;
use url::Url;

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
    /// Control/data flow relationship inside behaviors (e.g. `flow`, `first ... then ...`).
    Flow,
    Perform,
    Allocate,
    Satisfy,
    Subject,
    Reference,
    Derivation,
    Transition,
    /// `then` initial state in a state composite (`transition` without `first` uses the same resolution path with [`RelationshipKind::Transition`]).
    InitialState,
    /// Metadata usage annotates a model element (`annotatedElement` per SysML §7.27).
    Annotation,
}

/// Optional metadata when a `Connection` edge came from a resolved `connect` statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectStatementDetail {
    pub declaring_uri: Url,
    pub range: TextRange,
    pub source_expression: String,
    pub target_expression: String,
    pub container_prefix: Option<String>,
}

/// Edge weight in the semantic graph: relationship kind plus optional connect metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEdge {
    pub kind: RelationshipKind,
    /// Set when this `Connection` came from a resolved `connect` (or pending-expression resolve).
    pub connect: Option<ConnectStatementDetail>,
}

impl SemanticEdge {
    pub fn plain(kind: RelationshipKind) -> Self {
        Self {
            kind,
            connect: None,
        }
    }

    pub fn connection_with_connect(connect: ConnectStatementDetail) -> Self {
        Self {
            kind: RelationshipKind::Connection,
            connect: Some(connect),
        }
    }
}

impl RelationshipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipKind::Typing => "typing",
            RelationshipKind::Specializes => "specializes",
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Flow => "flow",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
            RelationshipKind::Satisfy => "satisfy",
            RelationshipKind::Subject => "subject",
            RelationshipKind::Reference => "reference",
            RelationshipKind::Derivation => "derivation",
            RelationshipKind::Transition => "transition",
            RelationshipKind::InitialState => "initialState",
            RelationshipKind::Annotation => "annotation",
        }
    }

    /// Parses persisted relationship type strings (babel42 projection / Surreal).
    pub fn from_persisted_type(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "typing" => Some(RelationshipKind::Typing),
            "specializes" => Some(RelationshipKind::Specializes),
            "connection" => Some(RelationshipKind::Connection),
            "bind" => Some(RelationshipKind::Bind),
            "flow" => Some(RelationshipKind::Flow),
            "perform" => Some(RelationshipKind::Perform),
            "allocate" => Some(RelationshipKind::Allocate),
            "satisfy" => Some(RelationshipKind::Satisfy),
            "subject" => Some(RelationshipKind::Subject),
            "reference" => Some(RelationshipKind::Reference),
            "derivation" => Some(RelationshipKind::Derivation),
            "transition" | "initialstate" => Some(RelationshipKind::Transition),
            "annotation" => Some(RelationshipKind::Annotation),
            _ => None,
        }
    }
}

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: String,
    pub name: String,
    pub range: TextRange,
    pub attributes: HashMap<String, serde_json::Value>,
    pub parent_id: Option<NodeId>,
}
