pub(crate) mod evaluation;
pub(crate) mod graph;
pub(crate) mod import_resolution;
pub(crate) mod model;
pub(crate) mod reference_resolution;
pub(crate) mod relationships;
pub(crate) mod resolution;
pub(crate) mod workspace_uri;

pub use evaluation::evaluate_expressions;
pub use graph::SemanticGraph;
pub use import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use language_service::{hover_markdown_for_node, signature_from_node};
pub use model::{NodeId, RelationshipKind, SemanticNode};
pub use reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
pub use relationships::{
    add_cross_document_edges_for_uri, link_workspace_derivations, link_workspace_relationships,
    resolve_cross_document_edges_for_uri, resolve_workspace_pending_relationships,
};
pub use sysml_model::declared_packages_in_content;
pub use sysml_model::semantic::graph_builder::build_graph_from_doc;
pub use sysml_model::{finalize_and_evaluate, patch_graph_for_document};
pub use workspace_uri::uri_under_any_library;
