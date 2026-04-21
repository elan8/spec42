//! Semantic graph model for SysML v2 documents.
//!
//! Builds a petgraph-based graph from parsed ASTs. Nodes represent model elements
//! (packages, parts, ports, etc.); edges represent SysML relationships
//! (typing, specializes, connection, bind, allocate, transition).

mod ast_util;
mod evaluation;
mod graph;
mod graph_builder;
mod import_resolution;
mod model;
mod reference_resolution;
mod relationships;
mod root_element;
mod workspace_uri;

pub use graph::SemanticGraph;
pub use graph_builder::build_graph_from_doc;
pub use evaluation::evaluate_expressions;
pub use import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use model::{NodeId, RelationshipKind, SemanticNode};
pub use reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
pub use relationships::{add_cross_document_edges_for_uri, resolve_cross_document_edges_for_uri};
pub use workspace_uri::uri_under_any_library;

pub(crate) use root_element::root_element_body;
