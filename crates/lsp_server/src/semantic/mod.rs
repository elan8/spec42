pub mod presentation;

// All protocol-neutral semantic logic lives in the workspace crate.
pub use workspace::semantic::{
    add_cross_document_edges_for_uri, add_semantic_edge_once, build_graph_from_doc,
    declared_packages_in_content, evaluate_expressions, evaluate_workspace_graph,
    finalize_and_evaluate, hover_markdown_for_node, link_workspace_derivations,
    link_workspace_relationships, patch_graph_for_document, prepare_analysis_evaluation_context,
    NodeId, RelationshipKind, resolve_cross_document_edges_for_uri,
    resolve_expression_endpoint_strict, resolve_imported_node_ids_for_simple_name,
    resolve_member_via_type, resolve_type_reference_targets,
    resolve_workspace_pending_relationships, ResolveResult, SemanticGraph, SemanticNode,
    signature_from_node, uri_under_any_library,
};
pub use presentation::symbol_entries_for_uri;
