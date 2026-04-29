pub(crate) mod checks;
pub(crate) mod diagnostics_core;
pub(crate) mod helpers;

pub use crate::language::SymbolEntry;
pub use crate::semantic::{
    NodeId, RelationshipKind, SemanticGraph, SemanticNode, add_cross_document_edges_for_uri,
    build_graph_from_doc,
};
pub use crate::semantic_tokens::{
    ast_semantic_ranges, legend, semantic_tokens_full, semantic_tokens_range,
};
pub use checks::{DefaultSemanticChecks, compute_semantic_diagnostics};
