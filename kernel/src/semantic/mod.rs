pub mod presentation;

pub use crate::evaluation::evaluate_expressions;
pub use crate::graph::SemanticGraph;
pub use crate::graph_builder::build_graph_from_doc;
pub use crate::import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use crate::model::{NodeId, RelationshipKind, SemanticNode};
pub use crate::reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
pub use crate::relationships::{
    add_cross_document_edges_for_uri, resolve_cross_document_edges_for_uri,
};
pub use crate::workspace_uri::uri_under_any_library;
pub use presentation::hover_markdown_for_node;
pub use presentation::symbol_entries_for_uri;
