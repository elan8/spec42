//! Host semantic model projection (no LSP transport types).

use serde::Serialize;
use semantic_core::TextRange;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HostSemanticModelNode {
    pub uri: String,
    pub qualified_name: String,
    pub name: String,
    pub element_kind: String,
    pub range: TextRange,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HostSemanticModelRelationship {
    pub source: String,
    pub target: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HostSemanticProjection {
    pub nodes: Vec<HostSemanticModelNode>,
    pub relationships: Vec<HostSemanticModelRelationship>,
}
