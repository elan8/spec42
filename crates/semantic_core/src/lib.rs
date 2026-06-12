pub mod semantic;

pub use semantic::activity_graph::enrich_activity_diagrams_from_graph;
pub use semantic::diagnostics::{
    collect_diagnostics_from_graph, collect_untyped_part_usage_diagnostics,
    missing_library_context_diagnostic, DiagnosticRelatedInfo, DiagnosticSeverity,
    DiagnosticsOptions, SemanticDiagnostic,
};
pub use semantic::dto::{
    range_to_dto, visualization_model_not_ready, GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto,
    RelationshipDto, SysmlElementDto, SysmlGraphDto, SysmlModelStatsDto,
    SysmlVisualizationGroupDto, SysmlVisualizationPackageCandidateDto, SysmlVisualizationResultDto,
    SysmlVisualizationViewCandidateDto, WorkspaceFileModelDto, WorkspaceModelDto,
    WorkspaceModelSummaryDto,
};
pub use semantic::evaluation::{
    evaluate_expressions, evaluate_expressions_with_unit_catalogs, UnitRegistry,
};
pub use semantic::explicit_views::{
    build_view_candidates, build_view_catalog, evaluate_views, project_ids_for_renderer,
    renderer_view_for_view_type, EvaluatedView, ExposeSpec, FilterExpr, ViewCatalog,
    ViewDefinitionSpec, ViewUsageSpec,
};
pub use semantic::extracted_model::extract_activity_diagrams;
pub use semantic::graph::{PendingExpressionRelationship, PendingRelationship, SemanticGraph};
pub use semantic::graph_builder::build_graph_from_doc;
pub use semantic::ibd::{
    build_ibd_for_uri, finalize_merged_ibd_connectors, merge_ibd_payloads, IbdDataDto,
};
pub use semantic::interconnection_elk::build_elk_graph_from_scene;
pub use semantic::interconnection_scene::{
    build_interconnection_scene, InterconnectionSceneDto,
};
pub use semantic::import_resolution::{
    resolve_imported_node_ids_for_simple_name, resolve_type_reference_targets,
};
pub use semantic::library_loader::{
    resolve_library_closure, LibraryClosureOptions, LoadedLibraryFile, WorkspaceSource,
};
pub use semantic::model::{
    ConnectStatementDetail, NodeId, RelationshipKind, SemanticEdge, SemanticNode,
};
pub use semantic::reference_resolution::{
    parse_expose_target_suffix, resolve_expose_target, resolve_expression_endpoint_strict,
    resolve_expression_endpoint_workspace, resolve_inherited_member_via_type,
    resolve_member_via_type, ExposeExpandMode, ExposeTargetResolution, ResolveResult,
};
pub use semantic::relationships::{
    add_cross_document_edges_for_uri, link_workspace_relationships,
    resolve_cross_document_edges_for_uri, resolve_workspace_pending_relationships,
    TYPE_REFERENCE_ATTR_KEYS,
};
pub use semantic::relationships::{add_semantic_edge_once, AddSemanticEdgeResult};
pub use semantic::root_element::root_element_body;
pub use semantic::source::providers::filesystem::FileSystemDocumentProvider;
pub use semantic::source::{
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
pub use semantic::state_views::build_workspace_state_machines;
pub use semantic::text_span::{TextPosition, TextRange};
pub use semantic::visualization_entry::build_sysml_visualization_from_graph;
pub use semantic::visualization_workspace::{
    attach_ibd_package_container_groups, build_ibd_package_container_groups,
    build_merged_workspace_ibd, build_package_groups_from_graph,
    build_sysml_visualization_from_artifacts, build_sysml_visualization_from_graph_and_documents,
    build_sysml_visualization_workspace, build_sysml_visualization_workspace_with_meta,
    build_workspace_activity_diagrams, build_workspace_graph_dto_for_uris,
    build_workspace_visualization_artifacts, filter_ibd_by_visible_ids,
    select_interconnection_ibd_scope, select_interconnection_ibd_scope_with_trace, uri_under_root,
    workspace_uris_for_root, IbdScopeTrace, VisualizationBuildMeta, VisualizationBuildOptions,
    WorkspaceVisualizationArtifacts,
};
pub use semantic::workspace_graph::{
    build_semantic_graph_from_documents, build_semantic_graph_with_provider,
    WorkspaceParsedDocument,
};
