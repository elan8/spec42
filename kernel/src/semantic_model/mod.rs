//! Semantic graph model for SysML v2 documents.
//!
//! Compatibility facade over `crate::semantic` during migration.
pub use crate::semantic::{
    add_cross_document_edges_for_uri, build_graph_from_doc, evaluate_expressions,
    hover_markdown_for_node,
    resolve_cross_document_edges_for_uri, resolve_expression_endpoint_strict,
    resolve_imported_node_ids_for_simple_name, resolve_member_via_type,
    resolve_type_reference_targets, NodeId, RelationshipKind, ResolveResult, SemanticGraph,
    SemanticNode, symbol_entries_for_uri,
};
