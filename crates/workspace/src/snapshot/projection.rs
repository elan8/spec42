//! Host semantic model projection (no LSP transport types).
//!
//! This is a 1:1 representation of the semantic graph in a serializable form.
//! Every field present in [`SemanticNode`] and [`SemanticEdge`] is preserved here;
//! nothing is dropped or summarised. Consumers such as babel42 receive the full
//! semantic model and can filter or present it as needed.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysml_model::{ConnectStatementDetail, ElementKind, RelationshipKind, TextRange};

/// A node in the semantic model — maps 1:1 to [`sysml_model::SemanticNode`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostSemanticModelNode {
    /// Document URI the node was declared in.
    pub uri: String,
    /// Fully-qualified name (unique workspace-wide, may include `#kind` disambiguator).
    pub qualified_name: String,
    /// Simple (unqualified) name as written in the source.
    pub name: String,
    /// SysML element kind (typed enum, serialises as its canonical lowercase string).
    pub element_kind: ElementKind,
    /// Source range of the element declaration.
    pub range: TextRange,
    /// Qualified name of the parent element, if any.
    pub parent: Option<String>,
    /// Element-specific attributes extracted during graph construction.
    /// Keys and value shapes are kind-dependent (e.g. `"typeRef"`, `"multiplicity"`,
    /// `"redefines"`, `"evaluatedValue"`, …).
    #[serde(default)]
    pub attributes: HashMap<String, Value>,
}

/// A directed relationship between two nodes — maps 1:1 to [`sysml_model::SemanticEdge`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostSemanticModelRelationship {
    /// Qualified name of the source node.
    pub source: String,
    /// Qualified name of the target node.
    pub target: String,
    /// Relationship kind (typed enum, serialises as its canonical lowercase string).
    pub kind: RelationshipKind,
    /// Present when this `Connection` edge was resolved from an explicit `connect` statement.
    #[serde(default)]
    pub connect: Option<ConnectStatementDetail>,
}

/// The complete semantic projection of a workspace — all nodes and relationships
/// from workspace (non-library) documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostSemanticProjection {
    pub nodes: Vec<HostSemanticModelNode>,
    pub relationships: Vec<HostSemanticModelRelationship>,
}
