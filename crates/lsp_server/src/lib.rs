//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod common;
pub mod host;
pub mod language;
mod lsp_runtime;

pub mod semantic;
pub mod semantic_tokens;
pub mod syntax;
pub mod validation;
pub mod views;
pub(crate) mod workspace;

// Host contract exports (intended stable composition surface for edition hosts).
pub use host::config::{
    CapabilityAugmenter, CapabilityMetadata, CapabilityProvider, CustomMethodProvider,
    CustomRpcContext, CustomRpcProvider, PipelineHook, Spec42Config, ValidationPipelineHook,
    KERNEL_INTERFACE_VERSION,
};
pub use host::default_config::default_config as default_server_config;
pub use lsp_runtime::run as run_lsp;

// Core data model exports.
pub use analysis::{
    add_cross_document_edges_for_uri, ast_semantic_ranges, build_graph_from_doc,
    compute_semantic_diagnostics, legend, semantic_tokens_full, semantic_tokens_range, NodeId,
    RelationshipKind, SemanticGraph, SemanticNode, SymbolEntry,
};
pub use common::util::{merge_host_and_client_library_paths, parse_library_paths_from_value};
pub use syntax::ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use validation::{
    built_workspace_input_from_snapshot, semantic_report_from_built_workspace, validate_paths,
    validate_paths_with_semantics, BuiltWorkspaceInput, SemanticValidationReport,
    ValidatedDocument, ValidationReport, ValidationRequest, ValidationSummary,
};
pub use views::dto::{
    SysmlClearCacheResultDto, SysmlFeatureInspectorElementDto, SysmlFeatureInspectorElementRefDto,
    SysmlFeatureInspectorParamsDto, SysmlFeatureInspectorRelationshipDto,
    SysmlFeatureInspectorResolutionDto, SysmlFeatureInspectorResultDto, SysmlLibrarySearchItemDto,
    SysmlLibrarySearchPackageDto, SysmlLibrarySearchParamsDto, SysmlLibrarySearchResultDto,
    SysmlLibrarySearchSourceDto, SysmlModelResultDto, SysmlServerCachesDto, SysmlServerMemoryDto,
    SysmlServerStatsDto, SysmlVisualizationParamsDto, TextDocumentIdentifierDto,
};
pub use views::{
    build_sysml_model_response, build_sysml_visualization_for_paths,
    empty_feature_inspector_response, empty_model_response, parse_sysml_feature_inspector_params,
    parse_sysml_model_params,
};

/// SysML v2 textual parser (`sysml-v2-parser`). Version is pinned in the Spec42 workspace;
/// hosts should use this module instead of depending on `sysml-v2-parser` directly.
pub mod sysml_v2 {
    pub use ::sysml_v2_parser::*;
}
