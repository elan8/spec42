//! Semantic graph model for SysML v2 documents.
//!
//! Core graph types and builders live in the `semantic-model` workspace crate. This module
//! re-exports that API and provides LSP-facing adapters (`hover`, `symbol_entries`).

mod hover;
mod symbol_entries;

pub use hover::hover_markdown_for_node;
pub(crate) use hover::signature_from_node;
pub use semantic_model_crate::{
    add_cross_document_edges_for_uri, build_graph_from_doc, evaluate_expressions,
    resolve_cross_document_edges_for_uri, resolve_expression_endpoint_strict,
    resolve_imported_node_ids_for_simple_name, resolve_member_via_type,
    resolve_type_reference_targets, NodeId, RelationshipKind, ResolveResult, SemanticGraph,
    SemanticNode,
};
pub use symbol_entries::symbol_entries_for_uri;
