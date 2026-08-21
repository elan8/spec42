#![recursion_limit = "256"]

//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod common;
pub mod host;
pub mod language;
mod lsp_runtime;

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
    ast_semantic_ranges, legend, semantic_tokens_full, semantic_tokens_range, SymbolEntry,
};
pub use common::util::{merge_host_and_client_library_paths, parse_library_paths_from_value};
pub use syntax::ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use validation::{
    built_workspace_input_from_snapshot, semantic_report_from_built_workspace, validate_paths,
    validate_paths_with_semantics, BuiltWorkspaceInput, SemanticValidationReport,
    ValidatedDocument, ValidationReport, ValidationRequest, ValidationSummary,
};
pub use views::dto::{
    SysmlClearCacheResultDto, SysmlFeatureInspectorAnalysisDto, SysmlFeatureInspectorElementDto,
    SysmlFeatureInspectorElementRefDto, SysmlFeatureInspectorEvaluationDto,
    SysmlFeatureInspectorInheritedFeatureDto, SysmlFeatureInspectorLanguageHelpDto,
    SysmlFeatureInspectorParamsDto, SysmlFeatureInspectorReferenceDto,
    SysmlFeatureInspectorRelationshipDto, SysmlFeatureInspectorResolutionDto,
    SysmlFeatureInspectorResultDto, SysmlFeatureInspectorSelectionDto, SysmlLibrarySearchItemDto,
    SysmlLibrarySearchPackageDto, SysmlLibrarySearchParamsDto, SysmlLibrarySearchResultDto,
    SysmlLibrarySearchSourceDto, SysmlServerCachesDto, SysmlServerMemoryDto, SysmlServerStatsDto,
    TextDocumentIdentifierDto,
};
pub use views::{empty_feature_inspector_response, parse_sysml_feature_inspector_params};

