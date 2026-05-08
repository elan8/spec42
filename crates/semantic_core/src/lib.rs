pub mod semantic;

pub use semantic::dto::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, RelationshipDto, SysmlGraphDto,
};
pub use semantic::evaluation::evaluate_expressions;
pub use semantic::graph::SemanticGraph;
pub use semantic::graph_builder::build_graph_from_doc;
pub use semantic::import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use semantic::model::{NodeId, RelationshipKind, SemanticNode};
pub use semantic::reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
pub use semantic::relationships::{
    add_cross_document_edges_for_uri, resolve_cross_document_edges_for_uri,
};
pub use semantic::root_element::root_element_body;
pub use semantic::workspace_graph::{build_semantic_graph_for_paths, WorkspaceParsedDocument};
