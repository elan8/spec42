//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod bench;
pub mod common;
pub mod host;
pub mod language;
mod lsp_runtime;
pub mod semantic_model;
pub mod semantic_tokens;
pub mod syntax;
pub mod validation;
pub mod views;
pub(crate) mod workspace;

// Host contract exports (intended stable composition surface for edition hosts).
pub use host::config::{
    CapabilityAugmenter, CustomMethodProvider, SemanticCheckProvider, Spec42Config,
};
pub use host::default_config::default_config as default_server_config;
pub use lsp_runtime::run as run_lsp;

// Core data model exports.
pub use syntax::ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use validation::{validate_paths, ValidationReport, ValidationRequest, ValidationSummary};
pub use views::dto::{
    DiagramBoundsDto, DiagramNodeCompartmentsDto, DiagramPointDto, DiagramSceneDto,
    GeneralDiagramEdgeDto, GeneralDiagramNodeDto, GeneralDiagramSceneDto, GraphEdgeDto,
    GraphNodeDto, IbdDiagramSceneDto, IbdSceneConnectorDto, IbdScenePartDto, IbdScenePortDto,
    IbdSceneRootDto, SysmlClearCacheResultDto, SysmlDiagramOptionsDto, SysmlDiagramParamsDto,
    SysmlDiagramResultDto, SysmlDiagramStatsDto, SysmlFeatureInspectorElementDto,
    SysmlFeatureInspectorElementRefDto, SysmlFeatureInspectorParamsDto,
    SysmlFeatureInspectorRelationshipDto, SysmlFeatureInspectorResolutionDto,
    SysmlFeatureInspectorResultDto, SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto,
    SysmlServerCachesDto, SysmlServerMemoryDto, SysmlServerStatsDto, WorkspaceFileModelDto,
    WorkspaceModelDto, WorkspaceModelSummaryDto,
};
pub use views::ibd::{build_ibd_for_uri, is_port_like, IbdDataDto};
pub use views::{
    build_sysml_diagram_response, build_sysml_model_response, empty_diagram_response,
    empty_feature_inspector_response, empty_model_response, parse_sysml_diagram_params,
    parse_sysml_feature_inspector_params, parse_sysml_model_params,
};
pub use {
    analysis::*,
    common::util::{merge_host_and_client_library_paths, parse_library_paths_from_value},
};
