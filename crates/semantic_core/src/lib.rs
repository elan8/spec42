pub mod semantic;

pub use semantic::evaluation::evaluate_expressions;
pub use semantic::graph::SemanticGraph;
pub use semantic::import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use semantic::model::{NodeId, RelationshipKind, SemanticNode};
pub use semantic::reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
