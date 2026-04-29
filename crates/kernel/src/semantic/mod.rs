pub(crate) mod ast_util;
pub(crate) mod evaluation;
pub(crate) mod graph;
pub(crate) mod graph_builder;
pub(crate) mod import_resolution;
pub(crate) mod model;
pub mod presentation;
pub(crate) mod reference_resolution;
pub(crate) mod relationships;
pub(crate) mod resolution;
pub(crate) mod root_element;
pub(crate) mod workspace_uri;

pub use evaluation::evaluate_expressions;
pub use graph::SemanticGraph;
pub use graph_builder::build_graph_from_doc;
pub use import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use model::{NodeId, RelationshipKind, SemanticNode};
pub use presentation::hover_markdown_for_node;
pub use presentation::symbol_entries_for_uri;
pub use reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
pub use relationships::{add_cross_document_edges_for_uri, resolve_cross_document_edges_for_uri};
pub use workspace_uri::uri_under_any_library;
